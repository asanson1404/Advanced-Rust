pub mod net;

#[derive(Debug)]
pub enum IdentificationResult {
    WelcomeLine(String),
    NoWelcomeLine,
    ConnectionRefused,
}