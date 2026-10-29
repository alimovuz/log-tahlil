# log-tahlil

Laravel log fayllarini tahlil qiladigan CLI dastur (Rust).

Log faylini o'qiydi, har bir darajadagi yozuvlarni sanaydi, eng ko'p takrorlanadigan xatolarni guruhlaydi va ular kodning qaysi joyidan kelganini ko'rsatadi. Yozuvlarni daraja va vaqt oralig'i bo'yicha filtrlash mumkin.

> Rust o'rganish jarayonida yozilgan loyiha. Joriy versiya: **0.4.0**.

## Imkoniyatlar

- **Darajalar soni** — har bir darajadagi yozuvlar soni (`Debug`, `Info`, `Notice`, `Warning`, `Error`, `Critical`, `Alert`, `Emergency`). Notanish daraja (masalan `CUSTOM`) `Unknown` sifatida alohida sanaladi.
- **Top xatolar** — `Error`, `Critical`, `Alert` va `Emergency` darajadagi yozuvlar sarlavha bo'yicha guruhlanadi. Eng ko'p uchraydigan 10 tasi chiqadi, qolganlari soni `... va yana K ta` ko'rinishida.
- **Xato joylari** — har bir guruh uchun xato tashlangan joy (`Fayl.php:qator`). Joy exception ichidagi `at …:qator)` qismidan olinadi.
- **Sarlavhani normallashtirish** — xabardagi raqamlar `N` ga, qo'shtirnoq ichidagi matn `"..."` ga almashtiriladi. Shu tufayli `Undefined array key 7` va `Undefined array key 12` bitta guruhga tushadi.
- **Filtrlar** — `--level`, `--from`, `--to` (pastda batafsil).
- **Noto'g'ri sanali yozuvlar** (masalan `2026-02-30`) tashlab yuboriladi va ularning soni ogohlantirishda ko'rsatiladi.

## Namuna natija

Repodagi `namuna.log` uchun:

```
$ cargo run -- namuna.log
Darajalar soni:
  Error: 9
  Warning: 6
  Info: 4
  Critical: 3
  Notice: 2
  Debug: 2
  Emergency: 1
  Alert: 1
  Unknown("CUSTOM"): 1

Top xatolar:
  6 × Undefined array key N joylar: CartController.php:12, OrderController.php:58
  2 × Payment gateway timeout after N seconds joylar: CurlFactory.php:210
  2 × SQLSTATE[HYN] [N] Connection refused joylar: Connector.php:65
  1 × Database server unreachable joylar: -
  1 × Disk usage at N% joylar: -
  1 × Queue worker stopped joylar: -
  1 × To'lov o'tmadi joylar: -
Ogohlantirish: vaqti noto'g'ri bo'lgan 4 ta yozuv o'tkazib yuborildi.
```

### Natija qanday tartiblanadi

| Bo'lim | Tartib |
|---|---|
| Darajalar soni | soni kamayish bo'yicha; soni teng bo'lsa — og'irroq daraja yuqorida; `Unknown` eng pastda |
| Top xatolar | soni kamayish bo'yicha; soni teng bo'lsa — sarlavha alifbo bo'yicha |
| Joylar | alifbo bo'yicha, takrorsiz; guruhda birorta ham joy bo'lmasa — `-` |

Ogohlantirish **stderr** ga yoziladi, shuning uchun natijani faylga yo'naltirganda (`> natija.txt`) u faylga tushmaydi.

## O'rnatish

Talab: Rust **1.85** yoki yangiroq (loyiha edition 2024 da yozilgan).

```
git clone https://github.com/alimovuz/log-tahlil.git
cd log-tahlil
cargo build --release
```

## Ishlatish

```
cargo run --release -- <FAYL> [--level <DARAJA>] [--from <VAQT>] [--to <VAQT>]
```

### Faylni to'liq tahlil qilish

```
cargo run --release -- laravel.log
```

### `--level` (`-l`) — daraja bo'yicha filtr

Ko'rsatilgan daraja **va undan og'irroq** darajalar qoladi. `--level error` → `Error`, `Critical`, `Alert`, `Emergency`.

Qabul qilinadigan qiymatlar: `debug`, `info`, `notice`, `warning`, `error`, `critical`, `alert`, `emergency` (katta-kichik harf farqi yo'q).

```
cargo run --release -- laravel.log --level error
```

### `--from` / `--to` — vaqt oralig'i

Format: `YYYY-MM-DD HH:MM:SS`. Oraliq **yarim ochiq**: `--from` dagi vaqt oraliqqa **kiradi**, `--to` dagi vaqt **kirmaydi**. Ikkalasini birga yoki alohida berish mumkin. `--from` `--to` dan oldin bo'lishi shart.

```
cargo run --release -- laravel.log --from "2026-10-26 08:00:00" --to "2026-10-26 08:20:00"
```

Filtrlar birga ishlaydi va ikkala bo'limga ham (darajalar soni va top xatolar) ta'sir qiladi.

### Yordam

```
cargo run --release -- --help
```

## Ma'lum cheklovlar

- Sarlavha xabarning birinchi ` {` gacha bo'lgan qismidan olinadi. Agar xabar matnining o'zida ` {` bo'lsa, sarlavha erta kesiladi.
- `Uncaught X: …` ko'rinishidagi xabarlar bir xil xato uchun alohida (dublikat) sarlavha hosil qilishi mumkin.
- `--from` / `--to` faqat to'liq vaqtni qabul qiladi — faqat sana (`2026-10-26`) qabul qilinmaydi.
- `--level` berilganda `Unknown` darajadagi yozuvlar natijaga chiqmaydi.
- Exception'siz qo'lda yozilgan xatolarda (`Log::error(...)`) joy bo'lmaydi — `joylar: -`.
- Fayl xotiraga to'liq o'qiladi; juda katta log fayllar uchun mo'ljallanmagan.

## Loyiha tuzilmasi

```
src/
├── main.rs      # CLI argumentlari, filtrlar va hisobotni chop etish
├── lib.rs       # modullarni e'lon qilish
├── daraja.rs    # Daraja enum va matndan o'girish
├── yozuv.rs     # Yozuv struct va sarlavha qatorini ajratish
├── yigish.rs    # ko'p qatorli yozuvlarni yig'ish, sarlavhani normallashtirish, joyni ajratish
├── hisobot.rs   # darajalar soni va top xatolarni guruhlash
└── filter.rs    # daraja va vaqt bo'yicha filtrlash
```

## Bog'liqliklar

- [`anyhow`](https://docs.rs/anyhow) — xatolarni boshqarish
- [`chrono`](https://docs.rs/chrono) — sana va vaqtni parse qilish
- [`clap`](https://docs.rs/clap) — CLI argumentlari (derive)
- [`regex`](https://docs.rs/regex) — log qatorlarini ajratish va normallashtirish

## Versiyalar

- **v0.1** — log yozuvlarini yig'ish, darajalar soni, top xatolar
- **v0.2** — `--level` filtri
- **v0.3** — `--from` / `--to` filtrlari, noto'g'ri sanali yozuvlarni tashlash
- **v0.4** — top xatolarda xato joylari