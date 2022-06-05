use std::env;
use std::{
    thread,
    time::{Duration, SystemTime},
};

use redis::RedisResult;
use tokio::time::interval;

// #[tokio::main]
// async fn main() {
fn main() {
    let timeout = env::args().nth(1).unwrap().to_owned();
    let timeout: u64 = timeout.parse::<u64>().unwrap();

    // Read redis connection URI and password from OS ENV
    let password = get_redis_password();
    let redis_addr = get_redis_addr();

    let mut redis_uri = String::new();

    if password.is_empty() {
        redis_uri.push_str(&format!("redis://{redis_addr}/")[..]);
    } else {
        redis_uri.push_str(&format!("redis://:{password}@{redis_addr}/")[..]);
    }

    let client = redis::Client::open(redis_uri).unwrap();
    let now = SystemTime::now();
    // let mut interval = interval(Duration::from_secs(timeout));
    let interval = Duration::from_secs(timeout);
    loop {
        // interval.tick().await;
        thread::sleep(interval);
        println!();
        println!("{}", now.elapsed().unwrap().as_secs());

        // match client.get_async_connection().await {
        match client.get_connection() {
            Ok(mut conn) => {
                println!("> PING");
                // let result: RedisResult<String> = redis::cmd("PING").query_async(&mut conn).await;
                let result: RedisResult<String> = redis::cmd("PING").query(&mut conn);
                println!("< {:?}", result);
            },
            Err(err) => {
                println!("{:?}", err);
            },
        }
    }
}

fn get_redis_password() -> String {
    match env::var_os("REDIS_PASSWORD") {
        Some(password) => password.into_string().unwrap(),
        None => panic!("Mandatory env variable needed: REDIS_PASSWORD"),
    }
}

fn get_redis_addr() -> String {
    match env::var_os("REDIS_ADDR") {
        Some(redis_addr) => redis_addr.into_string().unwrap(),
        None => panic!("Mandatory env variable needed: REDIS_ADDR"),
    }
}
