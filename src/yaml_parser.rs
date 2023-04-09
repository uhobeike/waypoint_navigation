use waypoint_navigation::Waypoint;
extern crate yaml_rust;
use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

pub struct YamlParser {
    pub yaml_path: String,
    // pub waypoint: [Waypoint],
    pub yaml: Vec<yaml_rust::Yaml>,
}

impl YamlParser {
    pub fn load_yaml(&mut self) {
        let f = fs::read_to_string(self.yaml_path.clone());
        let s = f.unwrap().to_string();
        let yaml = YamlLoader::load_from_str(&s).unwrap();
        self.yaml = yaml;
    }
}
