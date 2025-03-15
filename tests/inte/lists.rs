use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_lpush() {
    let mut server = start_server();

    let response = send_command("LPUSH names Alice Bob Charlie");
    assert!(response.contains("(integer) 3"));

    let response = send_command("LPUSH names David");
    assert!(response.contains("(integer) 4"));

    let response = send_command("LPUSH names Eve");
    assert!(response.contains("(integer) 5"));

    let response = send_command("LPUSH names Eve");
    assert!(response.contains("(integer) 6"));

    let response = send_command("GET names");
    assert!(
        response.contains("WRONGTYPE Operation against a key holding the wrong kind of value")
    );

    stop_server(&mut server);
}

#[test]
fn test_expire() {
    let mut server = start_server();

    send_command("LPUSH list:expire Alice Bob Charlie");
    let response = send_command("EXPIRE list:expire 3");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_rpush() {
    let mut server = start_server();

    let response = send_command("RPUSH names Alice Bob Charlie");
    assert!(response.contains("(integer) 3"));

    let response = send_command("RPUSH names David");
    assert!(response.contains("(integer) 4"));

    let response = send_command("RPUSH names Eve");
    assert!(response.contains("(integer) 5"));

    let response = send_command("RPUSH names Eve");
    assert!(response.contains("(integer) 6"));

    let response = send_command("GET names");
    assert!(
        response.contains("WRONGTYPE Operation against a key holding the wrong kind of value")
    );

    stop_server(&mut server);
}

#[test]
fn test_lrange() {
    let mut server = start_server();

    let response = send_command("LPUSH mylist C B A");
    assert!(response.contains("(integer) 3"));

    let response = send_command("LRANGE mylist 0 -1");
    assert_eq!(response, "[\"A\", \"B\", \"C\"]");

    let response = send_command("LRANGE mylist 1 2");
    assert_eq!(response, "[\"B\", \"C\"]");

    let response = send_command("LRANGE mylist -1 -1");
    assert_eq!(response, "[\"C\"]");

    let response = send_command("LRANGE mylist 10 20");
    assert!(response.contains("(empty array)"));

    let response = send_command("LRANGE unknownkey 0 -1");
    assert!(response.contains("(empty array)"));

    send_command("SET notalist 123");
    let response = send_command("LRANGE notalist 0 -1");
    assert!(
        response.contains("WRONGTYPE Operation against a key holding the wrong kind of value")
    );

    stop_server(&mut server);
}

#[test]
fn test_lpop() {
    let mut server = start_server();

    let response = send_command("LPUSH lpoplist A B C D E");
    assert_eq!(response, "(integer) 5");

    let response = send_command("LPOP lpoplist");
    assert!(response.contains("E"));

    let response = send_command("LPOP lpoplist 2");
    assert_eq!(response, "[\"D\", \"C\"]");

    let response = send_command("LPOP lpoplist 10");
    assert_eq!(response, "[\"B\", \"A\"]");

    let response = send_command("LPOP lpoplist");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}

#[test]
fn test_rpop() {
    let mut server = start_server();

    let response = send_command("RPUSH rpoplist A B C");
    assert!(response.contains("(integer) 3"));

    let response = send_command("RPOP rpoplist");
    assert!(response.contains("C"));

    let response = send_command("RPOP rpoplist 2");
    assert_eq!(response, "[\"B\", \"A\"]");

    let response = send_command("RPOP rpoplist");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}
