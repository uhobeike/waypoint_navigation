pub mod send_goal;
pub mod waypoint_server;
pub mod yaml_parser;

use std::env;
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let action_server_name = "navigate_to_pose";
    let waypoint_navigation_flag = Arc::new(Mutex::new(false));

    let mut yaml = yaml_parser::YamlParser {
        yaml_path: args[1].to_string(),
        yaml: Vec::new(),
    };

    yaml.load_yaml();
    let waypoint = yaml.get_waypoint();

    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, "waypoint_navigation", "")?;
    let arc_node = Arc::new(Mutex::new(node));

    let an = arc_node.clone();
    let way_nav_flag = waypoint_navigation_flag.clone();
    task::spawn(async move {
        waypoint_server::service_server(an, way_nav_flag)
            .await
            .unwrap()
    });

    let an = arc_node.clone();
    let way_nav_flag = waypoint_navigation_flag.clone();
    task::spawn(async move {
        send_goal::action_client(an, way_nav_flag, waypoint, &action_server_name.to_string())
            .await
            .unwrap()
    });

    let handle = tokio::task::spawn_blocking(move || loop {
        {
            arc_node
                .lock()
                .unwrap()
                .spin_once(std::time::Duration::from_millis(10));
        }
        std::thread::sleep(std::time::Duration::from_millis(100))
    });

    handle.await?;

    Ok(())
}
