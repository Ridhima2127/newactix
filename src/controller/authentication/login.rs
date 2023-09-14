use crate::model::users::get_users;
use actix_web::http::header;
use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Result};
use actix_web::{web, FromRequest, HttpRequest, Responder};
use liquid::model::Value;
use liquid::ParserBuilder;
use liquid::{Object, ObjectView};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/*pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}
*/
pub async fn get_user_by_username(username: &str) -> Option<User> {
    match get_users().await {
        Ok(users) => users.iter().find(|user| user.username == username).cloned(),
        Err(_) => {
            eprintln!("Error retrieving users");
            None
        }
    }
}
pub async fn login_user(form: web::Form<User>) -> Result<HttpResponse, actix_web::Error> {
    let username = form.username.clone();
    let password = form.password.clone();

    if let Some(user) = get_user_by_username(&username).await {
        if user.password == password {
            Ok(HttpResponse::SeeOther()
                .append_header((header::LOCATION, "/admin"))
                .finish())
        } else {
            Ok(HttpResponse::Unauthorized().body("Incorrect password"))
        }
    } else {
        Ok(HttpResponse::NotFound().body("User not found"))
    }
}

pub async fn login() -> Result<HttpResponse> {
    let html_template =
        fs::read_to_string("templates/login.html").map_err(ErrorInternalServerError)?;

    let context = liquid::Object::new();

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

/*#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );
    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}*/