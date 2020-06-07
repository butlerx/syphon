use super::Metric;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn parse(msg: String) -> Vec<Metric> {
    msg.split("\n")
        .map(|message| message.split_whitespace().collect())
        .filter(|fields: &Vec<&str>| {
            let two = fields.len() == 2;
            let three = fields.len() == 3;
            two | three
        })
        .map(|fields: Vec<&str>| {
            if fields.len() == 2 {
                let (path, tags) = parse_tags(fields[0]);
                let value: f32 = fields[1].parse().unwrap();
                let timestamp = time_now();
                return Metric::new(path.to_string(), tags, value, timestamp);
            }
            let (path, tags) = parse_tags(fields[0]);
            let value: f32 = fields[1].parse().unwrap();
            let timestamp: u64 = fields[2].parse().unwrap();
            Metric::new(path.to_string(), tags, value, timestamp)
        })
        .collect()
}

fn parse_tags(path: &str) -> (&str, HashMap<String, String>) {
    let mut tags: Vec<&str> = path.split(";").collect();
    if tags.len() == 1 {
        return (tags[0], HashMap::new());
    }
    let metric_path: Vec<&str> = tags.drain(0..1).collect();
    let mut parsed_tags = HashMap::new();
    while let Some(tag) = tags.pop() {
        let field_value: Vec<&str> = tag.split("=").collect();
        parsed_tags.insert(field_value[0].to_string(), field_value[1].to_string());
    }
    (metric_path[0], parsed_tags)
}

fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
