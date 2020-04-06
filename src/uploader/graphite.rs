use crate::parser::Metric;

pub fn format(metric: Metric) -> String {
    let tags: Vec<String> = metric
        .tags()
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();
    format!(
        "{path}{tags} {value} {time}\n",
        path = metric.path(),
        tags = if tags.len() == 0 {
            "".to_string()
        } else {
            format!(";{}", tags.join(";"))
        },
        value = metric.value(),
        time = metric.time()
    )
}
