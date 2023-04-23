use std::{collections::HashMap, i32};
use serde_json::{Value, Map};
use reqwest::Error;
use crate::constants::SPOT_ADVISOR_DATA;

#[derive(Debug)]
pub struct InstanceResourceInfo {
    num_cores: i32,
    size_memory_in_gb: f32
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

pub async fn process_spot_advisor_data() -> Result<HashMap<String, InstanceResourceInfo>, Error>  {
    let mut instance_type_map: HashMap<String, InstanceResourceInfo> = HashMap::new();
    match get_json_content().await {
        Ok(file_content) => {
            let map = parse_json_into_map(file_content);
            for key in map.keys() {
                match key.to_string().as_str() {
                    "instance_types" => {
                        let instance_map = map.get(key).unwrap().as_object().unwrap();
                        for instance_type in instance_map.keys() {
                            let resource_map = instance_map.get(instance_type).unwrap().as_object().unwrap();
                            let cores = resource_map["cores"].as_i64().unwrap() as i32;
                            let ram_gb = resource_map["ram_gb"].as_f64().unwrap() as f32;
                            instance_type_map.insert(instance_type.clone(),
                                 InstanceResourceInfo { num_cores: cores, size_memory_in_gb: ram_gb});
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