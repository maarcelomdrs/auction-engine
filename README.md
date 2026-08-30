# auction-engine

Motor de leilões concorrentes de alta vazão e baixa latência desenvolvido em Rust utilizando Clean Architecture.

---

### Funcionalidades

- **Processamento Concorrente:** Tratamento simultâneo de requisições de lances com mitigação de *Race Conditions*.
- **Clean Architecture:** Separação estrita de responsabilidades entre camadas de Domínio, Aplicação e Infraestrutura.
- **Consistência de Estado:** Sincronização segura de memória para ordenação determinística de eventos em tempo real.
- **Testes de Carga:** Cobertura de cenários com múltiplos threads disputando o mesmo recurso.

---

### Instalação & Execução

Certifique-se de ter o **Rust** e o gerenciador `cargo` instalados:

```bash
# Clone o repositório
git clone [https://github.com/maarcelomdrs/auction-engine.git](https://github.com/maarcelomdrs/auction-engine.git)
cd auction-engine

# Executar a suíte de testes de concorrência
cargo test

# Compilar e rodar a aplicação em modo otimizado
cargo run --release
```

---

### Arquitetura

O sistema é desenhado desacoplando a lógica de negócio principal de adaptadores externos (banco de dados, I/O e APIs). O controle transacional é mantido em memória através de canais assíncronos e primitivas de sincronização do Rust, garantindo previsibilidade e performance sob alta concorrência.

---

### Licença

Distribuído sob a licença MIT.
