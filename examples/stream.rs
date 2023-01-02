use futures::{stream, StreamExt};
use tokio::time;


async fn double(number: i32) -> i32 {
    time::sleep(time::Duration::from_millis(500)).await;
    number * number
}

async fn increase(number: i32) -> i32 {
    time::sleep(time::Duration::from_millis(500)).await;
    number + 1
}

#[tokio::main]
async fn main() {
    let stream = stream::iter(1..=3);
    let stream = stream.then(|x| async move {
        increase(x).await
    }).then(|x| async move {
        double(x).await
    });

    println!("{:?}", stream.collect::<Vec<_>>().await);
}