use waypoint_navigation::Waypoint;
extern crate yaml_rust;
use std::fs;
use std::{collections::HashMap, hash::Hash};
use yaml_rust::{yaml, YamlEmitter, YamlLoader};

pub struct YamlParser {
    pub yaml_path: String,
    pub yaml: Vec<yaml_rust::Yaml>,
}

impl YamlParser {
    pub fn load_yaml(&mut self) {
        let f = fs::read_to_string(self.yaml_path.clone());
        let s = f.unwrap().to_string();
        let yaml = YamlLoader::load_from_str(&s).unwrap();
        self.yaml = yaml;
    }

    pub fn get_waypoint(&self) -> Vec<Waypoint> {
        let yaml = self.yaml[0].clone();

        let mut vec_waypoint = Vec::new();
        for yaml in yaml["waypoints"].clone() {
            vec_waypoint.push(set_waypoint(yaml));
        }

        vec_waypoint
    }
}

fn set_waypoint(yaml: yaml_rust::Yaml) -> Waypoint {
    let mut waypoint = Waypoint {
        x: f64::default(),
        y: f64::default(),
        yaw: f64::default(),
    };

    waypoint.set_x(yaml["position"]["x"].as_f64().unwrap());
    waypoint.set_y(yaml["position"]["y"].as_f64().unwrap());
    waypoint.set_yaw(yaml["euler_angle"]["z"].as_f64().unwrap());

    waypoint
}

fn _get_waypoint_debug(yaml: yaml_rust::Yaml) {
    println!("{}", "Read Yaml Start");
    for waypoint in yaml {
        println!("{}", waypoint["id"].as_i64().unwrap());
        println!("{}", waypoint["position"]["x"].as_f64().unwrap());
        println!("{}", waypoint["position"]["y"].as_f64().unwrap());
        println!("{}", waypoint["euler_angle"]["z"].as_f64().unwrap());
    }
    println!("{}", "Read Yaml End");
}
