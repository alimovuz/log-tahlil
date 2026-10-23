use crate::yozuv::{Yozuv, sarlavhani_ajrat};
use regex::{Captures, Regex};

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
pub fn yig(matn: &str, re: &Regex, sarlavha_re: &Regex) -> Vec<Yozuv> {
    let mut yozuvlar: Vec<Yozuv> = Vec::new();
    let mut joriy: Option<Yozuv> = None;

    for qator in matn.lines() {
        if let Some((vaqt, daraja, xabar)) = sarlavhani_ajrat(qator, re) {
            if let Some(yozuv) = joriy {
                yozuvlar.push(yozuv);
            }

            joriy = Some(Yozuv {
                vaqt,
                daraja,
                sarlavha: sarlavhani_normallashtir(&xabar, sarlavha_re),
                matn: xabar,
                chaqiruvlar: vec![],
            });
        } else if let Some(y) = &mut joriy {
            y.chaqiruvlar.push(qator.to_string());
        }
    }

    if let Some(yozuv) = joriy {
        yozuvlar.push(yozuv)
    }

    yozuvlar
}
