use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_hset() {
    let mut server = start_server();

    let response = send_command("HSET myhash name Smith first_name John age 21");
    assert!(response.contains("(integer) 3"));

    let response = send_command("HSET myhash age 34 city Paris");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_expire() {
    let mut server = start_server();

    send_command("HSET myhash:expire name Smith first_name John age 21");
    let response = send_command("EXPIRE myhash:expire 3");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_hget() {
    let mut server = start_server();

    let response = send_command("HSET myhash name Smith first_name John age 21");
    assert!(response.contains("(integer) 3"));

    let response = send_command("HGET myhash name");
    assert!(response.contains("Smith"));

    let response = send_command("HGET myhash first_name");
    assert!(response.contains("John"));

    let response = send_command("HGET myhash age");
    assert!(response.contains("21"));

    let response = send_command("HGET myhash city");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}

#[test]
fn test_hgetall() {
    let mut server = start_server();

    let response = send_command("HSET myhashhgetall name Smith first_name John age 21");
    assert!(response.contains("(integer) 3"));

    let response = send_command("HGETALL myhashhgetall");
    assert!(response.contains("name"));
    assert!(response.contains("Smith"));
    assert!(response.contains("first_name"));
    assert!(response.contains("John"));
    assert!(response.contains("age"));
    assert!(response.contains("21"));

    let response = send_command("HGETALL non_existing_hash");
    assert!(response.contains("(empty array)"));

    stop_server(&mut server);
}

#[test]
fn test_hdel() {
    let mut server = start_server();

    let response =
        send_command("HSET myhashdel last_name Smith first_name John age 21 city Paris");
    assert!(response.contains("(integer) 4"));

    let response = send_command("HGETALL myhashdel");
    assert!(response.contains("last_name"));
    assert!(response.contains("Smith"));
    assert!(response.contains("first_name"));
    assert!(response.contains("John"));
    assert!(response.contains("age"));
    assert!(response.contains("21"));
    assert!(response.contains("city"));
    assert!(response.contains("Paris"));

    let response = send_command("HDEL myhashdel age");
    assert!(response.contains("(integer) 1"));

    let response = send_command("HGETALL myhashdel");
    assert!(!response.contains("age"));
    assert!(!response.contains("21"));

    let response = send_command("HDEL myhashdel last_name city");
    assert!(response.contains("(integer) 2"));

    let response = send_command("HGETALL myhashdel");
    assert!(!response.contains("last_name"));
    assert!(!response.contains("Smith"));
    assert!(!response.contains("city"));
    assert!(!response.contains("Paris"));

    let response = send_command("HDEL myhashdel first_name");
    assert!(response.contains("(integer) 1"));

    let response = send_command("HGETALL myhashdel");
    assert!(response.contains("(empty array)"));

    let response = send_command("HDEL non_existing_hash some_field");
    assert!(response.contains("(integer) 0"));

    stop_server(&mut server);
}
