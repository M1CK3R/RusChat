use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{broadcast, Mutex as TokioMutex},
};

const MAX_MSG_SIZE: usize = 256;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;

    // Un canal broadcast para que todos los clientes reciban cada mensaje
    let (tx, _rx) = broadcast::channel::<String>(32);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx_clone = tx.clone();
        let rx_clone = tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, tx_clone, rx_clone).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    remitente: broadcast::Sender<String>,
    mut destinatario: broadcast::Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Split the socket
    let (reader, writer) = socket.into_split();

    // 1️⃣ Wrap the reader in a BufReader para poder usar `read_line()`
    let mut buf_reader = BufReader::new(reader);

    // 2️⃣ Envolver el writer en un TokioMutex + Arc
    let writer = std::sync::Arc::new(TokioMutex::new(writer));

    // Tarea de escritura (solo lee del canal broadcast y escribe)
    let write_task = {
        let writer_clone = writer.clone();
        tokio::spawn(async move {
            while let Ok(msg) = destinatario.recv().await {
                // Obtener lock asíncrono
                let mut w = writer_clone.lock().await;
                if let Err(e) = w.write_all(format!("{}\n", msg).as_bytes()).await {
                    eprintln!("Error escribiendo mensaje: {}", e);
                    break; // Si falla, terminamos la tarea de escritura
                }
            }
        })
    };

    // Bucle principal que lee líneas del cliente y las rebroadcasta
    let mut line = String::new();
    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => break, // EOF - cliente desconectado
            Ok(_) => {
                // Remover el salto de línea al final
                let line = line.trim_end().to_string();
                
                if line.len() > MAX_MSG_SIZE {
                    // Enviar mensaje de error al propio cliente
                    let mut w = writer.lock().await;
                    let _ = w.write_all(b"Mensaje demasiado largo.\n").await;
                    continue;
                }

                if !line.is_empty() {
                    remitente.send(line).map_err(|e| eprintln!("Error: {}", e)).ok();
                }
            }
            Err(_) => break, // Error de lectura
        }
    }

    // Cuando el cliente se desconecta, cancelamos la tarea de escritura
    write_task.abort();

    Ok(())
}
