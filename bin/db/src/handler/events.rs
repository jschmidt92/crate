#[derive(Debug)]
pub enum ForgeEvent {
    Greet(String),
    Get(String),
    Set {
        key: String,
        value: String,
    },
    Del(String),
    Exists(String),
    HGetAll(String),
    HGet {
        key: String,
        field: String,
    },
    HSet {
        key: String,
        field: String,
        value: String,
    },
    HMGet {
        key: String,
        fields: Vec<String>,
    },
    HMSet {
        key: String,
        fields: HashMap<String, String>,
    },
    HDel {
        key: String,
        field: String,
    },
    HExists {
        key: String,
        field: String,
    },
    HLen(String),
    HKeys(String),
    HVals(String),
}
