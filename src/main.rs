use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use clap::Parser;
use log_tahlil::daraja::{Daraja, matndan};
use log_tahlil::filter::filter_funksiya;
use log_tahlil::hisobot::{darajalar_soni, top_xatolar};
use log_tahlil::yigish::{yig, VAQT_FORMAT};
use log_tahlil::yozuv::Yozuv;
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
    /// Ruxsat etilgan kirish formati quyidagicha: 2026-10-20 08:00:00
    #[arg(long, value_parser = vaqt_format)]
    from: Option<NaiveDateTime>,
    /// Ruxsat etilgan kirish formati quyidagicha (kiritilgan vaqt oraliqqa kirmaydi): 2026-10-20 08:00:00
    #[arg(long, value_parser = vaqt_format)]
    to: Option<NaiveDateTime>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let (Some(boshi), Some(oxiri)) = (&args.from, &args.to) {
        if boshi >= oxiri {
            anyhow::bail!("--from --to dan oldin bo'lishi kerak")
        }
    }

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

    let matn = fs::read_to_string(&args.fayl)
        .with_context(|| format!("Fayl topilmadi {}", args.fayl.display()))?;

    let (yozuvlar, xato_vaqtlar_soni) = yig(&matn);
    let filterlangan_yozuvlar: Vec<Yozuv> =
        filter_funksiya(yozuvlar, &minimal_daraja, (&args.from, &args.to));

    let daraja_sonlari = darajalar_soni(&filterlangan_yozuvlar);

    let mut tartiblangan: Vec<(&Daraja, u32)> = daraja_sonlari.into_iter().collect();

    tartiblangan.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    println!("Darajalar soni:");
    for (daraja, soni) in &tartiblangan {
        println!("  {:?}: {}", daraja, soni);
    }

    let xato_sonlari = top_xatolar(&filterlangan_yozuvlar);
    let mut tartiblangan_xatolar: Vec<(&str, (u32, Vec<String>))> = xato_sonlari.into_iter().collect();
    tartiblangan_xatolar.sort_by(|a, b| b.1.0.cmp(&a.1.0).then_with(|| a.0.cmp(&b.0)));
    println!("\nTop xatolar:");
    for (xato, (soni, joy)) in tartiblangan_xatolar.iter().take(10) {
        let mut joylar = joy.join(", ");
        if joylar.is_empty() {
            joylar = "-".to_string();
        }
        println!("  {} × {} joylar: {}", soni, xato, joylar);
    }

    if tartiblangan_xatolar.len() > 10 {
        println!("  ... va yana {} ta", tartiblangan_xatolar.len() - 10);
    }

    if xato_vaqtlar_soni > 0 {
        eprintln!(
            "Ogohlantirish: vaqti noto'g'ri bo'lgan {xato_vaqtlar_soni} ta yozuv o'tkazib yuborildi."
        )
    }
    Ok(())
}

fn vaqt_format(vaqt: &str) -> std::result::Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(vaqt, VAQT_FORMAT)
        .map_err(|_| format!(
            "Noto'g'ri sana formati: '{}'. Iltimos, quyidagi shaklda kiriting: YYYY-MM-DD HH:MM:SS (masalan: 2026-10-20 08:00:00)",
            vaqt
        ))
}
