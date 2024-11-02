use tokio::sync::{broadcast, watch};
use std::sync::Arc;
//use crate::consumer::{Consumer, Subscriber};
use anyhow::Result;

// #[derive(Debug)]
pub struct Producer<'a> {
    sender: watch::Sender<i32>,
    shutdown: broadcast::Sender<()>,
    callback: Vec<Callback<'a>>,
}

type Callback<'a> = Box<dyn Fn() + 'a>; //Box<dyn Fn() + Send + 'a>; // + 'static>;

impl<'a> Producer<'a> {
    pub fn new(
        tx: watch::Sender<i32>,
        broadcast: broadcast::Sender<()>,
    ) -> Producer<'a> {
        Producer {
            sender: tx,
            shutdown: broadcast,
            callback: Vec::new(),
        }
    }

    pub fn get_channel(&self) -> watch::Receiver<i32> {
        self.sender.subscribe()
    }

    pub fn get_broadcast(&self) -> broadcast::Receiver<()> {
        self.shutdown.subscribe()
    }

    pub fn send(&self, id: i32) -> Result<()> {
        self.sender.send(id)?;

        Ok(())
    }

    pub fn exit(&self) -> Result<()> {
        self.shutdown.send(())?;

        Ok(())
    }

    pub fn register(&mut self, callback: Callback<'a>) {
        self.callback.push(callback);
    }

    pub fn notify(&self) {
        self.callback
            .iter()
            .for_each( |func| func());
    }
}

pub type SubCallback<'a> = Box<dyn Fn() + Send + Sync + 'a>;

pub struct Publisher<'a> {
    //subscriber: Rc<&'a Subscriber>,   
    callback: Vec<SubCallback<'a>>,
}

impl<'a> Publisher<'a> {
    //pub fn new(sub: &'a Subscriber) -> Publisher {
    pub fn new() -> Publisher<'a> {
        Publisher {
            //subscriber: Rc::new(sub),
            callback: vec![],
        }
    }

    pub fn register(&mut self, callback: SubCallback<'a>) {
        self.callback.push(callback);
    }

    pub async fn notify(&self) {
        self.callback
            .iter()
            .for_each(|func| func());
    }

}
