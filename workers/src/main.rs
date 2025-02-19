use anyhow::Result;
use workers::tasks::Task;

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = vec![];
    for i in 1..=2 {
        let task = Task::new(i);

        handles.push(tokio::spawn(async move {
            let task = task.execute().await;
            if let Err(err) = task {
                println!("Task Error: {err}");
            }
        }));
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
