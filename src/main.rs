use std::sync::atomic::{AtomicUsize, Ordering};
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

// Constantes para un global de mensajes
static CONNECTIONS: AtomicUsize = AtomicUsize::new(0);
const MAX_MSG_SIZE: usize = 1024;
const MAX_CONNECTIONS: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {

    // Configuracion del servidor TCP y canal de broadcast para mensajes entre clientes en el puerto 8000
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    // Canal de broadcast con un buffer de 25 mensajes
    // Cada cliente tendra su propio receptor
    let (remitente, _) = broadcast::channel(25);

    print!("Servidor escuchando en 127.0.0.1:8000\n");
    loop {
        let (socket, _addr) = listener.accept().await?;

        // Limitar el numero de conexiones
        if CONNECTIONS.load(Ordering::SeqCst) >= MAX_CONNECTIONS {
            // Rechazar la conexion si se supera el maximo
            let mut socket = socket;
            socket.write_all(b"Servidor lleno, intente mas tarde.\n").await?;
            continue;
        }

        // Incrementar el contador de conexiones si se acepta la conexion
        CONNECTIONS.fetch_add(1, Ordering::SeqCst);
        print!("Nueva conexion: {} \n. Total conexiones: {}\n", _addr, CONNECTIONS.load(Ordering::SeqCst));

        // Clonar el remitente para cada cliente y crear un receptor
        let remitente = remitente.clone();
        let mut destinatario = remitente.subscribe();

        // Manejar la conexion en una tarea separada
        // Cada cliente tiene su propio socket, remitente y destinatario
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, remitente, destinatario).await {
                eprintln!("Error en la conexion de cliente {}: {}", _addr, e);
            }
            CONNECTIONS.fetch_sub(1, Ordering::SeqCst);
            print!("Conexion cerrada: {}. Total conexiones: {} \n", _addr, CONNECTIONS.load(Ordering::SeqCst));
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    remitente: broadcast::Sender<String>,
    mut destinatario: broadcast::Receiver<String>,
) -> Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = tokio::io::BufReader::new(reader).lines();

    // Crear una tarea para recibir mensajes del canal de broadcast
    let mut write_task = tokio::spawn(async move {
        // Escuchar mensajes del canal y enviarlos al cliente
        while let Ok(msg) = destinatario.recv().await {
            // Enviar el mensaje al cliente
            if let Err(e) = writer.write_all(format!("{}\n", msg).as_bytes()).await {
                eprintln!("Error escribiendo mensaje: {}", e);
                break;
            }
        }
    });



    // Crear una tarea para enviar mensajes al cliente
    while let Ok(Some(line)) = reader.next_line().await {
        // Verificar si el mensaje es demasiado largo
        if line.len() > MAX_MSG_SIZE {
            writer.write_all(b"Mensaje demasiado largo.\n").await?;
            continue;
        }

        // Enviar el mensaje a todos los demas clientes
        if !line.is_empty() {
            remitente.send(line).map_err(|e| eprintln!("Error: {}", e)).ok();
        }
    }

    // Si el cliente se desconecta, cancelar la tarea de escritura
    write_task.abort();   
    Ok(())
}