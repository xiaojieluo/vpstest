use std::time::{Duration, SystemTime};
use std::thread::sleep;

extern crate chrono;

use chrono::prelude::*;

fn main(){
    println!("============= 开始记录测试信息 =============");
    let utc: DateTime<Local> = Local::now();
    println!("测试开始时间: {}", utc.format("%Y-%m-%d %H:%M:%S").to_string());

    let now = SystemTime::now();

    system_info();

    let end_time: DateTime<Local> = Local::now();
    println!("测试结束时间: {}", end_time.format("%Y-%m-%d %H:%M:%S").to_string());

    match now.elapsed() {
        Ok(elapsed) => {
            println!("测试结束, 总耗时: {}", elapsed.as_secs())
        },
        Err(e) => {
            println!("{:?}", e)
        }
    };

}

fn system_info(){
    //
    // 系统信息

    // 延时 2 秒
    sleep(Duration::new(2, 0));
}
