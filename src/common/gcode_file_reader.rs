use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::common::app_error::AppError;

/// Parses a gcode file into a hashmap of line numbers and gcode commands
pub async fn parse_gcode_from_file(file: &str) -> Result<Vec<String>, AppError> {
    let mut commands: Vec<String> = Vec::new();
    let mut file = File::open(file).await.expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");
    let lines = contents.lines();

    let mut current_line = 0;

    for line in lines {
        if line.starts_with(';') || line.is_empty() {
            continue;
        }
        let command = line.to_string();
        let command = command.trim();
        let command = command.split(';').next().unwrap();
        let command = format!("{}\n", command);

        commands.insert(current_line, command);
        current_line += 1;
    }
    Ok(commands)
}

pub async fn get_gcode_from_file(file: &str) -> Result<Vec<String>, AppError> {
    let commands = parse_gcode_from_file(file).await?;
    Ok(commands)
}
