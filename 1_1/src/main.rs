use std::{thread, time};

use chrono::Utc;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();

    loop {
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
        let output = format!("{timestamp}: {uuid}");
        println!("{}", output);
        thread::sleep(time::Duration::from_secs(5));
    }
}
