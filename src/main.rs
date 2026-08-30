use auction_engine::application::place_bid::{execute_place_bid, PlaceBidCommand};
use auction_engine::domain::auction::Auction;
use auction_engine::domain::repository::AuctionRepository;
use auction_engine::infrastructure::persistence::in_memory_auction_repository::InMemoryAuctionRepository;

fn main() {
    println!("--- Iniciando Simulação com Clean Architecture ---");

    let repo = InMemoryAuctionRepository::new();

    let auction_id = "item_notebook_01".to_string();
    let auction = Auction::new(
        auction_id.clone(),
        "vendedor_01".to_string(),
        1000.0,
    );

    repo.save(auction).expect("Falha ao salvar leilão inicial");
    println!("Leilão salvo no repositório! ID: {}", auction_id);

    let command = PlaceBidCommand {
        auction_id: auction_id.clone(),
        bidder_id: "comprador_ana".to_string(),
        amount: 1200.0,
    };

    match execute_place_bid(command, &repo) {
        Ok(_) => println!("-> Sucesso: Lance aceito e persistido via Caso de Uso!"),
        Err(e) => println!("-> Erro ao processar lance: {}", e),
    }
}