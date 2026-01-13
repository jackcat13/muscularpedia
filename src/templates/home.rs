use crate::i18n::i18n;
use askama::{Template, filters::HtmlSafe};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{collections::HashMap, fmt::Display};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub exercises: HashMap<Category, Exercises>,
}

impl HomeTemplate {
    pub fn sorted_categories(&self) -> Vec<Category> {
        let mut categories: Vec<Category> = self.exercises.keys().cloned().collect();
        categories.sort_by(|a, b| a.cmp(b));
        categories
    }
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

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[derive(Clone)]
pub enum Category {
    Shoulder,
    Chest,
    Back,
    Arms,
    Core,
    Legs,
}

impl HtmlSafe for Category {}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Category::Shoulder => i18n("Category-Shoulder"),
            Category::Chest => i18n("Category-Chest"),
            Category::Back => i18n("Category-Back"),
            Category::Arms => i18n("Category-Arms"),
            Category::Core => i18n("Category-Core"),
            Category::Legs => i18n("Category-Legs"),
        };
        write!(f, "{}", s)
    }
}
