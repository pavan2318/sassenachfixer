# SassenachFixer

A cheeky Scots ↔ English translator built in Rust.

Turns confusing Scots into plain English… and fixes your English right back.

---

## Features

- Scots → English translation
- English → Scots translation
- Lightweight CLI tool
- Fast and simple (no external dependencies beyond Rust)

---

## Usage

Translate Scots to English:
```
sassenachfixer fix "Aye ye ken"
yes you know
```
Translate English to Scots:
```
sassenachfixer fix --to-scots "I cannot go home"
I cannae go hame
```
---

## Build
```
cargo build
```
---

## Run
```
./target/debug/sassenachfixer fix "Aye ye ken"
```
---

## Project Structure

src/        → core Rust logic  
data/       → dictionary data  
README.md   → documentation  
LICENSE     → MIT license  

---

## Roadmap

- Phrase-level translation (not just single words)
- Punctuation handling
- Capitalisation fixes
- Interactive chat mode
- GUI version

---

## Why?

Because sometimes ye just need things translated properly.

---

## License

MIT License
