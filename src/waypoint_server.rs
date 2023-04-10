use  r2r::std_srvs::srv::Trigger;

use futures::StreamExt;
use std::sync::{Arc, Mutex};

pub async fn service_server(arc_node: Arc<Mutex<r2r::Node>>) -> Result<(), r2r::Error> {
    let mut service = {
        let mut node = arc_node.lock().unwrap();
        node.create_service::<Trigger::Service>("/waypoint_navigation")?
    };
    loop {
        match service.next().await {
            Some(req) => {
                let resp = Trigger::Response {
                    success: true,
                    message: "Got service call".to_string()
                };
                req.respond(resp).expect("Could not send service response");
            }
            None => break,
        }
    }
    Ok(())
}