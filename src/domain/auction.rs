use crate::domain::bid::Bid;
use crate::domain::errors::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub enum AuctionStatus {
    Active,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Auction {
    pub id: String,
    pub owner_id: String,
    pub minimum_increment: f64,
    pub current_highest_bid: Option<Bid>,
    pub status: AuctionStatus,
}

impl Auction {
    pub fn new(id: String, owner_id: String, minimum_increment: f64) -> Self {
        Self {
            id,
            owner_id,
            minimum_increment,
            current_highest_bid: None,
            status: AuctionStatus::Active,
        }
    }

    pub fn place_bid(&mut self, bid: Bid) -> Result<(), DomainError> {
        if self.status != AuctionStatus::Active {
            return Err(DomainError::AuctionNotActive);
        }

        if bid.bidder_id == self.owner_id {
            return Err(DomainError::BidOnOwnAuction);
        }

        let minimum_required = match &self.current_highest_bid {
            Some(highest) => highest.amount + self.minimum_increment,
            None => self.minimum_increment,
        };

        if bid.amount < minimum_required {
            return Err(DomainError::BidTooLow {
                minimum: minimum_required,
                provided: bid.amount,
            });
        }

        self.current_highest_bid = Some(bid);
        Ok(())
    }
}