use std::time;

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Command>();

    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);
    let h = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    let tx_reply = tx_reply.clone();
                    h.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("Received reply: {reply}");
        }
    });

    let mut counter = 0;
    loop {
        tokio::time::sleep(time::Duration::from_millis(1000)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}
