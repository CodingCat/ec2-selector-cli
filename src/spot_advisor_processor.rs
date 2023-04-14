use std::{collections::HashMap, i32};
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
    println!("hello");
    println!("{}", content);
    Ok(content)
}

pub async fn process_spot_advisor_data() -> ()  {
    match get_json_content().await {
        Ok(file_content) => println!("{}", file_content),
        Err(e) => println!("Error: {}", e),
    }
}