use super::Metric;
use itertools::Itertools;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

/// Parse messagaes to an array of metrics
/// Message is considered one packet that could contain 1 or more metrics
/// Each line is parsed, those that dont follow the following format are dropped.
/// <series>;[<tags>] <value> [<timestamp>]
/// Tags are optional key value pairs sperated by `;`
/// if no timestamp is provided or a -1 is recieved the current time at parsing will be used
pub fn parse(msg: String) -> Vec<Metric> {
    msg.split('\n')
        .map(|message| message.split_whitespace().collect())
        .filter(|fields: &Vec<&str>| {
            let two = fields.len() == 2;
            let three = fields.len() == 3;
            two | three
        })
        .map(|fields: Vec<&str>| {
            // No timestamp provided so use current time
            if fields.len() == 2 || fields[2] == "-1" {
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

/// Parse a graphite series for tags
/// if no tags provided a empty hashmap is returned
/// string is split on `;` where the first field is the series name
/// the remaining is parsed as key value pairs and returned as  hashmap
fn parse_tags(path: &str) -> (&str, HashMap<String, String>) {
    let mut tags: Vec<&str> = path.split(';').collect();
    if tags.len() == 1 {
        return (tags[0], HashMap::new());
    }
    let metric_path: Vec<&str> = tags.drain(0..1).collect();
    let parsed_tags = tags
        .into_iter()
        .map(|tag: &str| {
            let (key, value) = tag.split('=').collect_tuple().unwrap();
            (key.to_string(), value.to_string())
        })
        .collect::<HashMap<String, String>>();
    (metric_path[0], parsed_tags)
}

/// Return the current epoch time to be consumed by graphite
fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
