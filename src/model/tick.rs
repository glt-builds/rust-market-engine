#[derive(Debug, Clone, PartialEq)]
pub struct PriceTick {
    pub venue: String,    
    pub symbol: String,
    pub timestamp: u64,
    pub price: f64,
    pub volume: f64,
}

