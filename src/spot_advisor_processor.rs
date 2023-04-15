use std::{collections::HashMap, i32};
use serde_json::{Value, Map};
use reqwest::Error;
use crate::constants::SPOT_ADVISOR_DATA;

struct SpotInstanceInfo {
    instance_type: String,
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

pub async fn process_spot_advisor_data() -> ()  {
    match get_json_content().await {
        Ok(file_content) => {
            let map = parse_json_into_map(file_content);
            for key in map.keys() {
                match key.to_string().as_str() {
                    "instance_types" => {
                        println!("{}", map.get("instance_types").unwrap().get("r5n.4xlarge").unwrap());
                    },
                    _ => ()
                }
            }
            /* 
            if let Some(value) = map.get("instance_types") {
                println!("{}", value);
            }  */ 
            //println!("{:?}", map);
        },
        Err(e) => println!("Error: {}", e),
    }
}