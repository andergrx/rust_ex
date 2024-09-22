use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::Duration;

type Error = tokio::io::Error;

const TASKS: i16 = 5;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut handles = vec![];
    for i in 1..=TASKS {
        handles.push(tokio::spawn(message(format!("Message {}", i))));
    }

    for i in 1..=TASKS {
        tokio::spawn(process(i)).await.unwrap();
    }

    for i in TASKS + 1..=2 * TASKS {
        handles.push(tokio::spawn(process(i)));
    }

    let (tx, rx) = oneshot::channel();

    let ch_task = tokio::spawn(async move {
        tokio::select! {
            _ = rx => {
                println!("Channel oneshot received...");
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                println!("Oneshot Task completed normally");
            }
        }
        println!("Task is cleaning up");
    });

    let _ = tx.send(());
    ch_task.await?;

    let (btx, _) = broadcast::channel(12);
    //let mut rx2 = btx.subscribe();

    for i in 1..=TASKS {
        let mut task_rx = btx.subscribe();
        handles.push(tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(x) = task_rx.recv() => {
                        println!("Channel Task {} received broadcast...{}", i, x);
                    }
                    else => break,
                    // _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    //     println!("Channel Task {} completed normally", i);
                    // }
                }
                println!("Channel Task {} is cleaning up", i);
            }
        }));
    }

    for i in TASKS + 1..=2 * TASKS {
        handles.push(tokio::spawn(channel_task(i, btx.subscribe())));
    }

    for i in 1..=3 {
        let _ = btx.send(format!("Sending {}", i));
    }
    drop(btx);

    for h in handles {
        h.await?;
    }

    //wait(handles);
    println!("Ending Main.");

    Ok(())
}

async fn message(msg: String) {
    println!("This is the async message: {}", msg)
}

async fn process(n: i16) {
    println!("This is the async data: {}", n);
}

async fn channel_task(n: i16, mut rx: broadcast::Receiver<String>) {
    loop {
        tokio::select! {
            Ok(x) = rx.recv() => {
                println!("fn channel_task {} received broadcast...{}", n, x);
            }
            else => break,
        }
        println!("fn channel_task {} is cleaning up", n);
    }
}

// async fn process(n: i16, rx: Arc<oneshot::Receiver<()>>) {
//     println!("This is the async data: {}", n);
//     tokio::select! {
//         _ = Arc::as_ptr(&rx)=> {
//             println!("Task is cancelling...");
//         }
//         _ = tokio::time::sleep(Duration::from_secs(10)) => {
//             println!("Task completed normally");
//         }
//     }
//     println!("Task is cleaning up");
// }

async fn wait(handles: Vec<JoinHandle<()>>) -> Result<(), Error>
// where
//     F: Future + Send + 'static,
//     F::Output: Send + 'static,
{
    println!("Handles size {}", handles.len());
    for h in handles {
        h.await?;
    }

    Ok(())
}
