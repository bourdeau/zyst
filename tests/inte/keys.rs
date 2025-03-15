use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_ping() {
    let mut server = start_server();

    let response = send_command("PING");
    assert!(response.contains("PONG"));

    stop_server(&mut server);
}

#[test]
fn test_set_get() {
    let mut server = start_server();

    let response = send_command("SET name Alice");
    assert_eq!(response, "OK");

    let response = send_command("GET name");
    assert!(response.contains("Alice"));

    stop_server(&mut server);
}

#[test]
fn test_delete_key() {
    let mut server = start_server();

    send_command("SET city Paris");
    let response = send_command("DEL city");
    assert!(response.contains("(integer) 1"));

    let response = send_command("GET city");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}

#[test]
fn test_delete_multiple_keys() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("DEL first_name last_name age");

    assert!(response.contains("(integer) 3"));

    stop_server(&mut server);
}

#[test]
fn test_key_regex() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("KEYS *");
    assert!(response.contains("\"first_name\""));
    assert!(response.contains("\"last_name\""));

    let response = send_command("KEYS first*");
    assert!(response.contains("\"first_name\""));

    let response = send_command("KEYS *name*");
    assert!(response.contains("\"first_name\""));
    assert!(response.contains("\"last_name\""));

    let response = send_command("KEYS f?rst_name");
    assert!(response.contains("\"first_name\""));

    send_command("FLUSHDB");
    std::thread::sleep(std::time::Duration::from_secs(10));

    let response = send_command("KEYS *");
    assert!(
        response.contains("(empty array)"),
        "Test failed! Actual response: {:?}",
        response
    );

    stop_server(&mut server);
}

#[test]
fn test_exists() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("EXISTS first_name");
    assert!(response.contains("(integer) 1"));

    let response = send_command("EXISTS middle_name");
    assert!(response.contains("(integer) 0"));

    let response = send_command("EXISTS first_name last_name middle_name");
    assert!(response.contains("(integer) 2"));

    let response = send_command("EXISTS first_name last_name age");
    assert!(response.contains("(integer) 3"));

    stop_server(&mut server);
}

#[test]
fn test_expire() {
    let mut server = start_server();

    send_command("SET name Smith");
    let response = send_command("EXPIRE name 3");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_ttl() {
    let mut server = start_server();

    send_command("SET name Smith");
    send_command("EXPIRE name 3");

    std::thread::sleep(std::time::Duration::from_secs(1));

    // (integer) 2
    let ttl = send_command("TTL name");

    let ttl_int: i32 = ttl
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Failed to parse TTL value");

    assert!(ttl_int < 3);

    stop_server(&mut server);
}

#[test]
fn test_background_delete() {
    let mut server = start_server();

    send_command("SET name Smith");
    send_command("EXPIRE name 10");

    send_command("LPUSH list:expire Alice Bob Charlie");
    send_command("EXPIRE list:expire 3");

    send_command("HSET hash:expire name Smith first_name John age 21");
    send_command("EXPIRE hash:expire 3");

    // Background delete occurs every 60 secs
    std::thread::sleep(std::time::Duration::from_secs(70));

    let response = send_command("EXISTS name");
    assert!(response.contains("(integer) 0"));

    let response = send_command("EXISTS list:expire");
    assert!(response.contains("(integer) 0"));

    let response = send_command("EXISTS hash:expire");
    assert!(response.contains("(integer) 0"));

    stop_server(&mut server);
}

#[test]
fn test_incr() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("INCR counter");
    assert!(response.contains("(integer) 1"));

    // Increment the key
    let response = send_command("INCR counter");
    assert!(response.contains("(integer) 2"));

    let response = send_command("GET counter");
    assert!(response.contains("2"));

    stop_server(&mut server);
}

#[test]
fn test_decr() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("DECR another_counter");
    assert!(response.contains("(integer) -1"));

    // Decrement the key
    let response = send_command("DECR another_counter");
    assert!(response.contains("(integer) -2"));

    let response = send_command("GET another_counter");
    assert!(response.contains("-2"));

    stop_server(&mut server);
}

#[test]
fn test_incrby() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("INCRBY incrby 5");
    assert!(response.contains("(integer) 5"));

    // Increment the key by 10
    let response = send_command("INCRBY incrby 10");
    assert!(response.contains("(integer) 15"));

    let response = send_command("GET incrby");
    assert!(response.contains("15"));

    // Decrement the key by 100
    let response = send_command("INCRBY incrby -100");
    assert!(response.contains("(integer) -85"));

    stop_server(&mut server);
}
