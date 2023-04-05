pub struct Waypoint {
    x: f64,
    y: f64,
    yaw: f64,
}

impl Waypoint {
    fn getX(&self) -> f64 {
        self.x
    }

    fn getY(&self) -> f64 {
        self.y
    }

    fn getYaw(&self) -> f64 {
        self.yaw
    }

    fn getQuaternionW(&self) -> f64 {
        (self.yaw / 2.0).cos()
    }

    fn getQuaternionZ(&self) -> f64 {
        (self.yaw / 2.0).sin()
    }
}
