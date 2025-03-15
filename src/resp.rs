use crate::errors::ZystError;

pub fn parse_resp_command(resp_command: &str) -> Result<Vec<Vec<String>>, ZystError> {
    let lines = resp_command.split_terminator("\r\n");
    let mut commands: Vec<Vec<String>> = Vec::new();
    let mut cmd_nb: Option<usize> = None;

    for line in lines {
        // Check if the line starts with '*' and is followed immediately by a number
        if let Some(rest) = line.strip_prefix('*') {
            if !rest.is_empty() && rest.chars().all(|c| c.is_numeric()) {
                commands.push(Vec::new());
                cmd_nb = Some(commands.len() - 1);
                continue;
            }
        }

        if line.starts_with('$') {
            continue;
        }

        if let Some(idx) = cmd_nb {
            commands[idx].push(line.to_string());
        }
    }

    Ok(commands)
}
