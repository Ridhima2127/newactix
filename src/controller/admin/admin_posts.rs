#![allow(unused)]
#![allow(deprecated)]

use crate::controller::posts::{Category, EditPost, PaginationQuery, Post};
use crate::model::database;
use crate::model::database::{get_posts, init_posts};
use actix_web::guard::Post;
use actix_web::http::header;
use actix_web::{web, App, HttpResponse};
use liquid::model::Value;
use liquid::object;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

pub struct AppState {
    pub database_post: Mutex<Vec<Post>>,
    pub database_category: Mutex<Vec<Category>>,
}

#[derive(serde::Deserialize)]
pub struct CategoryData {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    pub title: String,
    pub description: String,
}

#[derive(serde::Deserialize)]
pub struct PostUpdateForm {
    pub title: String,
    pub description: String,
}

pub async fn pagination_homepage(
    page_number: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let page_number = *page_number;

    let posts = get_posts(&data.get_ref().database_post).await?;

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

    let pagination = object!({
        "prev": if page_number > 1 { Value::scalar(page_number - 1) } else { Value::Nil },
        "next": if page_number < total_pages { Value::scalar(page_number + 1) } else { Value::Nil },
        "current": Value::scalar(page_number),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts": Value::Array(posts_array),
        "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/admin.html").expect("Failed to read the file");

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

pub async fn homepage(
    query: web::Query<PaginationQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = get_posts(&data.get_ref().database_post).await?;

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

    let html_template =
        fs::read_to_string("templates/admin.html").expect("Failed to read the file");

    let pagination = object!({
        "prev": if page > 1 { Value::scalar(page - 1) } else { Value::Nil },
        "next": if page < total_pages { Value::scalar(page + 1) } else { Value::Nil },
        "current": Value::scalar(page),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "posts":  Value::Array(posts_array),
         "pagination": Value::Object(pagination),
    });

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
pub async fn new_post() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_post.html").expect("Failed to read the file");

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

pub async fn create_post(
    data: web::Data<AppState>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_post.html").expect("Failed to read the file");

    let data = data.get_ref();

    let mut new_post = Post {
        title: form.title.clone(),
        description: form.description.clone(),
        ..Default::default()
    };

    let post_id = {
        let db_lock = data.database_post.lock().map_err(|err| {
            eprintln!("Failed to acquire database lock: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
        })?;
        db_lock.len() + 1
    };

    new_post.post_id = post_id as u64;

    if let Ok(mut inner) = data.database_post.lock() {
        inner.push(new_post);
    }

    let posts: Vec<Post> = data
        .database_post
        .lock()
        .map_err(|err| {
            eprintln!("Failed to acquire database lock: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
        })?
        .clone();

    let template_arc = Arc::new(Mutex::new(html_template));

    let response = {
        let template_arc_clone = template_arc.clone();
        let template = template_arc_clone
            .lock()
            .map_err(|err| {
                eprintln!("Failed to acquire template lock: {}", err);
                actix_web::error::ErrorInternalServerError("Failed to acquire template lock")
            })?
            .clone();

        let mut context = liquid::Object::new();

        context.insert(
            "posts".into(),
            liquid::model::Value::array(
                posts
                    .into_iter()
                    .map(|post| {
                        let mut post_map = liquid::Object::new();
                        post_map.insert("title".into(), liquid::model::Value::scalar(post.title));
                        post_map.insert(
                            "description".into(),
                            liquid::model::Value::scalar(post.description),
                        );
                        liquid::model::Value::Object(post_map)
                    })
                    .collect::<Vec<liquid::model::Value>>(),
            ),
        );

        let template_parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Failed to build parser: {}",
                    err
                ))
            })?;

        let template = template_parser.parse(&template).map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!("Failed to parse template: {}", err))
        })?;

        let output = template.render(&context).map_err(|err| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to render the template: {}",
                err
            ))
        })?;

        HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin"))
            .finish()
    };

    Ok(response)
}

pub async fn edit_post_html(
    post_id: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id_value: u64 = post_id.to_owned();

    let post_to_edit = {
        let db_lock = data.database_post.lock().map_err(|err| {
            eprintln!("Failed to acquire database lock: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
        })?;

        db_lock
            .iter()
            .find(|post| post.post_id == post_id_value)
            .cloned()
    };

    let post_to_edit = match post_to_edit {
        Some(post) => post,
        None => {
            return Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body("Post not found"));
        }
    };

    let mut context = liquid::Object::new();
    context.insert(
        "post".into(),
        liquid::model::Value::Object({
            let mut post_map = liquid::Object::new();
            post_map.insert(
                "post_id".into(),
                liquid::model::Value::scalar(post_to_edit.post_id.to_string()),
            );
            post_map.insert(
                "title".into(),
                liquid::model::Value::scalar(post_to_edit.title),
            );
            post_map.insert(
                "description".into(),
                liquid::model::Value::scalar(post_to_edit.description),
            );
            post_map
        }),
    );

    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

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

pub async fn edit_post(
    data: web::Data<AppState>,
    post_id: web::Path<u64>,
    form: web::Form<EditPost>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut inner_data = data.database_post.lock().map_err(|err| {
        eprintln!("Failed to acquire database lock: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
    })?;

    if let Some(post) = inner_data.iter_mut().find(|post| post.post_id == *post_id) {
        post.title = form.title.clone();
        post.description = form.description.clone();

        Ok(HttpResponse::Found().header("Location", "/admin").finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn delete_post_by_id(
    data: web::Data<AppState>,
    post_id: web::Path<u64>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut inner_data = data.database_post.lock().map_err(|err| {
        eprintln!("Failed to acquire database lock: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
    })?;

    if let Some(index) = inner_data.iter().position(|post| post.post_id == *post_id) {
        let deleted_post = inner_data.remove(index);
        Ok(HttpResponse::Found().header("Location", "/admin").finish())
    } else {
        Ok(HttpResponse::NotFound().json(None::<Post>))
    }
}
