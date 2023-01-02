use tokio::{task};
use std::thread;
use std::process;

#[tokio::main]
async fn main() {
    let _t = vec!(1,2,3);
    let mut handles = vec![];
    for _ in 1..5 {
        handles.push(task::spawn_blocking(move || {
            // do some compute-heavy work or call synchronous code
            let mut _sum: u32= 0;
            for _ in 1..100_000_000 {
                _sum += 1;
            }
            (thread::current().id(), process::id())
        }));
    }

    let results = futures::future::join_all(handles).await;
    for result in results {
        println!("{:?}", result);
    }
}
