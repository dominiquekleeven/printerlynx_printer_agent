use crate::driver::adapter::Adapter;

pub struct PrinterAgent {
    pub adapter: Box<dyn Adapter>,
}
