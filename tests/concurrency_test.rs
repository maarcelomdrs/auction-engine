use std::sync::Arc;
use std::thread;
use std::time::Duration;

use auction_engine::domain::auction::Auction;
use auction_engine::domain::bid::Bid;
use auction_engine::domain::errors::DomainError;
use auction_engine::domain::repository::AuctionRepository;
use auction_engine::infrastructure::persistence::in_memory_auction_repository::InMemoryAuctionRepository;

#[test]
fn should_prevent_stale_data_overwrite_with_optimistic_lock() {
    let repo = InMemoryAuctionRepository::new();
    let auction_id = "occ_auction_01".to_string();

    let auction = Auction::new(auction_id.clone(), "seller_01".to_string(), 100.0);
    repo.save(auction).expect("Falha ao salvar");

    let shared_repo = Arc::new(repo);

    // Thread A: Lê a versão 1, demora para processar
    let repo_a = Arc::clone(&shared_repo);
    let id_a = auction_id.clone();
    let handle_a = thread::spawn(move || {
        let mut auction = repo_a.find_by_id(&id_a).unwrap();
        thread::sleep(Duration::from_millis(50));

        let bid = Bid::new("user_a".to_string(), 110.0).unwrap();
        auction.place_bid(bid).unwrap();

        // Esta tentativa deve falhar com OptimisticLockError pois a Thread B gravou antes
        repo_a.save(auction)
    });

    // Thread B: Grava primeiro (versão 2) com lance de R$ 200
    let repo_b = Arc::clone(&shared_repo);
    let id_b = auction_id.clone();
    let handle_b = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));

        let mut auction = repo_b.find_by_id(&id_b).unwrap();
        let bid = Bid::new("user_b".to_string(), 200.0).unwrap();
        auction.place_bid(bid).unwrap();

        repo_b.save(auction)
    });

    let result_b = handle_b.join().unwrap();
    let result_a = handle_a.join().unwrap();

    // A gravação da Thread B tem sucesso
    assert!(result_b.is_ok());

    // A gravação desatualizada da Thread A é rejeitada
    assert_eq!(result_a.unwrap_err(), DomainError::OptimisticLockError);

    // O estado final permanece protegido em R$ 200
    let final_auction = shared_repo.find_by_id(&auction_id).unwrap();
    assert_eq!(final_auction.current_highest_bid.unwrap().amount, 200.0);
    assert_eq!(final_auction.version, 2);
}