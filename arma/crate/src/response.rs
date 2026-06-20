use crate::log;

pub(crate) fn json<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
