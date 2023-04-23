pub mod constants;

mod spot_advisor_processor;

use crate::constants::SPOT_ADVISOR_DATA;
use crate::constants::EC2_PRICE_DATA;
use crate::spot_advisor_processor::process_spot_advisor_data;

fn main() {
    println!("{}", SPOT_ADVISOR_DATA);
    println!("{}", EC2_PRICE_DATA);
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(process_spot_advisor_data()) {
        Ok(map) => 
            for (key, value) in &map {
                println!("{}:{:?}", key, value);
            }
        Err(e) => println!("error"),
    }

}
