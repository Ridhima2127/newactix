use crate::controller::admin::admin_posts::{AppState, CategoryData};
use crate::controller::posts::{Category, PaginationQuery};
use crate::model::database;
use crate::model::database::get_categories;
use actix_web::http::header;
use actix_web::{web, HttpResponse};
use liquid::model::Value;
use liquid::object;
use std::fs;
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

pub async fn admin_category(
    query: web::Query<PaginationQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let categories = get_categories(&data.get_ref().database_category).await?;

    let limit = 3;

    let PaginationQuery { page_number } = query.into_inner();

    let page = page_number.unwrap_or(1);

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
    let html_template =
        fs::read_to_string("templates/new_category.html").expect("Failed to read the file");

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

pub async fn create_category(
    data: web::Data<AppState>,
    form: web::Form<CategoryData>,
) -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_category.html").expect("Failed to read the file");

    let data = data.get_ref();

    let mut new_category = Category::default();
    new_category.name = form.name.clone();

    let id = {
        let db_lock = data.database_category.lock().unwrap();
        db_lock.len() + 1
    };
    new_category.id = id as u64;

    match data.database_category.lock() {
        Ok(mut inner) => {
            inner.push(new_category);
        }
        _ => {}
    }

    let categories: Vec<Category> = data.database_category.lock().unwrap().clone();

    let template_arc = Arc::new(Mutex::new(html_template));

    let response = {
        let template_arc_clone = template_arc.clone();
        let template = template_arc_clone.lock().unwrap().clone();

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

        let template_parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let template = template_parser
            .parse(&template)
            .expect("Failed to parse template");

        let output = template
            .render(&context)
            .expect("Failed to render the template");

        HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin/category"))
            .finish()
    };

    Ok(response)
}
