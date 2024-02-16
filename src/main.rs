use std::collections::HashMap;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use time::OffsetDateTime;

#[derive(Default)]
struct Account {
    balance: i64,
    limit: i64,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn with_limit(limit: i64) -> Self {
        Account {
            limit,
            ..Default::default()
        }
    }
}

enum TransactionType {
    Credit,
    Debit,
}

struct Transaction {
    value: i64,
    kind: TransactionType,
    description: String,
    created_at: OffsetDateTime,
}

#[tokio::main]
async fn main() {
    let account = HashMap::<u8, Account>::from_iter([
        (1, Account::with_limit(100_000)),
        (2, Account::with_limit(80_000)),
        (3, Account::with_limit(100_000)),
        (4, Account::with_limit(80_000)),
        (5, Account::with_limit(100_000)),
    ]);

    let app = Router::new()
        .route("/clientes/:id/transacoes", post(create_transaction))
        .route("/clientes/:id/extrato", get(view_account));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_transaction() -> impl IntoResponse {
    "Transação criada"
}

async fn view_account() -> impl IntoResponse {
    "Extrato"
}
