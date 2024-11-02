use tokio::sync::{broadcast, watch};

pub struct Consumer {
    id: i32,
    rx: watch::Receiver<i32>,
    exit: broadcast::Receiver<()>,
}

impl Consumer {
    pub fn new(id: i32, rx: watch::Receiver<i32>, brdcst: broadcast::Receiver<()>) -> Consumer {
        Consumer {
            id: id,
            rx: rx,
            exit: brdcst,
        }
    }

    pub async fn consume(&mut self) {
        loop {
            tokio::select! {
                Ok(_) = self.rx.changed() => {
                    println!("Consumer {} Received {}", self.id, *self.rx.borrow_and_update())
                }
                Ok(_) = self.exit.recv() => {
                    println!("Consumer {} received exit", self.id);
                    return
                }
            }
        }
    }

    pub fn call_me() {
        println!("Consumer callback");
    }

    pub fn callback(&mut self) {
        println!("Consumer type callback (id: {})...", self.id);
    }
}

pub struct Subscriber {
    id: i32,
}

impl Subscriber {
    pub fn new(id: i32) -> Subscriber {
        Subscriber { id: id }
    }

    pub fn callback(&self) {
        println!("Callback of subscriber {}", self.id);
    }
}
