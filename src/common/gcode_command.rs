// Follows the g-code specification from: https://reprap.org/wiki/G-code
// Compatible with Marlin firmware
#[allow(dead_code)]
#[derive(Debug)]
pub enum GcodeCommand {
    AutoHome,
    AutoBedLeveling,
    SystemInfo,
    DisplayMessage(String),
}

impl GcodeCommand {
    #[allow(dead_code)]
    fn value(&self) -> String {
        match self {
            GcodeCommand::AutoHome => String::from("G28"),
            GcodeCommand::AutoBedLeveling => String::from("G29"),
            GcodeCommand::SystemInfo => String::from("M115"),
            GcodeCommand::DisplayMessage(message) => format!("M117 {}", message),
        }
    }
}
