use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "under the dev".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    match result {
        Some(ref value) => println!("result = {:?}", value),
        _ => println!("Not found")
    }

    Ok(())
}