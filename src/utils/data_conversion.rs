use log::{debug, error};
use serde::Deserialize;
use serde_json::Value;

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

pub fn parse_str_to_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        Some(Value::String(ref s)) => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
        Some(Value::Number(ref n)) => Ok(n.as_f64()),
        None => Ok(None), // フィールドが存在しない場合
        _ => Err(serde::de::Error::custom(
            "Expected a string, number, or null for f64",
        )),
    }
}
