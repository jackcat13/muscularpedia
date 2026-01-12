use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use std::fmt::Formatter;
use askama::{Template, filters::HtmlSafe};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub exercises: HashMap<Category, Exercises>,
}

#[derive(Deserialize)]
pub struct Exercises {
    pub items: Vec<Exercise>,
}

impl HtmlSafe for Exercises {}

impl Display for Exercises {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
                <div>{}</div>
            "#,
            as_exercises_string(&self.items)
        )
    }
}

fn as_exercises_string(exercises: &Vec<Exercise>) -> String {
    let mut s = String::new();
    for exercise in exercises {
        s += &exercise.to_string();
    }
    s
}

#[derive(serde::Deserialize)]
pub struct Exercise {
    pub name: String,
    pub description: String,
    pub image: String,
    pub categories: Vec<Category>,
}

impl HtmlSafe for Exercise {}

impl Display for Exercise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
                <div>{}</div>
                <div>{}</div>
                <div>{}</div>
                <div>{}</div>
            "#,
            self.name,
            self.description,
            self.image,
            as_categories_string(&self.categories)
        )
    }
}

fn as_categories_string(categories: &Vec<Category>) -> String {
    let mut s = String::new();
    for category in categories {
        s += &category.to_string();
        s += &", ";
    }
    s
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, strum::Display)]
pub enum Category {
    Shoulder,
    Chest,
    Back,
    Arms,
    Core,
    Legs,
}

impl HtmlSafe for Category {}