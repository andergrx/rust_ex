use anyhow::Result;
use workers::tasks::Task;

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = vec![];
    for i in 1..=2 {
        let task = Task::new(i);

        handles.push(tokio::spawn(async move {
            task.execute().await.unwrap();
        }));
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
