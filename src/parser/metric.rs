use std::collections::HashMap;

named_tuple!(
    #[derive(Clone, Debug)]
    pub struct Metric {
        pub path: String,
        pub tags: HashMap<String, String>,
        pub value: f32,
        pub time: u64,
    }
);
