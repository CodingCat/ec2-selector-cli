pub mod constants;

use crate::constants::SPOT_ADVISOR_DATA;

use crate::constants::EC2_PRICE_DATA;

fn main() {
    println!("{}", SPOT_ADVISOR_DATA);
    println!("{}", EC2_PRICE_DATA);
}
