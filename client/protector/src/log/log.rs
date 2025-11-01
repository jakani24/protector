use chrono::Local;
use hostname::get;

pub fn log(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z");

    let hostname = get()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    let log_line = format!("{} {} {}", timestamp, hostname, msg);

    println!("{}", log_line);
}
