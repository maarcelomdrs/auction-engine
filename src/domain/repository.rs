use crate::domain::auction::Auction;
use crate::domain::errors::DomainError;

pub trait AuctionRepository: Send + Sync {
    fn save(&self, auction: Auction) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &str) -> Option<Auction>;
}