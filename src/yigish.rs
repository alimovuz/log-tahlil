use crate::yozuv::{Yozuv, sarlavhani_ajrat};
use chrono::NaiveDateTime;
use regex::{Captures, Regex};

pub const VAQT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
fn sarlavhani_normallashtir(xabar: &str, re: &Regex) -> String {
    let xabar_qismlar = xabar.split(" {").collect::<Vec<&str>>();
    let sarlavha = re.replace_all(xabar_qismlar[0], |c: &Captures| {
        let topilgan = &c[0];
        if topilgan.starts_with(|ch: char| ch.is_ascii_digit()) {
            "N".to_string()
        } else {
            "\"...\"".to_string()
        }
    });
    sarlavha.into_owned()
}

pub fn yig(matn: &str) -> (Vec<Yozuv>, u32) {
    let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\] \w+\.(\w+): (.*)$").expect("qator regex'i noto'g'ri!");
    let re_sarlavha = Regex::new(r#""[^"]*"|\d+"#).expect("sarlavha regex'i noto'g'ri!");
    let re_joy = Regex::new(r" at (\S+:\d+)\)").expect("joy regex'i noto'g'ri!");
    let mut yozuvlar: Vec<Yozuv> = Vec::new();
    let mut joriy: Option<Yozuv> = None;
    let mut xato_vaqtlar_soni: u32 = 0;

    for qator in matn.lines() {
        if let Some((vaqt, daraja, xabar)) = sarlavhani_ajrat(qator, &re) {
            if let Some(yozuv) = joriy {
                yozuvlar.push(yozuv);
            }
            let parse_vaqt = NaiveDateTime::parse_from_str(vaqt.as_str(), VAQT_FORMAT);
            let joy = match re_joy.captures(xabar.as_str()) {
                Some(cap) => Some(cap[1].to_string()),
                None => None,
            };

            match parse_vaqt {
                Ok(vaqt) => {
                    joriy = Some(Yozuv {
                        vaqt,
                        joy,
                        daraja,
                        sarlavha: sarlavhani_normallashtir(&xabar, &re_sarlavha),
                        matn: xabar,
                        chaqiruvlar: vec![],
                    });
                }
                Err(_) => {
                    xato_vaqtlar_soni += 1;
                    joriy = None;
                }
            }
        } else if let Some(y) = &mut joriy {
            y.chaqiruvlar.push(qator.to_string());
        }
    }

    if let Some(yozuv) = joriy {
        yozuvlar.push(yozuv)
    }

    (yozuvlar, xato_vaqtlar_soni)
}
