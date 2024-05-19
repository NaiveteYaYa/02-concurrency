use anyhow::{Ok, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: u32 = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: u32,
    data: usize,
}

impl Msg {
    fn new(id: u32, data: usize) -> Self {
        Self { id, data }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 1..=NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx).unwrap());
    }
    drop(tx);

    // consumer(rx)?;
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer : {:?}", msg);
        }
    });

    let secret = consumer.join().map_err(|e| anyhow::anyhow!("{:?}", e));
    println!("secret: {:?}", secret);
    Ok(())
}

fn producer(ind: u32, tx: mpsc::Sender<Msg>) -> Result<(), anyhow::Error> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(ind, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 2 == 0 {
            println!("producer {} is done", ind);
            break;
        }
    }
    Ok(())
}
