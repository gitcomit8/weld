# 🦀 Weld

**Fuse words. Forge security.**

**Weld** is a lightning-fast, interactive CLI utility built in **Rust**. It generates high-entropy "Diceware" style
passphrases that are easy for humans to remember but mathematically exhausting for machines to guess.

Unlike standard generators that require memorizing command-line flags, Weld features a fully interactive terminal UI (
TUI) and allows you to extend the core dictionary with your own custom words, which are seamlessly welded into the
generation pool.

## ✨ Features

* **🛡️ Cryptographically Secure:** Uses `rand`'s `ThreadRng` for unbiased selection from the EFF Large Wordlist.
* **🎮 Interactive TUI:** Navigate with a clean, menu-driven interface using arrow keys.
* **📂 Custom Word Banks:** Automatically detects and creates a local configuration folder. any words you add are
  persistently stored and mixed into the entropy pool.
* **⚙️ Flexible Output:** Customize word count, separators, and capitalization on the fly.
* **⚡ Blazingly Fast:** Built with Rust for instant startup and execution.

## 🚀 Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
git clone [https://github.com/yourusername/weld.git](https://github.com/yourusername/weld.git)
cd weld
cargo install --path .

```

📖 Usage
--------

Simply run the binary to enter the interactive menu:

Bash

```
weld

```

### The Menu

You will be presented with the following options:

1. **🚀 Generate Passphrase:** Instantly generates a secure passphrase (defaults to 4 words).

2. **➕ Add Custom Word:** Type a word to add it to your local `custom_words.txt`. It will immediately be available for
   future generations.

3. **⚙️ Configure Generation:** Change the default word count, separator (hyphens, spaces, etc.), and capitalization
   rules.

4. **❌ Exit:** Close the application.

📂 Data Storage
---------------

Weld stores your custom words in your operating system's standard configuration directory:

- **Linux:** `~/.config/weld/custom_words.txt`

- **Windows:** `C:\Users\You\AppData\Roaming\weld\custom_words.txt`

- **macOS:** `~/Library/Application Support/weld/custom_words.txt`

🛠️ Built With
--------------

- [Rust](https://www.rust-lang.org/)

- [Dialoguer](https://crates.io/crates/dialoguer) - for the TUI

- [Rand](https://crates.io/crates/rand) - for cryptographic randomness

- [Directories](https://crates.io/crates/directories) - for cross-platform config management

📄 License
----------

This project is licensed under the [MIT License](LICENSE).