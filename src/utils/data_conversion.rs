use serde_json::Value;
use log::{debug, error};
use serde::Deserialize;

pub fn parse_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(ref s) => {
            debug!("Parsing string to f64: {}", s);
            s.parse::<f64>().map_err(serde::de::Error::custom)
        }
        Value::Number(ref n) => {
            debug!("Parsing number to f64: {}", n);
            n.as_f64()
                .ok_or_else(|| serde::de::Error::custom("Failed to convert number to f64"))
        }
        _ => {
            error!("Unexpected type for f64: {:?}", value);
            Err(serde::de::Error::custom(
                "Expected a string or number for f64",
            ))
        }
    }
}