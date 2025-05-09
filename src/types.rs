#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}