use crate::domain::auction::Auction;
use crate::domain::bid::Bid;
use crate::domain::errors::DomainError;

pub struct PlaceBidCommand {
    pub auction_id: String,
    pub bidder_id: String,
    pub amount: f64,
}

pub struct PlaceBidUseCase;

impl PlaceBidUseCase {
    pub fn execute(auction: &mut Auction, command: PlaceBidCommand) -> Result<(), DomainError> {
        let bid = Bid::new(command.bidder_id, command.amount)
            .map_err(|_| DomainError::BidTooLow { minimum: 0.0, provided: command.amount })?;

        auction.place_bid(bid)
    }
}