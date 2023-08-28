use crate::controller::posts::Category;
use crate::model::database;
use actix_web::{error, web, App, HttpResponse, HttpServer, Result};
use liquid::model::Value;
use liquid::object;
use std::fs;

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
