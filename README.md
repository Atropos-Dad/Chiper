# 🍟 Chiper - A CHIP-8 Interpreter

<div align="center">
  
  ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
  ![winit](https://img.shields.io/badge/winit-1E1E1E?style=for-the-badge&logoColor=white)
  ![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)
  
  **Another rust CHIP-8 Interpreter written in Rust**
  
</div>

---

This was done MOSTLY for fun and learning, but I do think this is pretty cute and has some features I think make chip8 experiences nicer! Mostly the *phosphor* effect which mitigates some of [chip8's flicker](www.reddit.com/r/EmuDev/comments/n9dcli/is_chip_8_emulator_flicker_normal/). 
Some stuff still needs to be done (especially interpreting the whole suite of chip8 derivatives!).



## ✨ Features

- 🎨 **Customizable Display** - Adjustable colors, scale, and refresh rate
- 📼 **GIF Recording** - Capture gameplay moments with built-in recording
- ⚙️ **Configurable** - TOML-based configuration for easy customization

## 🎯 Demos

<div align="center">
  <table>
    <tr>
      <td align="center">
        <img src="media/chip8_recording_1754270801.gif" alt="CHIP-8 Demo 1" width="400" loading="lazy"/>
        <br />
        <em>Space Invaders!</em>
      </td>
      <td align="center">
        <img src="media/chip8_PONG2_1754507695.gif" alt="PONG2 Demo" width="400" loading="lazy"/>
        <br />
        <em>PONG2 in Action</em>
      </td>
    </tr>
  </table>
  </div>

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/Atropos-Dad/Chiper.git
cd Chiper

# Build the project
cargo build --release

# Run with a ROM
cargo run --release -- path/to/rom.ch8
```

## 🎮 Controls

The CHIP-8 uses a 16-key hexadecimal keypad. Here's the default keyboard mapping:

```
Original CHIP-8 Keypad:      Keyboard Mapping:
┌───┬───┬───┬───┐            ┌───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ C │            │ 1 │ 2 │ 3 │ 4 │
├───┼───┼───┼───┤            ├───┼───┼───┼───┤
│ 4 │ 5 │ 6 │ D │            │ Q │ W │ E │ R │
├───┼───┼───┼───┤            ├───┼───┼───┼───┤
│ 7 │ 8 │ 9 │ E │            │ A │ S │ D │ F │
├───┼───┼───┼───┤            ├───┼───┼───┼───┤
│ A │ 0 │ B │ F │            │ Z │ X │ C │ V │
└───┴───┴───┴───┘            └───┴───┴───┴───┘
```

### Special Keys

- **Ctrl + R** - Start/Stop GIF recording

## 🔧 Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run -- --debug rom.ch8

# Check code
cargo check
cargo clippy
```

## 🙏 Acknowledgments

- CHIP-8 was originally developed by Joseph Weisbecker
- The amazing CHIP-8 community for documentation and test ROMs

---
