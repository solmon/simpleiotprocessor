use futures::{future, future::BoxFuture, FutureExt }; // 0.3.13
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    loop {
        let batch_of_tasks = future::join_all(all_tasks());
        future::join(batch_of_tasks, time::sleep(Duration::from_secs(5))).await;
    }
}

fn all_tasks() -> Vec<BoxFuture<'static, ()>> {
    vec![process_shift().boxed()]
}

async fn process_shift() {
    
    println!("called process shift");
    let input = rtftp::getmessage("one").unwrap();

    let processed = ddshift::transform(input.as_str());

    let struct_str = format!("{:#?}", processed);
    //println!("{}", struct_str);
    let _ = rbmq::send_msg(&struct_str);        
}