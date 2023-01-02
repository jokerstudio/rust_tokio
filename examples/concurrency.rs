use std::sync::Arc;
use tokio::{sync::Mutex};

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    let data1 = Arc::new(Mutex::new(0));
    for _ in 1..11 {
        let data2 = Arc::clone(&data1);
        handles.push(tokio::spawn(async move {
            // do some I/O work or call asynchronous code
            let mut lock = data2.lock().await;
            *lock += 1;
        }));
    }

    {
        let mut lock = data1.lock().await;
        *lock += 1;
    }

    futures::future::join_all(handles).await;
    let lock2 = data1.lock().await;
    println!("{}", lock2);
}
