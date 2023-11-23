/// Follows the g-code specification from: https://reprap.org/wiki/G-code
/// Compatible with Marlin firmware
#[derive(Debug)]
pub enum GcodeCommand {
    AutoHome,
    AutoBedLeveling,
    SystemInfo,
    DisplayMessage(String),
}

impl GcodeCommand {
    pub fn value(&self) -> &[u8] {
        match self {
            GcodeCommand::AutoHome => b"G28\n",
            GcodeCommand::AutoBedLeveling => b"G29\n",
            GcodeCommand::SystemInfo => b"M115\n",
            GcodeCommand::DisplayMessage(message) => message.as_bytes(),
        }
    }
}
