#[derive(Debug, PartialEq)]
pub enum DomainError {
    AuctionNotActive,
    BidTooLow { minimum: f64, provided: f64 },
    BidOnOwnAuction,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::AuctionNotActive => write!(f, "O leilão não está ativo."),
            DomainError::BidTooLow { minimum, provided } => {
                write!(f, "O lance de {} é menor que o mínimo exigido de {}.", provided, minimum)
            }
            DomainError::BidOnOwnAuction => write!(f, "O dono do leilão não pode dar lances no próprio item."),
        }
    }
}

impl std::error::Error for DomainError {}