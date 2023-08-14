use crate::controller::posts::Category;
use crate::model::database;
use actix_web::{error, web, App, HttpResponse, HttpServer, Result};
use liquid::model::Value;
use liquid::object;
use std::fs;

pub async fn categories_display() -> Result<HttpResponse, actix_web::Error> {
    let categories = database::get_categories().await?;

    let categories_array = categories
        .into_iter()
        .map(|category| {
            let mut category_map = object!({
                "name": Value::scalar(category.name),
            });

            Value::Object(category_map)
        })
        .collect::<Vec<Value>>();

    let mut context = object!({
        "categories": Value::Array(categories_array),
    });

    let html_template =
        fs::read_to_string("../../templates/cat.html").expect("Failed to read the file");

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

pub async fn new_category() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database render the 'new_category.html.liquid' template.

    /*
        todo!()
        let template_str = include_str!("templates/new_category.html.liquid");

        let template = Parser::parse(template_str)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

        let html = template.render(&context)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}

pub async fn create_category() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database and render the 'create_category.html.liquid' template.

    /*
    todo!()
    let template_str = include_str!("templates/create_category.html.liquid");

    let template = liquid::Parser::parse(template_str)
        .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

    let html = template.render(&liquid::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */

    // Once the template rendering is implemented, return the HttpResponse.
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}

pub async fn edit_category() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database and render the 'edit_category.html.liquid' template.

    /*
        todo!()
        let template_str = include_str!("templates/edit_category.html.liquid");

        let template = liquid::Parser::parse(template_str)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

        let html = template.render(&liquid::Context::new())
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}

pub async fn update_category() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database and to render the 'edit_category.html.liquid' template.

    /*
        todo!()
        let template_str = include_str!("templates/edit_category.html.liquid");

        let template = liquid::Parser::parse(template_str)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

        let html = template.render(&liquid::Context::new())
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}

pub async fn delete_category() -> Result<HttpResponse, actix_web::Error> {
    // TODO: Implement the logic to connect to database and render the 'delete_category.html.liquid' template.

    /*
        todo!()
        let template_str = include_str!("templates/delete_category.html.liquid");

        let template = liquid::Parser::parse(template_str)
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to parse template: {}", e)))?;

        let html = template.render(&liquid::Context::new())
            .map_err(|e| error::ErrorInternalServerError(format!("Failed to render template: {}", e)))?;
    */
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(()))
}
