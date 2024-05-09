use std::ops::Add;

use axum::{response::Html, routing::{get, post}, Form, Json, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(handler))
        .route("/component/input", get(input_handler))
        .route("/add-task", post(add_task))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
    <!DOCTYPE html>
    <html lang="ja">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <script src="static/htmx.min.js"></script>
        <title>Document</title>
    </head>
    <body>
        <div hx-get="/component/input" hx-trigger="revealed" hx-swap="outerHTML"></div>
        
    </body>
    </html>
    "#,
    )
}


async fn input_handler() -> Html<&'static str> {
    Html(
        r##"
        <div>
            <form hx-post="/add-task" hx-target="#result" hx-swap="afterbegin">
                <input type="text" name="task">
                <button type="submit">送信</button>
            </form>
            <div id="result"></div>
        </div>
    "##,
    )
}

#[derive(Deserialize)]
struct AddTask {
    task: String,
}
async fn add_task(
    Form(task): Form<AddTask>,
) -> Html<String> {
    let html = format!("<p>{}</p>", task.task);
    Html(html)
}
