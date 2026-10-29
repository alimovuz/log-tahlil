use crate::daraja::Daraja;
use crate::yozuv::Yozuv;
use std::collections::HashMap;
pub fn top_xatolar(loglar: &[Yozuv]) -> HashMap<&str, (u32, Vec<String>)> {
    let mut xato_sonlari: HashMap<&str, (u32, Vec<String>)> = HashMap::new();

    for log in loglar {
        if matches!(
            log.daraja,
            Daraja::Error | Daraja::Alert | Daraja::Critical | Daraja::Emergency
        ) {
            let (soni, vec) = xato_sonlari.entry(log.sarlavha.as_str()).or_insert((0, Vec::new()));
            *soni += 1;
            if let Some(joy) = &log.joy {
                vec.push(joy.split("/").last().unwrap_or("").to_string())
            }
        }
    }

    for (_, joylar) in xato_sonlari.values_mut() {
        joylar.sort();
        joylar.dedup();
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
