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

        get_waypoint_debug(yaml["waypoints"].clone());

        for waypoint in yaml["waypoints"].clone() {}

        let waypoint = Vec::new();
        waypoint
    }
}

fn get_waypoint_debug(yaml: yaml_rust::Yaml) {
    println!("{}", "Read Yaml Start");
    for waypoint in yaml {
        println!("{}", waypoint["id"].as_i64().unwrap());
        println!("{}", waypoint["position"]["x"].as_f64().unwrap());
        println!("{}", waypoint["position"]["y"].as_f64().unwrap());
        println!("{}", waypoint["euler_angle"]["z"].as_f64().unwrap());
    }
    println!("{}", "Read Yaml End");
}