mod model;
use crate::model::tick::PriceTick;

fn main() {
    let tick = PriceTick {
        venue: "kraken".to_string(),
        symbol: "ETH-USD".to_string(),
        timestamp: 1625247600,
        price: 2000.50,
        volume: 1.5,
    };
    println!("Here is a price tick: {:?}", tick);
}
