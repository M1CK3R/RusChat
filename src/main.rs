use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {

    // Configuracion del servidor TCP y canal de broadcast para mensajes entre clientes en el puerto 8000
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let (destinatario, remitente) = broadcast::channel(10);

    loop {
        let (socket, _addr) = listener.accept().await?;
        let destinatario = destinatario.clone();
        let mut remitente = destinatario.subscribe();

        tokio::spawn(async move {
            handle_client(socket, destinatario, remitente).await;
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    remitente: broadcast::Sender<String>,
    mut destinatario: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = tokio::io::BufReader::new(reader).lines();
    let mut line = String::new();

    // Recibir mensajes del cliente y enviarlos al canal de broadcast
    while let Ok(line) = reader.next_line().await {
        let line = match line {
            Some(l) => l,
            None => continue,
        };
        remitente.send(line).unwrap();
    }

    // Enviar mensajes del canal de broadcast al cliente
    while let Ok(msg) = destinatario.recv().await {
        writer.write_all(msg.as_bytes()).await.unwrap();
    }
}