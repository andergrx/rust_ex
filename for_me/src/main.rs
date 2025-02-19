use for_me::personal::Personal;

#[tokio::main]
async fn main() {

    let mut me = Personal::new();
    me.generate_message().await;
    println!("{}", me);
}