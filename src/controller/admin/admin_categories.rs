#![allow(deprecated)]

use crate::controller::admin::admin_posts::{AppState, CategoryData};
use crate::controller::posts::{Category, EditCategory, PaginationQuery};
use crate::model::database;
use crate::model::database::get_categories;
use actix_web::http::header;
use actix_web::{web, HttpResponse};
use liquid::model::Value;
use liquid::object;
use std::fs;
use std::process::id;
use std::sync::{Arc, Mutex};

pub async fn admin_category_pagination(
    page_number: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let page_number = *page_number;

    let categories = get_categories(&data.get_ref().database_category).await?;

    let limit = 3;

    let total_pages = (categories.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page_number - 1) * limit;

    let categories_array = categories
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|category| {
            let mut category_map = object!({
                  "id": Value::scalar(category.id.to_string()),
                "name": Value::scalar(category.name),
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
        "categories": Value::Array(categories_array),
        "pagination": Value::Object(pagination),
    });

    let html_template =
        fs::read_to_string("templates/admin_categories.html").expect("Failed to read the file");

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

pub async fn admin_category(
    query: web::Query<PaginationQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let categories = get_categories(&data.get_ref().database_category).await?;

    let limit = 3;

    let PaginationQuery { page_number } = query.into_inner();

    let page = match page_number {
        Some(value) => value,
        None => 1,
    };
    let total_pages = (categories.len() as f64 / limit as f64).ceil() as i32;

    let offset = (page - 1) * limit;

    let categories_array = categories
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|category| {
            let mut category_map = object!({
                  "id": Value::scalar(category.id.to_string()),
                "name": Value::scalar(category.name),
            });

            Value::Object(category_map)
        })
        .collect::<Vec<Value>>();

    let html_template =
        fs::read_to_string("templates/admin_categories.html").expect("Failed to read the file");

    let pagination = object!({
        "prev": if page > 1 { Value::scalar(page - 1) } else { Value::Nil },
        "next": if page < total_pages { Value::scalar(page + 1) } else { Value::Nil },
        "current": Value::scalar(page),
        "pages": (1..=total_pages).map(Value::scalar).collect::<Vec<Value>>(),
    });

    let mut context = object!({
        "categories": Value::Array(categories_array),
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

pub async fn new_category() -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_category.html").expect("Failed to read the file");

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

pub async fn create_category(
    data: web::Data<AppState>,
    form: web::Form<CategoryData>,
) -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_category.html").expect("Failed to read the file");

    let data = data.get_ref();

    let mut new_category = Category {
        name: form.name.clone(),
        ..Category::default()
    };

    let id = {
        let db_lock = data.database_category.lock().map_err(|err| {
            eprintln!("Failed to acquire database lock: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
        })?;
        db_lock.len() + 1
    };

    new_category.id = id as u64;

    if let Ok(mut inner) = data.database_category.lock() {
        inner.push(new_category);
    }

    let categories: Vec<Category> = data
        .database_category
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
            "categories".into(),
            liquid::model::Value::array(
                categories
                    .into_iter()
                    .map(|category| {
                        let mut category_map = liquid::Object::new();
                        category_map
                            .insert("name".into(), liquid::model::Value::scalar(category.name));
                        liquid::model::Value::Object(category_map)
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
            .append_header((header::LOCATION, "/admin/category"))
            .finish()
    };

    Ok(response)
}

pub async fn edit_category_html(
    id: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let category_id_value: u64 = id.to_owned();

    let category_to_edit = {
        let db_lock = data.database_category.lock().map_err(|err| {
            eprintln!("Failed to acquire database lock: {}", err);
            actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
        })?;

        db_lock
            .iter()
            .find(|category| category.id == category_id_value)
            .cloned()
    };

    let category_to_edit = match category_to_edit {
        Some(category) => category,
        None => {
            return Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body("Post not found"));
        }
    };

    let mut context = liquid::Object::new();
    context.insert(
        "category".into(),
        liquid::model::Value::Object({
            let mut post_map = liquid::Object::new();
            post_map.insert(
                "id".into(),
                liquid::model::Value::scalar(category_to_edit.id.to_string()),
            );
            post_map.insert(
                "name".into(),
                liquid::model::Value::scalar(category_to_edit.name),
            );
            post_map
        }),
    );

    let html_template =
        fs::read_to_string("templates/edit_category.html").expect("Failed to read the file");

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

pub async fn edit_category(
    data: web::Data<AppState>,
    id: web::Path<u64>,
    form: web::Form<EditCategory>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut inner_data = data.database_category.lock().map_err(|err| {
        eprintln!("Failed to acquire database lock: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
    })?;

    if let Some(category) = inner_data.iter_mut().find(|category| category.id == *id) {
        category.name = form.name.clone();

        Ok(HttpResponse::Found()
            .header("Location", "/admin/category")
            .finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn delete_category_by_id(
    data: web::Data<AppState>,
    id: web::Path<u64>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut inner_data = data.database_category.lock().map_err(|err| {
        eprintln!("Failed to acquire database lock: {}", err);
        actix_web::error::ErrorInternalServerError("Failed to acquire database lock")
    })?;

    if let Some(index) = inner_data.iter().position(|category| category.id == *id) {
        let deleted_category = inner_data.remove(index);
        Ok(HttpResponse::Found()
            .header("Location", "/admin/category")
            .finish())
    } else {
        Ok(HttpResponse::NotFound().json(None::<Category>))
    }
}
