pub mod guac_integration;
pub mod dojo_integration;
pub mod chainloop_integration;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}