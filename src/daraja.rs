#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Daraja {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
    Unknown(String),
}

pub fn matndan(s: &str) -> Daraja {
    match s.to_uppercase().as_str() {
        "DEBUG" => Daraja::Debug,
        "INFO" => Daraja::Info,
        "NOTICE" => Daraja::Notice,
        "WARNING" => Daraja::Warning,
        "ERROR" => Daraja::Error,
        "CRITICAL" => Daraja::Critical,
        "ALERT" => Daraja::Alert,
        "EMERGENCY" => Daraja::Emergency,
        _ => Daraja::Unknown(s.to_string()),
    }
}
