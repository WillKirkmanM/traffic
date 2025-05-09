use rand::Rng;
use crate::constants::*;
use crate::types::{Direction, TrafficLightState};
use crate::traffic_light::TrafficLight;

#[derive(Clone)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub direction: Direction,
    pub stopped: bool,
}

impl Vehicle {
    pub fn new(x: f32, y: f32, direction: Direction) -> Self {
        let max_speed = 60.0 + rand::rng().random_range(0.0..40.0);
        Vehicle {
            x,
            y,
            speed: max_speed / 2.0,
            max_speed,
            direction,
            stopped: false,
        }
    }

    pub fn update(&mut self, dt: f32, traffic_lights: &[TrafficLight], vehicles: &[Vehicle]) {
        let mut should_stop = false;
        
        for light in traffic_lights {
            if light.direction == self.direction && light.state != TrafficLightState::Green {
                match self.direction {
                    Direction::Right => {
                        if self.x < light.x as f32 && self.x > light.x as f32 - 100.0 {
                            should_stop = true;
                        }
                    },
                    Direction::Down => {
                        if self.y < light.y as f32 && self.y > light.y as f32 - 100.0 {
                            should_stop = true;
                        }
                    },
                    Direction::Left => {
                        if self.x > light.x as f32 && self.x < light.x as f32 + 100.0 {
                            should_stop = true;
                        }
                    },
                    Direction::Up => {
                        if self.y > light.y as f32 && self.y < light.y as f32 + 100.0 {
                            should_stop = true;
                        }
                    },
                }
            }
        }
        
        for vehicle in vehicles {
            if self as *const _ != vehicle as *const _ && self.direction == vehicle.direction {
                let distance = match self.direction {
                    Direction::Right => {
                        if vehicle.x > self.x && (self.y - vehicle.y).abs() < ROAD_WIDTH as f32 / 4.0 {
                            vehicle.x - self.x
                        } else {
                            f32::MAX
                        }
                    },
                    Direction::Down => {
                        if vehicle.y > self.y && (self.x - vehicle.x).abs() < ROAD_WIDTH as f32 / 4.0 {
                            vehicle.y - self.y
                        } else {
                            f32::MAX
                        }
                    },
                    Direction::Left => {
                        if vehicle.x < self.x && (self.y - vehicle.y).abs() < ROAD_WIDTH as f32 / 4.0 {
                            self.x - vehicle.x
                        } else {
                            f32::MAX
                        }
                    },
                    Direction::Up => {
                        if vehicle.y < self.y && (self.x - vehicle.x).abs() < ROAD_WIDTH as f32 / 4.0 {
                            self.y - vehicle.y
                        } else {
                            f32::MAX
                        }
                    },
                };
                
                if distance < VEHICLE_LENGTH as f32 * 2.0 {
                    should_stop = true;
                    break;
                }
            }
        }

        self.stopped = should_stop;
        
        if should_stop {
            self.speed = (self.speed - 50.0 * dt).max(0.0);
        } else {
            self.speed = (self.speed + 20.0 * dt).min(self.max_speed);
        }

        match self.direction {
            Direction::Right => self.x += self.speed * dt,
            Direction::Down => self.y += self.speed * dt,
            Direction::Left => self.x -= self.speed * dt,
            Direction::Up => self.y -= self.speed * dt,
        }
    }
}