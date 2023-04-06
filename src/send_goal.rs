use waypoint_navigation::Waypoint;

use r2r::nav2_msgs::action::NavigateToPose;

use std::sync::{Arc, Mutex};
pub async fn client(arc_node: Arc<Mutex<r2r::Node>>) -> Result<(), r2r::Error> {
    let (client, mut timer, service_available) = {
        let mut node = arc_node.lock().unwrap();
        let client = node.create_client::<NavigateToPose::SendGoal::Service>("/add_two_ints")?;
        let timer = node.create_wall_timer(std::time::Duration::from_secs(2))?;
        let service_available = node.is_available(&client)?;
        (client, timer, service_available)
    };
    println!("waiting for service...");
    service_available.await?;
    println!("service available.");
    let uuid = r2r::unique_identifier_msgs::msg::UUID::default();
    let goal_pose = NavigateToPose::Goal::default();
    let req = NavigateToPose::SendGoal::Request {
        goal_id: uuid,
        goal: goal_pose,
    };
    if let Ok(_resp) = client.request(&req).unwrap().await {
        println!("Debug");
    }

    let waypoint = Waypoint {
        x: 1.0,
        y: 1.0,
        yaw: 0.1,
    };
    timer.tick().await?;
    Ok(())
}
