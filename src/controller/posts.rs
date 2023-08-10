use actix_web::{error, web, App, HttpResponse, HttpServer, Result, ResponseError};
use liquid::model::Value;
use liquid::{object, Parser};
use liquid::ParserBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use actix_web::error::ErrorInternalServerError;
use crate::model::database;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub category: String,
    pub category_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: u64,
    pub name: String,
}

pub async fn index() -> Result<HttpResponse, actix_web::Error> {



    let posts = database::get_posts().await?;

    let categories = database::get_categories().await?;

    let posts_array = posts
        .into_iter()
        .map(|post| {
            let mut post_map = object!({
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
            });
            Value::Object(category_map)
        })
        .collect::<Vec<Value>>();

    let mut context = object!({
        "posts":  Value::Array(posts_array),
        "categories": Value::Array(categories_array),
    });


    let html_template = fs::read_to_string("templates/index.html").expect("Failed to read the file");

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




pub async fn specific_post() -> Result<HttpResponse, actix_web::Error> {

    let single_post = database::get_specific_post().await?;

    let single_post_array = single_post
        .into_iter()
        .map(|post| {
            let mut post_map = object!({
             "title" : Value::scalar(post.title),
            "description" : Value::scalar(post.description),
            });
            Value::Object(post_map)
        })
        .collect::<Vec<Value>>();

    let mut context = object!({
        "posts":  Value::Array(single_post_array),
    });


    let html_template = fs::read_to_string("templates/single_post.html").expect("Failed to read the file");

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

pub async fn category_posts( path: web::Path<i32>) -> Result<HttpResponse> {

    let category_id = path.into_inner();

    let posts = database::get_posts().await?;

    let posts: Vec<Post> =  posts
            .into_iter()
            .filter(|post| post.category_id == category_id  as u64)
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


    let html_template = fs::read_to_string("templates/post_category.html").expect("Failed to read the file");

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
