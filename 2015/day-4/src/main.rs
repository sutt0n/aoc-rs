use std::{sync::{Mutex, Arc, mpsc}, thread};

struct Message {
    input: String,
    hash: String,
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("{} {}", self.input, self.hash)
    }
}

fn part1(input: &'static str, starts_with: &'static str) {
    let counter = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel::<Message>();

    for i in 1..20 {
        let tx1 = tx.clone();
        let counter = Arc::clone(&counter);
        let _ = thread::Builder::new().name(format!("thread_{}", i)).spawn(move || {
            // prolly get some poison errors, but oh well
            let mut count = counter.lock().unwrap();
            while i * 1000000 > *count {
                let digest = md5::compute(format!("{}{}", input, *count));
                let formatted = format!("{:x}", digest).clone();
                tx1.send(Message { input: format!("{}{}", input, *count), hash: formatted }).unwrap();
                *count += 1;
            }
        });
    }

    drop(tx);

    while let Ok(msg) = rx.recv() {
        if msg.hash.starts_with(starts_with) {
            println!("hash {}", msg.to_string());
            break;
        }
    }

}

fn main() -> Result<(), std::io::Error> {
    let input = "bgvyzdsv";

    part1(&input, "00000");
    part1(&input, "000000");

    Ok(())
}
