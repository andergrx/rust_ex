use anyhow::Result;
use pub_sub::consumer::Subscriber;
use pub_sub::producer::{Publisher, SubCallback};
use pub_sub::{consumer::Consumer, producer::Producer};
use std::sync::Arc;
use tokio::sync::{broadcast, watch};
use tokio::time::{sleep, Duration};
//use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, _) = watch::channel(0);
    let (btx, _) = broadcast::channel(1);

    let mut producer = Producer::new(tx, btx);

    let mut handles = vec![];
    for i in 1..=5 {
        let mut consumer = Consumer::new(i, producer.get_channel(), producer.get_broadcast());

        handles.push(tokio::spawn(async move {
            consumer.consume().await;
        }));
    }

    producer.register(Box::new(Consumer::call_me));
    //let mut sub = Consumer::new(99, producer.get_channel(), producer.get_broadcast());
    //let cb: Rc<dyn Fn()> = Rc::new(|| sub.callback());
    // let cb: Rc<dyn Fn()> = Rc::new(|| sub.callback());
    // producer.register(Rc::clone(&cb));

    for i in 1..=5 {
        let _ = producer.send(i);
        producer.notify();
        sleep(Duration::from_millis(50)).await;
    }
    //let _ = time::sleep(Duration::from_secs(10));
    let _ = producer.exit();

    for handle in handles {
        handle.await?;
    }

    let mut subs = vec![];
    for i in 1..=5 {
        subs.push(Subscriber::new(i));
        
    }

    let mut publ = Publisher::new();
    for sub in subs {
        publ.register(Box::new(move || sub.callback()));
    }

    let handle = tokio::spawn(async move {
        publ.notify().await;
    });

    handle.await?;

    Ok(())
}
