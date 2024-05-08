use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PROCESSORS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // Create Producers
    for i in 0..NUM_PROCESSORS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    drop(tx);

    // Create Consumers
    let consumers = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    let secret = consumers
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("consumer exit secret {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: usize = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let time_sleep = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(time_sleep));

        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }

    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Msg {
        Self { idx, value }
    }
}
