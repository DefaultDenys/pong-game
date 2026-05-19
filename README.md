# Pong Game

A classic Pong game built with Rust and [macroquad](https://macroquad.rs/).

## Features

- Two-player local multiplayer
- Ball physics with increasing speed
- Score tracking
- Simple retro visuals

## Controls

| Player | Move Up | Move Down |
|--------|---------|-----------|
| Player 1 (Left) | `W` | `S` |
| Player 2 (Right) | `↑` | `↓` |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable)

## Running

```bash
cargo run
```

## Building (release)

```bash
cargo build --release
```

The binary will be at `target/release/pong-game.exe`.

## Tech Stack

- **Language:** Rust (edition 2024)
- **Graphics:** [macroquad](https://macroquad.rs/) 0.4
