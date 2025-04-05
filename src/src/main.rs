use axum::{
    routing::get,
    extract::Query,
    response::Html,
    Router,
};
use std::{collections::HashMap, net::SocketAddr};
use axum::serve; // Используем serve

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Сервер запущен на http://{}", addr);

    // Создаем TCP слушателя
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Запуск сервера с использованием функции `serve`
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let default = "bitcoin".to_string();
    let query = params.get("q").unwrap_or(&default);

    Html(format!("<h1>Query: {}</h1>", query))
}
