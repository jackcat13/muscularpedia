use crate::i18n::{i18n, Lang};
use crate::middlewares::lang_middleware::lang_middleware;
use crate::templates::home::Category::{Arms, Back, Core, Legs};
use crate::templates::home::{Category, Exercise, Exercises};
use askama::Template;
use axum::{Router, middleware, response::Html, routing::get};
use std::collections::HashMap;
use templates::home::HomeTemplate;
use tokio::task_local;
use tower_http::services::ServeDir;

mod i18n;
mod middlewares;
mod templates;

task_local! {
    pub static CURRENT_LANG: Lang;
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(lang_middleware));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let template = HomeTemplate {
        exercises: init_exercises(),
    };
    Html(template.render().expect("Failed to render home template"))
}

fn init_exercises() -> std::collections::HashMap<Category, Exercises> {
    let mut map = HashMap::new();
    map.insert(
        Legs,
        Exercises {
            items: vec![Exercise {
                name: "Squat".to_string(),
                description: i18n("Exercise-Squat-Description").to_string(),
                image: "todo".to_string(),
                categories: vec![Legs, Core, Back],
            }],
        },
    );
    map.insert(
        Arms,
        Exercises {
            items: vec![Exercise {
                name: "Curl biceps".to_string(),
                description: i18n("Exercise-Curl-Biceps-Description").to_string(),
                image: "todo".to_string(),
                categories: vec![Arms],
            }],
        },
    );
    map
}
