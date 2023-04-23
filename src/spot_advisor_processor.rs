use std::{collections::HashMap, i32};
use serde_json::{Value, Map};
use reqwest::Error;
use crate::constants::SPOT_ADVISOR_DATA;

#[derive(Debug)]
pub struct SpotInstanceInfo {
    num_cores: i32,
    size_memory_in_gb: i32,
    preemption_rate: f32
}

async fn get_json_content() -> Result<String, Error> {
    let response = reqwest::get(SPOT_ADVISOR_DATA).await?;
    let content = response.text().await?;
    Ok(content)
}

fn parse_json_into_map(json_str: String) -> Map<String, Value> {
    let json: Value = serde_json::from_str(&json_str).unwrap();

    if let Value::Object(map) = json {
        return map.into_iter().collect();
    } else {
        let map: Map<String, Value> = Map::new();
        return map;
    }
}
//println!("{:?}", hashmap);

pub async fn process_spot_advisor_data() -> Result<HashMap<String, SpotInstanceInfo>, Error>  {
    let mut instance_type_map: HashMap<String, SpotInstanceInfo> = HashMap::new();
    match get_json_content().await {
        Ok(file_content) => {
            let map = parse_json_into_map(file_content);
            for key in map.keys() {
                match key.to_string().as_str() {
                    "instance_types" => {
                        let instance_map = map.get(key).unwrap().as_object().unwrap();
                        for instance_type in instance_map.keys() {
                            instance_type_map.insert(instance_type.clone(),
                                 SpotInstanceInfo { num_cores: 0, size_memory_in_gb: 0, preemption_rate: 2.4 });
                        }
                    },
                    _ => ()
                }
            }
            return Ok(instance_type_map);
        },
        Err(e) => return Err(e),
    }
}