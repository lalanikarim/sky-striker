# Sky Striker

## Overview

Dive into a fast-paced, pixelated 2D aircraft shooter where you control a sleek fighter plane. Blast through waves of enemy aircraft and obstacles, manage your limited magazine capacity, and reload strategically to stay in the air. Collect power-ups and upgrade your ship to become the ultimate sky striker!

## Features

- Player-controlled aircraft with movement controls.
- Shooting mechanics with a limited magazine capacity and reload time.
- Simple bullet animation.
- Display of score and remaining bullet count.

## Setup

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```bash
   git clone https://github.com/lalanikarim/sky-striker.git
   cd sky-striker
   ```
3. Run the game using Cargo:
   ```bash
   cargo run
   ```

## Structure

- `src/main.rs`: Main entry point of the application.
- `src/constants.rs`: Constants used throughout the project.
- `src/components.rs`: Definitions of components used in the ECS.
- `src/systems.rs`: Systems that define game logic.

## Dependencies

This project uses Bevy and other dependencies managed by Cargo. Ensure you have Rust and Cargo installed to handle these dependencies automatically.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes.

## Credits

- [Bevy Game Engine](https://bevyengine.org/)
- Assets used in the game (ensure proper attribution if necessary).
