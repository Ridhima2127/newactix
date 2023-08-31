use crate::model::database;
use actix_web::error::ErrorInternalServerError;
use actix_web::{error, web, App, HttpResponse, HttpServer, ResponseError, Result};
use liquid::model::Value;
use liquid::ParserBuilder;
use liquid::{object, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Post {
    pub post_id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub category_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditPost {
    pub post_id: u64,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub num_of_posts: u64,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page_number: Option<i32>,
}

pub async fn pagination_index(
    page_number: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let page_number = *page_number;

    let posts = database::init_posts().await?;

    let categories = database::init_categories().await?;

    let mut category_post_counts = vec![0; categories.len()];
    for (i, category) in categories.iter().enumerate() {
        let count = posts
            .iter()
            .filter(|post| post.category_id == category.id)
            .count();
        category_post_counts[i] = count;
    }

    let limit = 3;

    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page_number - 1) * limit;

    let posts_array = posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
                "post_id": Value::scalar(post.post_id.to_string()),
                "title": Value::scalar(post.title),
                "description": Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let categories_with_count = categories
        .iter()
        .zip(category_post_counts.iter())
        .map(|(category, count)| {
            let mut category_map = object!({
                "id": Value::scalar(category.id.to_string()),
                "name": Value::scalar(category.name.clone()),
                "number": Value::scalar(count.to_string()),
            });
            Value::Object(category_map)
        })
        .collect::<Vec<Value>>();

    let pagination = object!({
        "prev": if page_number > 1 { Value::scalar(page_number - 1) } else { Value::Nil },
        "next": if page_number < total_pages { Value::scalar(page_number + 1) } else { Value::Nil },
        "current": Value::scalar(page_number),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts": Value::Array(posts_array),
        "categories": Value::Array(categories_with_count),
        "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/index.html").expect("Failed to read the file");

    let template_parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|err| {
            eprintln!("Failed to build parser: {}", err);
            actix_web::error::ErrorInternalServerError(format!("Failed to build parser: {}", err))
        })?;

    let template = template_parser.parse(&html_template).map_err(|err| {
        eprintln!("Failed to parse template: {}", err);
        actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
    })?;

    let output = template.render(&context).map_err(|err| {
        eprintln!("Failed to render the template: {}", err);
        actix_web::error::ErrorInternalServerError(format!(
            "Failed to render the template: {}",
            err
        ))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn index(query: web::Query<PaginationQuery>) -> Result<HttpResponse, actix_web::Error> {
    let posts = database::init_posts().await?;

    let categories = database::init_categories().await?;

    let mut category_post_counts = vec![0; categories.len()];
    for (i, category) in categories.iter().enumerate() {
        let count = posts
            .iter()
            .filter(|post| post.category_id == category.id)
            .count();
        category_post_counts[i] = count;
    }

    let limit = 3;

    let PaginationQuery { page_number } = query.into_inner();

    let page = match page_number {
        Some(value) => value,
        None => 1,
    };

    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page - 1) * limit;

    let posts_array = posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
                "post_id": Value::scalar(post.post_id.to_string()),
             "title" : Value::scalar(post.title),
            "description" : Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let categories_with_count = categories
        .iter()
        .zip(category_post_counts.iter())
        .map(|(category, count)| {
            let mut category_map = object!({
                "id": Value::scalar(category.id.to_string()),
                "name": Value::scalar(category.name.clone()),
                "number": Value::scalar(count.to_string()),
            });
            Value::Object(category_map)
        })
        .collect::<Vec<Value>>();

    let pagination = object!({
        "prev": if page > 1 { Value::scalar(page - 1) } else { Value::Nil },
        "next": if page < total_pages { Value::scalar(page + 1) } else { Value::Nil },
        "current": Value::scalar(page),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts":  Value::Array(posts_array),
        "categories": Value::Array(categories_with_count),
         "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/index.html").expect("Failed to read the file");

    let template_parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|err| {
            eprintln!("Failed to build parser: {}", err);
            actix_web::error::ErrorInternalServerError(format!("Failed to build parser: {}", err))
        })?;

    let template = template_parser.parse(&html_template).map_err(|err| {
        eprintln!("Failed to parse template: {}", err);
        actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
    })?;

    let output = template.render(&context).map_err(|err| {
        eprintln!("Failed to render the template: {}", err);
        actix_web::error::ErrorInternalServerError(format!(
            "Failed to render the template: {}",
            err
        ))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn specific_post(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();

    let single_post = database::get_specific_post().await?;

    if let Some(post) = single_post
        .into_iter()
        .find(|post| post.post_id == post_id as u64)
    {
        let mut context = object!({
            "post": Value::Object(object!({
                "title": Value::scalar(post.title),
                "description": Value::scalar(post.description),
            })),
        });

        let html_template =
            fs::read_to_string("templates/single_post.html").expect("Failed to read the file");

        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to build parser: {}",
                    err
                ))
            })?;

        let template = parser.parse(&html_template).map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
        })?;

        let output = template.render(&context).map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to render the template: {}",
                err
            ))
        })?;

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(output))
    } else {
        Err(actix_web::error::ErrorNotFound("Post not found"))
    }
}

pub async fn category_posts(path: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let (category_id, page_number) = path.into_inner();

    let posts = database::init_posts().await?;

    let category_posts: Vec<Post> = posts
        .into_iter()
        .filter(|post| post.category_id == category_id as u64)
        .collect();

    let limit = 3;
    let num_posts_in_category = category_posts.len();

    let total_pages = (num_posts_in_category as f64 / limit as f64).ceil() as i32;

    let offset = ((page_number - 1) * limit);

    let posts = category_posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
            "title": Value::scalar(post.title),
            "description": Value::scalar(post.description),
            "category": Value::scalar(post.category),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let mut context = object!({
    "posts": Value::Array(posts),
    "category_id": Value::scalar(category_id),
    "num_posts_in_category": Value::scalar(num_posts_in_category.to_string()),
    "pagination": object!({
    "prev": if page_number > 1 { Value::scalar(page_number - 1) } else { Value::Nil },
            "next": if page_number < total_pages { Value::scalar(page_number + 1) } else { Value::Nil },
    "current": Value::scalar(page_number),
    "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
        }),
        });

    let html_template =
        fs::read_to_string("templates/post_category.html").expect("Failed to read the file");

    let parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Failed to build parser: {}", err))
        })?;

    let template = parser.parse(&html_template).map_err(|err| {
        actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
    })?;

    let output = template.render(&context).map_err(|err| {
        actix_web::error::ErrorInternalServerError(format!(
            "Failed to render the template: {}",
            err
        ))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn update_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Failed to build parser: {}", err))
        })?;

    let template = parser.parse(&html_template).map_err(|err| {
        actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
    })?;

    let output = template.render(&context).map_err(|err| {
        actix_web::error::ErrorInternalServerError(format!(
            "Failed to render the template: {}",
            err
        ))
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}
