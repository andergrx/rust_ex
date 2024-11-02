use anyhow::Result;
use rand::prelude::*;
use tokio::{
    sync::{
        broadcast,
        watch::{self, Receiver, Sender},
    },
    time::{sleep, Duration},
};

const WORKERS: i32 = 5;

#[derive(Debug)]
pub struct Task {
    id: i32,
    tx: Sender<String>,
    end: broadcast::Sender<i32>,
}

impl Task {
    pub fn new(id: i32) -> Task {
        let (tx, _) = watch::channel(String::from(""));
        let (end, _) = broadcast::channel(10);

        Task {
            id: id,
            tx: tx,
            end: end,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        println!("Task {} executing", self.id);

        let adj = (self.id - 1) * WORKERS;
        let mut join = vec![];

        (1..=WORKERS).into_iter().for_each(|id| {
            let mut worker = Worker::new(id + adj, self.tx.subscribe(), self.end.subscribe());
            join.push(tokio::spawn(async move {
                worker.execute().await;
            }));
        });

        let end_tx = self.end.clone();
        let id = self.id;
        join.push(tokio::spawn(async move {
            Ender::new(id, adj, end_tx).execute().await;
        }));
        
        join.push(tokio::spawn(async move { Task::rando(adj).await }));

        self.sender(adj).await?;
        for j in join {
            j.await?;
        }

        Ok(())
    }

    async fn sender(&self, adj: i32) -> Result<()> {
        for i in 1..=7 {
            self.tx.send(format!("Message {}", i + adj))?;
            sleep(Duration::from_secs(2)).await;
        }

        Ok(())
    }

    async fn rando(adj: i32) {
        for i in 1..=5 {
            println!("Rando {} is talking", i + adj);
            sleep(Duration::from_secs(2)).await;
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: i32,
    rx: Receiver<String>,
    end: broadcast::Receiver<i32>,
}

impl Worker {
    fn new(id: i32, rx: Receiver<String>, end: broadcast::Receiver<i32>) -> Worker {
        Worker {
            id: id,
            rx: rx,
            end: end,
        }
    }

    async fn execute(&mut self) {
        println!("Worker {} executing...", self.id);

        loop {
            tokio::select! {
                Ok(_) = self.rx.changed() => {
                    println!("Worker {} rx {}", self.id, *self.rx.borrow_and_update());
                }
                Ok(id) = self.end.recv() => {
                    if id == self.id {
                        println!("Worker {} received exit. Closing.", self.id);
                        return
                    }
                }
                else => {
                    println!("Worker {} unknown exit.", self.id);
                    return
                }
            }
        }
    }
}

struct Ender {
    id: i32,
    adj: i32,
    end: broadcast::Sender<i32>,
}

impl Ender {
    fn new(id: i32, adj: i32, end: broadcast::Sender<i32>) -> Ender {
        Ender { id: id, adj: adj, end: end }
    }

    async fn execute(&self) {
        let mut total_wait = 8;
        let mut wait = 0;
        let mut worker_ids: Vec<i32> = (1..=WORKERS).collect();
        {
            let mut rng = rand::thread_rng();
            worker_ids.shuffle(&mut rng);
        }
        for worker in worker_ids {
            let random: u64 = {
                let mut rng = rand::thread_rng();
                rng.gen_range(2..total_wait)
            };
            wait += random;
            total_wait -= random;
            if total_wait < 3 {
                total_wait = 3;
            }

            sleep(Duration::from_secs(random)).await;
            println!("{}: Ending worker {} after wait of {}", self.id, worker + self.adj, wait);
            self.end.send(worker + self.adj).unwrap();
        }
    }
}
