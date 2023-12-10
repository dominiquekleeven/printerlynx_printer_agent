use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::common::app_error::AppError;

/// Parses a gcode file into a hashmap of line numbers and gcode commands
pub async fn from_file(file: &str) -> Result<Vec<String>, AppError> {
    let mut file = File::open(file).await.expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");

    let commands = parse_lines(&contents.lines().collect());
    Ok(commands)
}

/// Parses a gcode file from a data vec
pub fn from_data(data: &[u8]) -> Result<Vec<String>, AppError> {
    let contents = String::from_utf8(data.to_vec()).unwrap();
    let lines = contents.lines();

    let commands = parse_lines(&lines.collect());
    Ok(commands)
}

fn parse_lines(lines: &Vec<&str>) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for line in lines {
        if line.starts_with(';') || line.is_empty() {
            continue;
        }
        let mut command = line.to_string();
        command = command.trim().to_string();
        command = command.split(';').next().unwrap().to_string();
        command = format!("{}\n", command);
        commands.push(command);
    }
    commands
}
