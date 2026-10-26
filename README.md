# log_tahlil

Laravel log fayllarini tahlil qiluvchi CLI dastur (Rust).
Log faylni o'qib, har bir daraja necha marta uchraganini va eng ko'p takrorlangan xatolarni chiqaradi.

> Rust o'rganish jarayonida yozilgan o'quv loyihasi. Joriy versiya: **v0.1**.

## Namuna natija

`namuna.log` ustida:

```
Darajalar soni:
  Error: 5
  Warning: 5
  Critical: 3
  Info: 3
  Notice: 2
  Debug: 2
  Emergency: 1
  Alert: 1
  Unknown("CUSTOM"): 1

Top xatolar:
  3 × Undefined array key N
  2 × Payment gateway timeout after N seconds
  2 × SQLSTATE[HYN] [N] Connection refused
  1 × Database server unreachable
  1 × Disk usage at N%
  1 × Queue worker stopped
```

## Ishga tushirish

Talab: Rust (edition 2024) va `cargo`.

```bash
git clone https://github.com/<username>/log_tahlil.git
cd log_tahlil
```

v0.1 da fayl nomi kodda belgilangan: dastur loyiha papkasidagi `laravel.log` faylini o'qiydi.
Namuna bilan sinab ko'rish uchun:

```bash
cp namuna.log laravel.log     # Windows: copy namuna.log laravel.log
cargo run --release
```

## Qanday ishlaydi

**1. Yozuvlarni yig'ish.** Laravel logida bitta yozuv bir necha qatordan iborat bo'lishi mumkin (stack trace bilan).
Yangi yozuv boshi — qator boshidagi vaqt belgisi:

```
[2026-10-21 09:20:03] production.ERROR: Connection refused {"exception":"..."}
[stacktrace]
#0 /var/www/vendor/predis/Client.php(120): ...
```

Vaqt belgisi bilan boshlanmagan qatorlar oldingi yozuvga qo'shiladi.
Birinchi yozuvdan oldingi qatorlar tashlab yuboriladi.

**2. Daraja.** Laravel (Monolog) ning 8 ta darajasi taniladi:
`Debug`, `Info`, `Notice`, `Warning`, `Error`, `Critical`, `Alert`, `Emergency`.
Katta-kichik harf farqlanmaydi. Tanilmagan daraja asl matni bilan `Unknown(...)` sifatida saqlanadi.

**3. Sarlavha normalizatsiyasi.** Bir xil xato har safar boshqa qiymatlar bilan yoziladi
(`User 15 not found`, `User 42 not found`). Ularni bitta guruhga yig'ish uchun:

- xabar birinchi ` {` gacha qirqiladi (Laravel kontekst JSON'i tashlanadi)
- qo'shtirnoq ichidagi matn → `"..."`
- raqamlar ketma-ketligi → `N`

| Xabar | Sarlavha |
|---|---|
| `User 15 not found {"userId":15}` | `User N not found` |
| `Undefined variable "name" in line 42` | `Undefined variable "..." in line N` |

**4. Top xatolar.** Faqat `Error`, `Critical`, `Alert`, `Emergency` darajalari hisobga olinadi
(`Unknown` kirmaydi). Yozuvlar sarlavha bo'yicha guruhlanadi, soni bo'yicha kamayish tartibida
saralanadi, soni teng bo'lsa — alifbo tartibida. Eng ko'pi 10 ta chiqadi, qolganlari
`... va yana K ta` qatori bilan ko'rsatiladi.

## Loyiha tuzilmasi

```
src/
├── main.rs      # faylni o'qish, hisobotni chiqarish
├── lib.rs       # modullar
├── daraja.rs    # enum Daraja + matndan()
├── yozuv.rs     # struct Yozuv + sarlavha qatorini ajratish
├── yigish.rs    # qatorlarni yozuvlarga yig'ish + sarlavha normalizatsiyasi
└── hisobot.rs   # darajalar soni, top xatolar
```

Bog'liqliklar: [`regex`](https://docs.rs/regex), [`anyhow`](https://docs.rs/anyhow).

## Ma'lum cheklovlar

- Fayl nomi kodda belgilangan (`laravel.log`) — v0.2 da argument bo'ladi
- Xabar matnining o'zida ` {` bo'lsa, sarlavha noto'g'ri joydan qirqiladi
- Bir xil xato turli prefiks bilan yozilsa (masalan, oldida `Uncaught BadMethodCallException:`),
  ular alohida sarlavha bo'lib chiqadi

## Reja

- [x] **v0.1** — darajalar soni, top xatolar
- [ ] **v0.2** — `clap`: fayl yo'li argument sifatida, `--level` filtri
- [ ] **v0.3** — vaqt oralig'i: `--from` / `--to`
- [ ] **v0.4** — xato joyini stack trace'dan aniqlash