use crate::domain::bid::Bid;
use crate::domain::errors::DomainError;

#[derive(Clone, Debug, PartialEq)]
pub enum AuctionStatus {
    Active,
    Closed,
}

#[derive(Clone)]
pub struct Auction {
    pub id: String,
    pub owner_id: String,
    pub minimum_increment: f64,
    pub current_highest_bid: Option<Bid>,
    pub status: AuctionStatus,
    pub version: u64,
}

impl Auction {
    pub fn new(id: String, owner_id: String, minimum_increment: f64) -> Self {
        Self {
            id,
            owner_id,
            minimum_increment,
            current_highest_bid: None,
            status: AuctionStatus::Active,
            version: 1,
        }
    }

    pub fn place_bid(&mut self, bid: Bid) -> Result<(), DomainError> {
        if self.status == AuctionStatus::Closed {
            return Err(DomainError::AuctionNotActive);
        }

        if self.owner_id == bid.bidder_id {
            return Err(DomainError::BidOnOwnAuction);
        }

        if let Some(ref highest_bid) = self.current_highest_bid {
            if bid.amount <= highest_bid.amount {
                return Err(DomainError::BidTooLow {
                    minimum: highest_bid.amount + 0.01,
                    provided: bid.amount,
                });
            }
        } else {
            if bid.amount < self.minimum_increment {
                return Err(DomainError::BidTooLow {
                    minimum: self.minimum_increment,
                    provided: bid.amount,
                });
            }
        }

        self.current_highest_bid = Some(bid);
        self.version += 1;
        Ok(())
    }

    pub fn close(&mut self, requester_id: &str) -> Result<Option<Bid>, DomainError> {
        if self.status == AuctionStatus::Closed {
            return Err(DomainError::AuctionAlreadyClosed);
        }

        if self.owner_id != requester_id {
            return Err(DomainError::OnlyOwnerCanCloseAuction);
        }

        self.status = AuctionStatus::Closed;
        self.version += 1;
        Ok(self.current_highest_bid.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_increment_version_on_new_bid() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        assert_eq!(auction.version, 1);

        let bid = Bid::new("user_2".to_string(), 150.0).unwrap();
        auction.place_bid(bid).unwrap();
        assert_eq!(auction.version, 2);
    }

    #[test]
    fn should_not_accept_bid_when_auction_is_closed() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        auction.status = AuctionStatus::Closed;
        let bid = Bid::new("user_2".to_string(), 150.0).unwrap();

        let result = auction.place_bid(bid);
        assert_eq!(result.unwrap_err(), DomainError::AuctionNotActive);
    }

    #[test]
    fn should_not_accept_bid_lower_than_current() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        let bid = Bid::new("user_2".to_string(), 90.0).unwrap();
        let result = auction.place_bid(bid);
        assert!(result.is_err());
    }

    #[test]
    fn should_accept_higher_bid() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        let bid = Bid::new("user_2".to_string(), 150.0).unwrap();
        let result = auction.place_bid(bid);
        assert!(result.is_ok());
        assert_eq!(auction.current_highest_bid.unwrap().amount, 150.0);
    }

    #[test]
    fn should_close_auction_and_return_winner() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        let bid = Bid::new("user_winner".to_string(), 150.0).unwrap();
        auction.place_bid(bid).unwrap();

        let winner = auction.close("owner_1").unwrap();
        assert_eq!(auction.status, AuctionStatus::Closed);
        assert_eq!(winner.unwrap().bidder_id, "user_winner");
    }

    #[test]
    fn should_not_allow_non_owner_to_close_auction() {
        let mut auction = Auction::new("1".to_string(), "owner_1".to_string(), 100.0);
        let result = auction.close("intruder_user");
        assert_eq!(result.unwrap_err(), DomainError::OnlyOwnerCanCloseAuction);
    }
}