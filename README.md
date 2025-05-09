<p align="center">
  <img src="https://avatars.githubusercontent.com/u/138057124?s=200&v=4" width="150" />
</p>
<h1 align="center">Traffic</h1>

<p align="center">
  
</p>

<h4 align="center">
  <a href=""></a>
</h4>

<p align="center">Four Way Traffic Simulator with Lights & Vehicles Following Traffic Rules
</p>

## Features

- **Intersection with Traffic Lights**: Four-way intersection with coordinated traffic lights
- **Vehicle AI**: Vehicles that react to traffic lights and avoid collisions
- **Realistic Movement**: Vehicles accelerate, decelerate, and stop based on traffic conditions
- **Visual Feedback**: Color-coded traffic lights and directional vehicles

## Traffic Light Algorithm

The simulation implements a simple time-based traffic light algorithm:

1. **Coordination**: East-west and north-south lights are coordinated to avoid conflicts
2. **Cycle**: Each light cycles through Red (5s) → Green (5s) → Yellow (2s) → Red
3. **Priority**: When east-west lights are green, north-south lights are red and vice versa

## Vehicle Behavior

Vehicles in the simulation exhibit several intelligent behaviors:

- Stop at red and yellow traffic lights
- Maintain safe distances from other vehicles
- Accelerate when the path is clear
- Travel at different speeds (randomized for each vehicle)
- Automatically despawn when out of view

## Running the Simulation

### Prerequisites

- Rust and Cargo (1.39 or newer)

### Installation

1. Clone this repository
```bash
git clone https://github.com/WillKirkmanM/traffic.git
```

2. Build and run:

```bash
cargo build --release
cargo run --release
```
