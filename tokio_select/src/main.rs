use std::time;

async fn receiver(
    mut rx: tokio::sync::mpsc::Receiver<u32>,
    mut broadcast_rx: tokio::sync::broadcast::Receiver<u32>,
) {
    loop {
        tokio::select!{
            Some(count) = rx.recv() => println!("rx: {count}"),
            Ok(n) = broadcast_rx.recv() => println!("broadcast: {n}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<u32>(1);
    let (broadcast_tx, broadcast_rx) = tokio::sync::broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, broadcast_rx));

    for count in 0..10 {
        if count % 2 == 0 {
            tx.send(count).await.unwrap();
        } else {
            broadcast_tx.send(count).unwrap();
        }

        tokio::time::sleep(time::Duration::from_secs(1));
    }
}
