use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    println!("Setting up listener");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let(tx,_rs) = broadcast::channel(10);
    
    loop{
        println!("Setting up socket");
        let (mut socket, addr) = listener.accept().await.unwrap();
        
        println!("client joined!");
        let sender=tx.clone();
        let mut receiver = sender.subscribe();
        
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
        
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            let _welcome_message = "Welcome in tcp-chat.
            You can send messages by pressing enter.
            --------------------------------------------\r\n";
            let welcom_message = String::from(_welcome_message);
            writer.write_all(welcom_message.as_bytes()).await.unwrap();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) =>{
                        if result.unwrap() == 0 {
                            println!("client left!");
                            break;
                        }
                        sender.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = receiver.recv() =>{
                        let (msg, _other_addr) = result.unwrap();
                        if addr != _other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

// tcp echo server
// #[tokio::main]
// async fn main() {
//     println!("Setting up listener");
//     let listener = TcpListener::bind("localhost:8080").await.unwrap();
//     println!("Setting up socket");
//     let (mut socket, _addr) = listener.accept().await.unwrap();
//     loop {
//         let mut buffer=[0u8;1024];
        
//         println!("Reading bytes");
//         let bytes_read = socket.read(&mut buffer).await.unwrap();

//         println!("Writign bytes");
//         socket.write_all(&mut buffer[..bytes_read]).await.unwrap();
//     }
// }
