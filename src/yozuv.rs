use crate::daraja::{Daraja, matndan};
use regex::Regex;

#[derive(Debug)]
pub struct Yozuv {
    pub vaqt: String,
    pub daraja: Daraja,
    pub sarlavha: String,
    pub matn: String,
    pub chaqiruvlar: Vec<String>,
}
pub fn sarlavhani_ajrat(qator: &str, re: &Regex) -> Option<(String, Daraja, String)> {
    let c = re.captures(qator)?;
    Some((c[1].to_string(), matndan(&c[2]), c[3].to_string()))
}
