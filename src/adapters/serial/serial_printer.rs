use tokio_serial::SerialPortInfo;

pub struct Printer {
    pub name: String,
    pub serial_number: String,
    pub manufacturer: String,
    pub model: String,
    pub serial_port: SerialPortInfo,
    pub baud_rate: u32,
}
