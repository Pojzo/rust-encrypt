use std::str::FromStr;

#[derive(Debug)]
pub enum OperationType {
    ENCRYPT,
    DECRYPT,
}

impl FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "encrypt" => Ok(OperationType::ENCRYPT),
            "decrypt" => Ok(OperationType::DECRYPT),
            _ => Err(format!("Invalid value for operation type: {}", s)),
        }
    }
}
