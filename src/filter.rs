use crate::daraja::Daraja;
use crate::yozuv::Yozuv;
use chrono::NaiveDateTime;

fn log_oraliqdami(
    log_vaqti: NaiveDateTime,
    from: Option<NaiveDateTime>,
    to: Option<NaiveDateTime>,
) -> bool {
    if let Some(boshi) = from {
        if log_vaqti < boshi {
            return false;
        }
    }

    if let Some(oxiri) = to {
        if log_vaqti >= oxiri {
            return false;
        }
    }

    true
}

fn daraja_mosmi(log_darajasi: &Daraja, minimal: &Option<Daraja>) -> bool {
    match minimal {
        Some(min) => min <= log_darajasi,
        None => true,
    }
}

pub fn filter_funksiya(
    yozuvlar: Vec<Yozuv>,
    daraja: &Option<Daraja>,
    oraliq: (&Option<NaiveDateTime>, &Option<NaiveDateTime>),
) -> Vec<Yozuv> {
    yozuvlar
        .into_iter()
        .filter(|elem| {
            daraja_mosmi(&elem.daraja, daraja) && log_oraliqdami(elem.vaqt, *oraliq.0, *oraliq.1)
        })
        .collect()
}
