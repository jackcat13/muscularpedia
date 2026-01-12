use std::collections::HashMap;
use askama::Template;
use axum::{Router, response::Html, routing::get};
use templates::home::HomeTemplate;

use crate::templates::home::{Category, Exercise, Exercises};
use crate::templates::home::Category::{Back, Core, Legs};

mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));
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
    map.insert(Legs, Exercises {
        items: vec![
            Exercise {
                name: "Squat".to_string(),
                description: "todo".to_string(),
                image: "todo".to_string(),
                categories: vec![Legs, Core, Back],
            }
        ]
    });
    map
}
