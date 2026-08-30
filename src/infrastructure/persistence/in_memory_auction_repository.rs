use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::auction::Auction;
use crate::domain::errors::DomainError;
use crate::domain::repository::AuctionRepository;

#[derive(Clone)]
pub struct InMemoryAuctionRepository {
    storage: Arc<Mutex<HashMap<String, Auction>>>,
}

impl InMemoryAuctionRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AuctionRepository for InMemoryAuctionRepository {
    fn save(&self, auction: Auction) -> Result<(), DomainError> {
        let mut map = self.storage.lock().unwrap();

        if let Some(existing) = map.get(&auction.id) {
            // Se a versão atual no banco não for exatamente anterior à nova, houve conflito
            if existing.version >= auction.version {
                return Err(DomainError::OptimisticLockError);
            }
        }

        map.insert(auction.id.clone(), auction);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Option<Auction> {
        let map = self.storage.lock().unwrap();
        map.get(id).cloned()
    }
}