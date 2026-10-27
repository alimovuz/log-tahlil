use anyhow::{Context, Result};
use clap::Parser;
use log_tahlil::daraja::{Daraja, matndan};
use log_tahlil::hisobot::{darajalar_soni, top_xatolar};
use log_tahlil::yigish::yig;
use log_tahlil::yozuv::Yozuv;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// Laravel log fayllarini tahlil qiladi
#[derive(Parser, Debug)]
struct Args {
    /// Log faylining yo'li
    fayl: PathBuf,
    /// Ruxsat berilgan daraja qiymatlari: debug, info, notice, warning, error, critical, alert, emergency
    #[arg(short, long)]
    level: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let minimal_daraja = match &args.level {
        Some(level) => {
            let min_daraja = matndan(level);
            if matches!(min_daraja, Daraja::Unknown(_)) {
                anyhow::bail!(
                    "Daraja - {level} qiymat qabul qila olmaydi (mavjud qiymatlar: debug, info, notice, warning, error, critical, alert, emergency)! foydalanish bo'yicha ko'rsatmalarni --help buyrug'i orqali olishingiz mumkin."
                )
            } else {
                Some(min_daraja)
            }
        }
        None => None,
    };

    let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\] \w+\.(\w+): (.*)$")?;
    let re_sarlavha = Regex::new(r#""[^"]*"|\d+"#)?;

    let matn = fs::read_to_string(&args.fayl)
        .with_context(|| format!("Fayl topilmadi {}", args.fayl.display()))?;

    let (yozuvlar, xato_vaqtlar_soni) = yig(&matn, &re, &re_sarlavha);

    let filterlangan_yozuvlar: Vec<Yozuv> = match minimal_daraja {
        Some(daraja) => yozuvlar
            .into_iter()
            .filter(|elem| daraja <= elem.daraja)
            .collect(),
        None => yozuvlar,
    };

    let daraja_sonlari = darajalar_soni(&filterlangan_yozuvlar);

    let mut tartiblangan: Vec<(&Daraja, u32)> = daraja_sonlari.into_iter().collect();

    tartiblangan.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    println!("Darajalar soni:");
    for (daraja, soni) in &tartiblangan {
        println!("  {:?}: {}", daraja, soni);
    }

    let xato_sonlari = top_xatolar(&filterlangan_yozuvlar);
    let mut tartiblangan_xatolar: Vec<(&str, u32)> = xato_sonlari.into_iter().collect();
    tartiblangan_xatolar.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    println!("\nTop xatolar:");
    for (xato, soni) in tartiblangan_xatolar.iter().take(10) {
        println!("  {} × {}", soni, xato);
    }

    if tartiblangan_xatolar.len() > 10 {
        println!("  ... va yana {} ta", tartiblangan_xatolar.len() - 10);
    }

    if xato_vaqtlar_soni > 0 {
        eprintln!("Ogohlantirish: vaqti noto'g'ri bo'lgan {xato_vaqtlar_soni} ta yozuv o'tkazib yuborildi.")
    }
    Ok(())
}
