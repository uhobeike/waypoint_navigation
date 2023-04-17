use colored::Colorize;
use waypoint_navigation::Waypoint;

use r2r::{nav2_msgs::action::NavigateToPose, ClockType::RosTime};

use async_std::task;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub async fn action_client(
    arc_node: Arc<Mutex<r2r::Node>>,
    way_nav_flag: Arc<Mutex<bool>>,
    waypoint: Vec<Waypoint>,
    action_server_name: &String,
) -> Result<(), r2r::Error> {
    let (client, service_available) = {
        let mut node = arc_node.lock().unwrap();
        let client = node.create_action_client::<NavigateToPose::Action>(action_server_name)?;
        let service_available = node.is_available(&client)?;
        (client, service_available)
    };
    println!("{}", "waiting for service...".yellow());
    service_available.await?;
    println!("{}", "service available.".green());

    let goal_distance = f32::default();
    let arc_goal_distance = Arc::new(Mutex::new(goal_distance));
    let feeedback_lock_flag = true;
    let arc_feeedback_lock_flag = Arc::new(Mutex::new(feeedback_lock_flag));
    let arc_waypoint = Arc::new(Mutex::new(waypoint));
    let arc_waypoint_indx = Arc::new(Mutex::new(usize::default()));

    loop {
        {
            let way_nav_flag = *way_nav_flag.clone().lock().unwrap();
            let agd = Arc::clone(&arc_goal_distance);
            let aflf = Arc::clone(&arc_feeedback_lock_flag);
            let awt = Arc::clone(&arc_waypoint);
            let awi = Arc::clone(&arc_waypoint_indx);

            if way_nav_flag && *aflf.lock().unwrap() && *agd.lock().unwrap() < 0.5 {
                println!("{:?}", awt.lock().unwrap().iter());
                let goal_pose = set_goal(awt.lock().unwrap()[*awi.lock().unwrap()]);
                *awi.lock().unwrap() += 1;
                *aflf.lock().unwrap() = false;

                let (goal, _result, feedback) = client
                    .send_goal_request(goal_pose)
                    .expect("")
                    .await
                    .expect("Goal Rejected");

                let task = task::spawn(async move {
                    feedback
                        .for_each(move |msg| {
                            let mut feeedback_lock_flag = aflf.lock().unwrap();
                            let goal = goal.clone();
                            let mut goal_distance = agd.lock().unwrap();
                            let waypoint = awt.lock().unwrap();
                            let waypoint_index = *awi.lock().unwrap();

                            *feeedback_lock_flag = true;
                            *goal_distance = ((waypoint[waypoint_index - 1].get_x()
                                - msg.current_pose.pose.position.x)
                                .powf(2.0)
                                + (waypoint[waypoint_index - 1].get_y()
                                    - msg.current_pose.pose.position.y)
                                    .powf(2.0))
                            .sqrt() as f32;

                            async move {
                                println!(
                                    "got feedback msg [ Distance Remaining: {:.3} -- {:?} ]",
                                    msg.distance_remaining,
                                    goal.get_status()
                                );
                            }
                        })
                        .await
                });

                let agd = Arc::clone(&arc_goal_distance);
                if *agd.lock().unwrap() < 0.5 {
                    task.cancel().await;
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

fn set_goal(waypoint: Waypoint) -> NavigateToPose::Goal {
    let mut goal_pose = NavigateToPose::Goal::default();

    r2r::Clock::create(r2r::ClockType::SystemTime).unwrap();
    let clock = r2r::Clock::create(RosTime);
    let now = clock.unwrap().get_now();

    let mut header = r2r::std_msgs::msg::Header::default();
    header.frame_id = "map".to_string();
    header.stamp = r2r::Clock::to_builtin_time(&now.unwrap());

    let mut pose = r2r::geometry_msgs::msg::Pose::default();

    pose.position.x = waypoint.get_x();
    pose.position.y = waypoint.get_y();
    pose.orientation.w = waypoint.get_quaternion_w();
    pose.orientation.z = waypoint.get_quaternion_z();

    goal_pose.pose.header = header;
    goal_pose.pose.pose = pose;

    goal_pose
}
