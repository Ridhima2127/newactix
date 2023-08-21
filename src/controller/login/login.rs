use std::fs;
use actix_web::{web, Responder, HttpRequest, FromRequest};
use actix_web::{HttpResponse, Result, error::ErrorInternalServerError, get};
use liquid::model::Value;
use liquid::{Object, ObjectView};
use liquid::ParserBuilder;
use serde::Serialize;
use serde::Deserialize;
use crate::model::users::get_users;


#[derive(Deserialize)]
#[derive(Clone)]
pub struct User{
    pub(crate) username: String,
    pub(crate) password: String,
}


pub async fn get_user_by_username(username: &str) -> Option<User> {
        let users = get_users().await.unwrap_or_else(|_| vec![]);

        users.iter().find(|user| user.username == username).cloned()
    }


pub async fn login_user(form: web::Form<User>) -> Result<HttpResponse, actix_web::Error> {
    let username = form.username.clone();
    let password = form.password.clone();


    if let Some(user) = get_user_by_username(&username).await {
        if user.password == password {
            Ok(HttpResponse::Ok().body(format!("Welcome, {}!", username)))
        } else {
            Ok(HttpResponse::Unauthorized().body("Incorrect password"))
        }
    } else {
        Ok(HttpResponse::NotFound().body("User not found"))
    }
}



pub async fn login() -> Result<HttpResponse> {

    let html_template =
        fs::read_to_string("templates/login.html")
            .map_err(|e| ErrorInternalServerError(e))?;

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

