use crate::constants::*;
use crate::types::{Direction, TrafficLightState};

pub struct TrafficLight {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub state: TrafficLightState,
    pub timer: f32,
    pub red_duration: f32,
    pub yellow_duration: f32,
    pub green_duration: f32,
}

impl TrafficLight {
    pub fn new(x: usize, y: usize, direction: Direction, initial_state: TrafficLightState) -> Self {
        TrafficLight {
            x,
            y,
            direction,
            state: initial_state,
            timer: 0.0,
            red_duration: 5.0,
            yellow_duration: 2.0,
            green_duration: 5.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        
        match self.state {
            TrafficLightState::Red => {
                if self.timer >= self.red_duration {
                    self.state = TrafficLightState::Green;
                    self.timer = 0.0;
                }
            }
            TrafficLightState::Yellow => {
                if self.timer >= self.yellow_duration {
                    self.state = TrafficLightState::Red;
                    self.timer = 0.0;
                }
            }
            TrafficLightState::Green => {
                if self.timer >= self.green_duration {
                    self.state = TrafficLightState::Yellow;
                    self.timer = 0.0;
                }
            }
        }
    }

    pub fn get_color(&self) -> u32 {
        match self.state {
            TrafficLightState::Red => COLOR_LIGHT_RED,
            TrafficLightState::Yellow => COLOR_LIGHT_YELLOW,
            TrafficLightState::Green => COLOR_LIGHT_GREEN,
        }
    }
}