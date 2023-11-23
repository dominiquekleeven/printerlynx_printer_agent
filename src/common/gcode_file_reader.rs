use std::collections::HashMap;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::common::app_error::AppError;

pub async fn parse_gcode_from_file(file: &str) -> Result<HashMap<usize, String>, AppError> {
    let mut commands = HashMap::new();
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

pub async fn get_gcode_hashmap_test_file() -> Result<HashMap<usize, String>, AppError> {
    let commands = parse_gcode_from_file("test_files/benchy.gcode").await?;
    Ok(commands)
}
