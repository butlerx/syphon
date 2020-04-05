use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{self, sync::broadcast::Receiver};
use std::collections::HashMap;

named_tuple!(
    #[derive(Clone, Debug)]
    struct Metric(path, tags, value, time);
);

pub async fn parse(mut recv: Receiver<String>){
    loop {
        for message in recv.recv().await.unwrap().split("\n"){
            let fields:Vec<&str> = message.split_whitespace().collect();
            if fields.len() == 2 {
                let (path, tags) = parse_tags(fields[0]);
                let value: f32 = fields[1].parse().unwrap();
                let timestamp = time_now();
                let metric = Metric::new(path, tags, value, timestamp);
                println!("{:?}", metric)
            } else if fields.len() == 3 {
                let (path, tags) = parse_tags(fields[0]);
                let value: f32 = fields[1].parse().unwrap();
                let timestamp: u64 = fields[2].parse().unwrap();
                let metric = Metric::new(path, tags, value, timestamp);
                println!("{:?}", metric)
            }
        }
    }
}

fn parse_tags(path: &str) -> (&str, HashMap<String, String>){
    let mut tags:Vec<&str> =  path.split(";").collect();
    if tags.len() == 1 { return (tags[0], HashMap::new()) }
    let metric_path = tags.first().cloned().unwrap();
    tags.drain(0..1);
    let mut parsed_tags = HashMap::new();
    while let Some(tag) = tags.pop() {
        let field_value:Vec<&str> = tag.split("=").collect();
        println!("{:?}", field_value);
        parsed_tags.insert(
            field_value[0].to_string(),
            field_value[1].to_string(),
        );
    }
    (metric_path, parsed_tags)
}

fn time_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
     since_the_epoch.as_secs()
}
