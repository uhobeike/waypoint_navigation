#[derive(Debug, Copy, Clone)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
    pub yaw: f64,
}

impl Waypoint {
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_yaw(&mut self, yaw: f64) {
        self.yaw = yaw;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_yaw(&self) -> f64 {
        self.yaw
    }

    pub fn get_quaternion_w(&self) -> f64 {
        (self.yaw / 2.0).cos()
    }

    pub fn get_quaternion_z(&self) -> f64 {
        (self.yaw / 2.0).sin()
    }
}
