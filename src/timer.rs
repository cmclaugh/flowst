use tokio::sync::mpsc;
use chrono::Duration;
use crate::config::TimerInfo;
fn format_seconds(seconds: i64) -> Vec<i64>{
       return vec![(seconds/60),(seconds%60)];   
}

pub fn print_time(seconds: i64) -> String {
    let (minutes, seconds) = (format_seconds(seconds)[0], format_seconds(seconds)[1]);
    format!("{} minutes and {} seconds remaining ", minutes, seconds)
}

async fn countdown(seconds: Duration,  sender: tokio::sync::mpsc::Sender<String>) -> Result<(), std::io::Error>{

    for i in (0..=(seconds.num_seconds())).rev() {
        let countdown_string = print_time(i);
        sender.send(countdown_string).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}

pub async fn paused(timer_info: TimerInfo, sender: tokio::sync::mpsc::Sender<String>)-> Result<(),std::io::Error> {
        let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

        let pause_elapsed = if start_work_elapsed.num_seconds() <= timer_info.work_duration.num_seconds() {
            timer_info.work_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_work.unwrap())
        } else {
            timer_info.rest_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_rest.unwrap())
        };
        
        while !timer_info.run_state {
            let message = print_time(pause_elapsed.num_seconds());
            sender.send(message).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    Ok(())

}


pub async fn start_timer(work: Duration, rest: Duration, timer_info: TimerInfo) -> tokio::sync::mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel(1);
    let state = timer_info.run_state;

    if state {
        tokio::spawn(async move {
            let _ = countdown(work, sender.clone()).await;
            let _ = countdown(rest, sender).await;
        });
    }
    else {
        tokio::spawn(async move {
            let _ = paused(timer_info,sender.clone()).await;
        });
    }

    receiver
}


    

