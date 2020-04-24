use std::process::Command;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn doit(script_name: String, interval: usize) {
    let sys_time = SystemTime::now();
    println!("Current system time: {:?}", sys_time);

    let mut finished = false;
    let mut i = 0;
    while !finished {
	i += 1;
	println!(">>>>> Try {} time..", i);
	let status = Command::new(script_name.clone())
	    .status()
	    .expect("Failed to execute command");

	//println!("Hello, world! {:?}", status.success());

	finished = status.success();
        println!("<<<<< System time elapsed: {:?}", sys_time.elapsed().unwrap());
	sleep(Duration::new(interval as u64, 0));
    }

    println!("===== Total system time elapsed: {:?}", sys_time.elapsed().unwrap());
    println!("Current system time: {:?}", SystemTime::now());
    println!("Success and exit.")
}

fn main() {
    dotenv::dotenv().ok();
    let is_daemon = dotenv::var("DAEMON").unwrap_or("false".to_string());
    let is_daemon = is_daemon.parse::<bool>().unwrap();
    let interval = dotenv::var("INTERVAL").unwrap_or("2".to_string());
    let interval = interval.parse::<usize>().unwrap();
    let fail_interval = dotenv::var("FAIL_INTERVAL").unwrap_or("2".to_string());
    let fail_interval = fail_interval.parse::<usize>().unwrap();
    let script_name = dotenv::var("SCRIPT_NAME").unwrap_or("./scpr.sh".to_string());

    if is_daemon {
	loop {
		doit(script_name.clone(), fail_interval);
		sleep(Duration::new(interval as u64, 0));
        }
    }
    else {
	doit(script_name, fail_interval);
    }
}

