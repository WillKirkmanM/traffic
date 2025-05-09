use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use traffic::constants::*;
use traffic::simulation::TrafficSimulation;

fn main() {
    let mut window = Window::new(
        "Traffic Simulation",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });
    
    window.set_target_fps(60);
    
    let mut simulation = TrafficSimulation::new();
    let mut last_time = Instant::now();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32() * SIMULATION_SPEED;
        last_time = current_time;
        
        simulation.update(dt);
        
        simulation.render();
        
        window.update_with_buffer(&simulation.buffer, WIDTH, HEIGHT).unwrap();
    }
}