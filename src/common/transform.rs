use bytes::Bytes;
use serde_json::Value;

pub fn value_to_bytes(value: &Value) -> Result<Bytes, anyhow::Error> {
    let vec = serde_json::to_vec(value)?;

    Ok(Bytes::from(vec))
}
