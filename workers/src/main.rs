type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn hi_there() {
    println!("hi there");
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Command>();
    let h = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            match job {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    let job0 = hi_there;
    let job1 = || println!("Hello, world!");
    let job2 = || {
        for i in 0..10 {
            println!("{}", i);
        }
    };

    tx.send(Command::Run(Box::new(job0))).unwrap();
    tx.send(Command::Run(Box::new(job1))).unwrap();
    tx.send(Command::Run(Box::new(job2))).unwrap();
    tx.send(Command::Quit).unwrap();
    
    h.join().unwrap();

    println!("Bye!");
}
