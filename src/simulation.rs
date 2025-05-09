use rand::Rng;
use crate::constants::*;
use crate::types::{Direction, TrafficLightState};
use crate::traffic_light::TrafficLight;
use crate::vehicle::Vehicle;

pub struct TrafficSimulation {
    pub vehicles: Vec<Vehicle>,
    pub traffic_lights: Vec<TrafficLight>,
    pub buffer: Vec<u32>,
    pub spawn_timer: f32,
}

impl TrafficSimulation {
    pub fn new() -> Self {
        let mut traffic_lights = Vec::new();
        
        traffic_lights.push(TrafficLight::new(
            WIDTH / 2 - ROAD_WIDTH - TRAFFIC_LIGHT_SIZE, 
            HEIGHT / 2 - ROAD_WIDTH / 2, 
            Direction::Right,
            TrafficLightState::Green
        ));
        
        traffic_lights.push(TrafficLight::new(
            WIDTH / 2 + ROAD_WIDTH, 
            HEIGHT / 2 + ROAD_WIDTH / 2, 
            Direction::Left,
            TrafficLightState::Green
        ));
        
        traffic_lights.push(TrafficLight::new(
            WIDTH / 2 - ROAD_WIDTH / 2,
            HEIGHT / 2 - ROAD_WIDTH - TRAFFIC_LIGHT_SIZE, 
            Direction::Down,
            TrafficLightState::Red
        ));
        
        traffic_lights.push(TrafficLight::new(
            WIDTH / 2 + ROAD_WIDTH / 2,
            HEIGHT / 2 + ROAD_WIDTH, 
            Direction::Up,
            TrafficLightState::Red
        ));
        
        TrafficSimulation {
            vehicles: Vec::new(),
            traffic_lights,
            buffer: vec![COLOR_BACKGROUND; WIDTH * HEIGHT],
            spawn_timer: 0.0,
        }
    }

    pub fn spawn_vehicle(&mut self, direction: Direction) {
        let (x, y) = match direction {
            Direction::Right => (10.0, HEIGHT as f32 / 2.0 - ROAD_WIDTH as f32 / 4.0),
            Direction::Down => (WIDTH as f32 / 2.0 - ROAD_WIDTH as f32 / 4.0, 10.0),
            Direction::Left => (WIDTH as f32 - 10.0, HEIGHT as f32 / 2.0 + ROAD_WIDTH as f32 / 4.0),
            Direction::Up => (WIDTH as f32 / 2.0 + ROAD_WIDTH as f32 / 4.0, HEIGHT as f32 - 10.0),
        };
        
        self.vehicles.push(Vehicle::new(x, y, direction));
    }

    pub fn update(&mut self, dt: f32) {
        self.traffic_lights[0].update(dt);
        self.traffic_lights[1].state = self.traffic_lights[0].state;
        self.traffic_lights[1].timer = self.traffic_lights[0].timer;
        
        self.traffic_lights[2].update(dt);
        self.traffic_lights[3].state = self.traffic_lights[2].state;
        self.traffic_lights[3].timer = self.traffic_lights[2].timer;
        
        if self.traffic_lights[0].state == TrafficLightState::Green {
            self.traffic_lights[2].state = TrafficLightState::Red;
            self.traffic_lights[3].state = TrafficLightState::Red;
        } else if self.traffic_lights[0].state == TrafficLightState::Red {
            if self.traffic_lights[2].state == TrafficLightState::Red && self.traffic_lights[2].timer < 1.0 {
                self.traffic_lights[2].state = TrafficLightState::Green;
                self.traffic_lights[3].state = TrafficLightState::Green;
            }
        }
        
        let vehicles_copy = self.vehicles.clone();
        
        let mut i = 0;
        while i < self.vehicles.len() {
            self.vehicles[i].update(dt, &self.traffic_lights, &vehicles_copy);
            
            if self.vehicles[i].x < -(VEHICLE_LENGTH as f32)
               || self.vehicles[i].x > WIDTH as f32 + VEHICLE_LENGTH as f32
               || self.vehicles[i].y < -(VEHICLE_LENGTH as f32)
               || self.vehicles[i].y > HEIGHT as f32 + VEHICLE_LENGTH as f32 {
                self.vehicles.swap_remove(i);
            } else {
                i += 1;
            }
        }
        
        self.spawn_timer += dt;
        if self.spawn_timer > 1.0 {
            self.spawn_timer = 0.0;
            
            if rand::rng().random::<f32>() < 0.3 {
                let direction = match rand::rng().random_range(0..4) {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Up,
                };
                
                self.spawn_vehicle(direction);
            }
        }
    }

    pub fn render(&mut self) {
        self.buffer.fill(COLOR_BACKGROUND);
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if y >= HEIGHT / 2 - ROAD_WIDTH && y <= HEIGHT / 2 + ROAD_WIDTH {
                    self.buffer[y * WIDTH + x] = COLOR_ROAD;
                    
                    if y == HEIGHT / 2 && x % 20 < 10 {
                        self.buffer[y * WIDTH + x] = COLOR_ROAD_MARKING;
                    }
                }
                if x >= WIDTH / 2 - ROAD_WIDTH && x <= WIDTH / 2 + ROAD_WIDTH {
                    self.buffer[y * WIDTH + x] = COLOR_ROAD;
                    
                    if x == WIDTH / 2 && y % 20 < 10 {
                        self.buffer[y * WIDTH + x] = COLOR_ROAD_MARKING;
                    }
                }
            }
        }
        
        for light in &self.traffic_lights {
            let color = light.get_color();
            
            for dy in 0..TRAFFIC_LIGHT_SIZE {
                for dx in 0..TRAFFIC_LIGHT_SIZE {
                    let x = light.x + dx;
                    let y = light.y + dy;
                    
                    if x < WIDTH && y < HEIGHT {
                        self.buffer[y * WIDTH + x] = color;
                    }
                }
            }
        }
        
        for vehicle in &self.vehicles {
            let x = vehicle.x as isize;
            let y = vehicle.y as isize;
            
            match vehicle.direction {
                Direction::Right | Direction::Left => {
                    for dy in 0..VEHICLE_WIDTH {
                        for dx in 0..VEHICLE_LENGTH {
                            let draw_x = match vehicle.direction {
                                Direction::Right => x + dx as isize,
                                Direction::Left => x - dx as isize,
                                _ => unreachable!(),
                            };
                            
                            let draw_y = y - VEHICLE_WIDTH as isize / 2 + dy as isize;
                            
                            if draw_x >= 0 && draw_y >= 0 && draw_x < WIDTH as isize && draw_y < HEIGHT as isize {
                                self.buffer[draw_y as usize * WIDTH + draw_x as usize] = COLOR_VEHICLE;
                            }
                        }
                    }
                },
                Direction::Down | Direction::Up => {
                    for dy in 0..VEHICLE_LENGTH {
                        for dx in 0..VEHICLE_WIDTH {
                            let draw_y = match vehicle.direction {
                                Direction::Down => y + dy as isize,
                                Direction::Up => y - dy as isize,
                                _ => unreachable!(),
                            };
                            
                            let draw_x = x - VEHICLE_WIDTH as isize / 2 + dx as isize;
                            
                            if draw_x >= 0 && draw_y >= 0 && draw_x < WIDTH as isize && draw_y < HEIGHT as isize {
                                self.buffer[draw_y as usize * WIDTH + draw_x as usize] = COLOR_VEHICLE;
                            }
                        }
                    }
                },
            }
        }
    }
}