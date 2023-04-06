pub mod waypoint;

fn main() {
    let waypoint = waypoint::Waypoint {
        x: 1.0,
        y: 1.0,
        yaw: 0.1,
    };

    println!(
        "{}, {}, {}, {}, {}",
        waypoint.get_x(),
        waypoint.get_y(),
        waypoint.get_yaw(),
        waypoint.get_quaternion_w(),
        waypoint.get_quaternion_z()
    );
}
