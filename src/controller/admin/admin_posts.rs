/*use std::fs;
use actix_web::{HttpResponse, web};
use liquid::model::Value;
use liquid::object;
use crate::controller::posts::PaginationQuery;
use crate::model::database;

pub async fn admin_posts() -> Result<HttpResponse, actix_web::Error> {

        let posts = database::get_posts().await?;

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


        let posts_array = posts
            .into_iter()
            .map(|post| {
                let mut post_map = object!({
                     "post_id": Value::scalar(post.post_id.to_string()),
             "title" : Value::scalar(post.title),
            "description" : Value::scalar(post.description),
            });
                Value::Object(post_map)
            })
            .collect::<Vec<Value>>();

        let mut context = object!({
        "posts":  Value::Array(posts_array),
              "categories": Value::Array(categories_with_count),
    });


        let html_template = fs::read_to_string("templates/admin.html").expect("Failed to read the file");

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
    }*/