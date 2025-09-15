use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {

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
    destinatario: broadcast::Sender<String>,
    mut remitente: broadcast::Receiver<String>,
) {
    // Aqui despues ira la logica para recibir y enviar mensajes
}