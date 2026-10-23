use anyhow::Result;
use log_tahlil::daraja::Daraja;
use log_tahlil::hisobot::{darajalar_soni, top_xatolar};
use log_tahlil::yigish::yig;
use regex::Regex;
use std::fs;

fn main() -> Result<()> {
    let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\] \w+\.(\w+): (.*)$")?;
    let re_sarlavha = Regex::new(r#""[^"]*"|\d+"#)?;

    let matn = fs::read_to_string("laravel.log")?;

    let yozuvlar = yig(&matn, &re, &re_sarlavha);

    let daraja_sonlari = darajalar_soni(&yozuvlar);

    let mut tartiblangan: Vec<(&Daraja, u32)> = daraja_sonlari.into_iter().collect();

    tartiblangan.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Darajalar soni:");
    for (daraja, soni) in &tartiblangan {
        println!("  {:?}: {}", daraja, soni);
    }

    let xato_sonlari = top_xatolar(&yozuvlar);
    let mut tartiblangan_xatolar: Vec<(&str, u32)> = xato_sonlari.into_iter().collect();
    tartiblangan_xatolar.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("\nTop xatolar:");
    for (xato, soni) in tartiblangan_xatolar.iter().take(10) {
        println!("  {} × {}", soni, xato);
    }

    if tartiblangan_xatolar.len() > 10 {
        println!("  ... va yana {} ta", tartiblangan_xatolar.len() - 10);
    }

    Ok(())
}
