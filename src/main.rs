mod application;
mod domain;

use application::place_bid::{PlaceBidCommand, PlaceBidUseCase};
use domain::auction::Auction;

fn main() {
    println!("--- Iniciando Simulação do Motor de Leilão ---\n");

    // 1. Cria um leilão (Dono: "carlos_123", Incremento mínimo: 50.0)
    let mut auction = Auction::new(
        "item_notebook_01".to_string(),
        "carlos_123".to_string(),
        50.0,
    );

    println!("Leilão criado! ID: {}, Incremento Mínimo: 50.0", auction.id);

    // 2. Tenta dar um lance válido (Usuário "ana_456" dá 100.0)
    let cmd1 = PlaceBidCommand {
        auction_id: auction.id.clone(),
        bidder_id: "ana_456".to_string(),
        amount: 100.0,
    };

    match PlaceBidUseCase::execute(&mut auction, cmd1) {
        Ok(_) => println!("-> Sucesso: Lance de Ana aceito!"),
        Err(e) => println!("-> Erro: {}", e),
    }

    // 3. Tenta dar um lance muito baixo (Usuário "joao_789" dá 120.0, mas o mínimo é 150.0)
    let cmd2 = PlaceBidCommand {
        auction_id: auction.id.clone(),
        bidder_id: "joao_789".to_string(),
        amount: 120.0,
    };

    match PlaceBidUseCase::execute(&mut auction, cmd2) {
        Ok(_) => println!("-> Sucesso: Lance de João aceito!"),
        Err(e) => println!("-> Erro esperado: {}", e),
    }

    // 4. NOVO TESTE: O próprio dono ("carlos_123") tenta dar um lance no seu item
    let cmd3 = PlaceBidCommand {
        auction_id: auction.id.clone(),
        bidder_id: "carlos_123".to_string(),
        amount: 200.0,
    };

    match PlaceBidUseCase::execute(&mut auction, cmd3) {
        Ok(_) => println!("-> Sucesso: Lance do dono aceito!"),
        Err(e) => println!("-> Erro esperado (Dono não pode dar lance): {}", e),
    }
}