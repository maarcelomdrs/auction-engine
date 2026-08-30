use crate::domain::bid::Bid;
use crate::domain::errors::DomainError;
use crate::domain::repository::AuctionRepository;

pub struct CloseAuctionCommand {
    pub auction_id: String,
    pub requester_id: String,
}

pub fn execute_close_auction(
    command: CloseAuctionCommand,
    repository: &dyn AuctionRepository,
) -> Result<Option<Bid>, DomainError> {
    const MAX_RETRIES: u32 = 3;
    let mut attempts = 0;

    loop {
        attempts += 1;

        let mut auction = repository
            .find_by_id(&command.auction_id)
            .ok_or(DomainError::AuctionNotFound)?;

        let winner = auction.close(&command.requester_id)?;

        match repository.save(auction) {
            Ok(_) => return Ok(winner),
            Err(DomainError::OptimisticLockError) if attempts < MAX_RETRIES => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}