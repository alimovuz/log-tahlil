use crate::daraja::Daraja;
use crate::yozuv::Yozuv;
use std::collections::HashMap;
pub fn top_xatolar(loglar: &[Yozuv]) -> HashMap<&str, u32> {
    let mut xato_sonlari: HashMap<&str, u32> = HashMap::new();

    for log in loglar {
        if matches!(
            log.daraja,
            Daraja::Error | Daraja::Alert | Daraja::Critical | Daraja::Emergency
        ) {
            let soni = xato_sonlari.entry(log.sarlavha.as_str()).or_insert(0);
            *soni += 1;
        }
    }

    xato_sonlari
}

pub fn darajalar_soni(yozuvlar: &[Yozuv]) -> HashMap<&Daraja, u32> {
    let mut daraja_sonlari = HashMap::new();
    for yozuv in yozuvlar {
        let soni = daraja_sonlari.entry(&yozuv.daraja).or_insert(0);
        *soni += 1;
    }

    daraja_sonlari
}
