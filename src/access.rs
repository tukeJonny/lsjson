use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::Value as JSONValue;
use serde_json::map::Map as JSONMap;


pub fn read_json<P: AsRef<Path>>(path: P) -> Result<JSONMap<String, JSONValue>> {
    let content = fs::read_to_string(path)?;
    parse_json(&content)
}

fn parse_json(content: &str) -> Result<JSONMap<String, JSONValue>> {
    let deserialized: JSONMap<String, JSONValue> = serde_json::from_str(content)?;
    Ok(deserialized)
}

fn walk_json(m: JSONMap<String, JSONValue>, cur: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for (key, value) in m.into_iter() {
        let next_cur = [cur, &key].join(".");
        vec.push(next_cur.clone());
        if value.is_object() {
            let mut deep_keys = walk_json(value.as_object().unwrap().clone(), &next_cur);
            vec.append(&mut deep_keys);
        }
    }

    vec
}

pub fn get_json_keys<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let json_map = read_json(path)?;
    let mut keys = walk_json(json_map, &"");
    keys.sort();
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_json() {
        let simple_data = r#"
            {
                "key1": "value1",
                "key2": {
                    "key3": "value2"
                },
                "key4": [
                    "value3",
                    "value4"
                ]
            }"#;
        let j = parse_json(&simple_data).unwrap();
        assert_eq!(j.get("key1").unwrap(), "value1");
    }
}