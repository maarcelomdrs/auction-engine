#[derive(Debug, PartialEq)]
pub enum DomainError {
    AuctionNotActive,
    AuctionAlreadyClosed,
    OnlyOwnerCanCloseAuction,
    BidTooLow { minimum: f64, provided: f64 },
    BidOnOwnAuction,
    AuctionNotFound,
    OptimisticLockError,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::AuctionNotActive => write!(f, "O leilão não está ativo."),
            DomainError::AuctionAlreadyClosed => write!(f, "O leilão já se encontra encerrado."),
            DomainError::OnlyOwnerCanCloseAuction => write!(f, "Apenas o criador do leilão pode encerrá-lo."),
            DomainError::BidTooLow { minimum, provided } => {
                write!(f, "O lance de {} é menor que o mínimo exigido de {}.", provided, minimum)
            }
            DomainError::BidOnOwnAuction => write!(f, "O dono do leilão não pode dar lances no próprio item."),
            DomainError::AuctionNotFound => write!(f, "O leilão solicitado não foi encontrado."),
            DomainError::OptimisticLockError => write!(f, "Conflito de concorrência: os dados foram modificados por outro processo."),
        }
    }
}

impl std::error::Error for DomainError {}