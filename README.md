# 🎮 CHIP-8 Emulator

<div align="center">
  
  ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
  ![SDL2](https://img.shields.io/badge/SDL2-1E1E1E?style=for-the-badge&logo=sdl&logoColor=white)
  ![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)
  
  **A high-performance CHIP-8 emulator written in Rust with SDL2**
  
</div>

---

## ✨ Features

- 🚀 **Fast & Accurate** - Cycle-accurate emulation with configurable CPU speed
- 🎨 **Customizable Display** - Adjustable colors, scale, and refresh rate
- 🎮 **Full Input Support** - Complete 16-key hexadecimal keypad emulation
- 📼 **GIF Recording** - Capture gameplay moments with built-in recording
- ⚙️ **Configurable** - TOML-based configuration for easy customization
- 🔧 **Debug Mode** - Built-in debugging capabilities for development

## 🎯 Demos

<div align="center">
  <table>
    <tr>
      <td align="center">
        <img src="media/chip8_recording_1754270801.gif" alt="CHIP-8 Demo 1" width="400"/>
        <br />
        <em>Classic Game Running</em>
      </td>
      <td align="center">
        <img src="media/chip8_PONG2_1754507695.gif" alt="PONG2 Demo" width="400"/>
        <br />
        <em>PONG2 in Action</em>
      </td>
    </tr>
  </table>
</div>

## 🚀 Quick Start

### Prerequisites

- Rust (1.70 or later)
- SDL2 development libraries

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/chip8-emulator.git
cd chip8-emulator

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

- **Space** - Start/Stop GIF recording
- **Esc** - Quit emulator

## ⚙️ Configuration

The emulator can be configured via `chip8_config.toml`:

```toml
[display]
scale = 10
fg_color = [0, 255, 0]     # Green pixels
bg_color = [0, 0, 0]        # Black background

[cpu]
clock_speed = 700           # Instructions per second

[timers]
delay_timer_hz = 60
sound_timer_hz = 60

[gif]
enabled = true
output_dir = "./"
max_duration_secs = 30
```

## 🏗️ Architecture

The emulator is built with a modular architecture:

- **CPU Module** - Implements all 35 CHIP-8 opcodes
- **Display Module** - Handles 64x32 monochrome display rendering
- **Timer Module** - Manages delay and sound timers at 60Hz
- **Settings Module** - TOML-based configuration management
- **GIF Recorder** - Real-time gameplay recording to GIF

## 📚 Supported Opcodes

The emulator implements the complete CHIP-8 instruction set:

- **Flow Control**: `RET`, `JP`, `CALL`, `SE`, `SNE`, `SKP`, `SKNP`
- **Memory**: `LD`, `ADD`, `LDI`, `LDF`, `LDB`, `ST`, `LD[I]`
- **Graphics**: `CLS`, `DRW`, `SCD`, `SCR`, `SCL`
- **Arithmetic**: `ADD`, `SUB`, `SUBN`, `OR`, `AND`, `XOR`, `SHR`, `SHL`
- **Random**: `RND`

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

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

- CHIP-8 was originally developed by Joseph Weisbecker
- SDL2 bindings for Rust by the rust-sdl2 team
- The amazing CHIP-8 community for documentation and test ROMs

---

<div align="center">
  <strong>Built with ❤️ in Rust</strong>
</div>