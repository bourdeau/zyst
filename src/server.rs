use crate::errors::format_redis_error;
use crate::process::process_command;
use crate::resp::parse_resp_command;
use crate::types::Db;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut socket: TcpStream, db: Db) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = socket.read(&mut buffer).await?;

        if bytes_read == 0 {
            // Client disconnected
            return Ok(());
        }

        let raw_command = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        let parsed_commands = match parse_resp_command(&raw_command) {
            Ok(parsed) => parsed,
            Err(e) => {
                let error_response = format_redis_error(e);
                socket.write_all(error_response.as_bytes()).await?;
                socket.flush().await?;
                continue;
            }
        };

        for parsed in parsed_commands {
            match process_command(parsed, &db, false).await {
                Ok(resp) => {
                    let response = resp.to_string();
                    socket.write_all(response.as_bytes()).await?;
                    socket.flush().await?;
                }
                Err(e) => {
                    let error_response = format_redis_error(e);
                    socket.write_all(error_response.as_bytes()).await?;
                    socket.flush().await?;
                }
            }
        }
    }
}
