#[derive(Debug, Clone)]
pub struct Bid {
    pub bidder_id: String,
    pub amount: f64,
}

impl Bid {
    pub fn new(bidder_id: String, amount: f64) -> Result<Self, &'static str> {
        if amount <= 0.0 {
            return Err("O valor do lance deve ser maior que zero.");
        }
        Ok(Self { bidder_id, amount })
    }
}