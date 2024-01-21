/// Follows the g-code specification from: https://reprap.org/wiki/G-code
/// Compatible with Marlin firmware
#[derive(Debug)]
pub enum GcodeCommand {
    AutoHome,
    AutoBedLeveling,
    SystemInfo,
}

impl GcodeCommand {
    pub fn value(&self) -> &str {
        match self {
            GcodeCommand::AutoHome => "G28",
            GcodeCommand::AutoBedLeveling => "G29",
            GcodeCommand::SystemInfo => "M115",
        }
    }
}
