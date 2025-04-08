use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

pub fn deserialize_to_string_map<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: Option<HashMap<String, serde_json::Value>> = Option::deserialize(deserializer)?;
    if let Some(map) = map {
        let transformed_map = map
            .into_iter()
            .map(|(key, value)| (key, value.to_string().trim_matches('"').to_string()))
            .collect();
        Ok(Some(transformed_map))
    } else {
        Ok(None)
    }
}