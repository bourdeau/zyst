use redis::{Client, Cmd, FromRedisValue, RedisResult, Value};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

pub fn start_server() -> Child {
    let child = Command::new("cargo")
        .args(["run"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to start Redis-like server");

    // Actively check if the server is ready before continuing
    for _ in 0..20 {
        if TcpStream::connect("127.0.0.1:6379").is_ok() {
            send_command("FLUSHDB");
            return child;
        }
        sleep(Duration::from_secs(1)); // Wait before retrying
    }

    panic!("Server did not start in time");
}

pub fn stop_server(server: &mut Child) {
    if let Err(e) = server.kill() {
        eprintln!("Warning: Failed to kill server: {:?}", e);
    }
    let _ = server.wait(); // Ensure process cleanup
    sleep(Duration::from_secs(1)); // Give OS time to release the port
}

pub fn send_command(command: &str) -> String {
    let client = Client::open("redis://127.0.0.1:6379/").expect("Failed to connect to Redis");
    let mut conn = client
        .get_connection()
        .expect("Failed to get Redis connection");

    let args: Vec<&str> = command.split_whitespace().collect();
    if args.is_empty() {
        return "-ERR Empty command\r\n".to_string();
    }

    // Execute the command once and capture the raw response
    let raw_response: RedisResult<Value> = Cmd::new().arg(args).query(&mut conn);

    match raw_response {
        Ok(Value::Okay) => "OK".to_string(), // Handle OK response
        Ok(Value::Int(int_value)) => format!("(integer) {}", int_value), // Handle integers (e.g., LPUSH, LLEN)
        Ok(Value::BulkString(bytes)) => String::from_utf8_lossy(&bytes).to_string(), // Handle bulk string responses
        Ok(Value::Array(items)) => {
            let strings: Vec<String> = items
                .into_iter()
                .filter_map(|v| FromRedisValue::from_redis_value(&v).ok())
                .collect();
            format!("{:?}", strings) // Handle multi-value responses (e.g., LPOP with count)
        }
        Ok(Value::SimpleString(s)) => s, // Handle simple string responses
        Ok(Value::Nil) => "(nil)".to_string(), // Handle nil responses
        Err(e) => format!("{}", e),      // Handle errors
        _ => "-ERR Unexpected response\r\n".to_string(),
    }
}
