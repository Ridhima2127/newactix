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


#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Post {
    pub post_id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub category_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub num_of_posts: u64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn index(page_number: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {

    let posts = database::get_posts().await?;

    let categories = database::get_categories().await?;

    let limit = 3;

    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let page = page_number.into_inner();



    let offset = (page-1)*limit;

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

    let categories_array = categories
        .into_iter()
        .map(|category| {
            let mut category_map = object!({
             "id" : Value::scalar(category.id.to_string()),
            "name" : Value::scalar(category.name),
                "number": Value::scalar(category.num_of_posts.to_string()),
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
        "categories": Value::Array(categories_array),
         "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/index.html").expect("Failed to read the file");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

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

        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&html_template)
            .expect("Failed to parse template");

        let output = template
            .render(&context)
            .expect("Failed to render the template");

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(output))
    } else {
        Err(actix_web::error::ErrorNotFound("Post not found"))
    }
}

pub async fn category_posts(path: web::Path<i32>) -> Result<HttpResponse> {
    let category_id = path.into_inner();

    let posts = database::get_posts().await?;

    let posts: Vec<Post> = posts
        .into_iter()
        .filter(|post| post.category_id == category_id as u64)
        .collect();

    let posts_array = posts
        .into_iter()
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
        "posts": Value::Array(posts_array),
        "category_id": Value::scalar(category_id),
    });

    let html_template =
        fs::read_to_string("templates/post_category.html").expect("Failed to read the file");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn paginated_posts(page_number: web::Path<i32>)->Result<HttpResponse>{


    let page = page_number.into_inner();

    let limit = 3;

    let offset = (page-1)*limit;


    let posts = vec![
        Post {
            post_id: 1,
            title: "Post 1".to_string(),
            description: "Description of Post 1".to_string(),
            category: "Travel".to_string(),
            category_id: 0,
        },
        Post {
            post_id: 2,
            title: "Post 2".to_string(),
            description: "Description of Post 2".to_string(),
            category: "Life".to_string(),
            category_id: 0,
        },
        Post {
            post_id: 3,
            title: "Post 3".to_string(),
            description: "Description of Post 3".to_string(),
            category: "Life".to_string(),
            category_id: 0,
        },
        Post {
            post_id: 4,
            title: "Post 4".to_string(),
            description: "Description of Post 4".to_string(),
            category: "Home".to_string(),
            category_id: 0,
        },
        Post {
            post_id: 5,
            title: "Post 5".to_string(),
            description: "Description of Post 5".to_string(),
            category: "Home".to_string(),
            category_id: 0,
        },

        Post {
            post_id: 6,
            title: "Post 6".to_string(),
            description: "Description of Post 5".to_string(),
            category: "Home".to_string(),
            category_id: 0,
        },
        Post {
            post_id: 7,
            title: "Post 7".to_string(),
            description: "Description of Post 5".to_string(),
            category: "Home".to_string(),
            category_id: 0,
        },


    ];


    let total_pages = (posts.len() as f64 / limit as f64).ceil() as i32;

    let paginated_posts = posts
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|post| {
            let mut post_map = object!({
                "title": Value::scalar(post.title),
                "description": Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();



    let pagination = object!({
        "prev": if page > 1 { Value::scalar(page - 1) } else { Value::Nil },
        "next": if page < total_pages { Value::scalar(page + 1) } else { Value::Nil },
        "current": Value::scalar(page),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let context = object!({
        "posts": Value::Array(paginated_posts),
        "pagination": Value::Object(pagination),
    });


    let html_template =
        fs::read_to_string("templates/paginated_posts.html").expect("Failed to read the file");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}



pub async fn new_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_post.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn create_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_post.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn edit_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn update_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

    let context = liquid::Object::new();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&html_template)
        .expect("Failed to parse template");

    let output = template
        .render(&context)
        .expect("Failed to render the template");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(output))
}

pub async fn delete_post() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database and render the 'delete_post.html.liquid' template.

    /*
      todo!()
      let template_str = include_str!("templates/delete_post.html.liquid");

        let template = liquid::Parser::parse(template_str)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

        let html = template.render(&liquid::Context::new())
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}
