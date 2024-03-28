enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Command>();

    let h = std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::SayHello => {
                    println!("Hello")
                }

                Command::Quit => {
                    println!("Bye");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap()
    }

    println!("Sending quit");
    tx.send(Command::Quit).unwrap();

    h.join().unwrap();
}
