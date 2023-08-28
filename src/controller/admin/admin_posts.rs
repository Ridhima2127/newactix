use crate::controller::posts::{Category, PaginationQuery, Post};
use crate::model::database;
use crate::model::database::{get_posts, init_posts};
use actix_web::http::header;
use actix_web::{web, HttpResponse};
use liquid::model::Value;
use liquid::object;
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use actix_web::guard::Post;

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

pub async fn homepage(
    query: web::Query<PaginationQuery>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = get_posts(&data.get_ref().database_post).await?;

    let limit = 3;

    let PaginationQuery { page_number } = query.into_inner();

    let page = page_number.unwrap_or(1);

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

pub async fn create_post(
    data: web::Data<AppState>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, actix_web::Error> {
    let html_template =
        fs::read_to_string("templates/new_post.html").expect("Failed to read the file");

    let data = data.get_ref();

    let mut new_post = Post::default();
    new_post.title = form.title.clone();
    new_post.description = form.description.clone();

    let post_id = {
        let db_lock = data.database_post.lock().unwrap();
        db_lock.len() + 1
    };
    new_post.post_id = post_id as u64;

    match data.database_post.lock() {
        Ok(mut inner) => {
            inner.push(new_post);
        }
        _ => {}
    }

    let posts: Vec<Post> = data.database_post.lock().unwrap().clone();

    let template_arc = Arc::new(Mutex::new(html_template));

    let response = {
        let template_arc_clone = template_arc.clone();
        let template = template_arc_clone.lock().unwrap().clone();

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

        let template_parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let template = template_parser
            .parse(&template)
            .expect("Failed to parse template");

        let output = template
            .render(&context)
            .expect("Failed to render the template");

        HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin"))
            .finish()
    };

    Ok(response)
}


pub async fn edit_post(
    data: web::Data<AppState>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, actix_web::Error> {

    let data = data.get_ref();

    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

    /*   let post_id_to_edit = form.post_id;

    let mut posts_lock = data.database_post.lock().unwrap();
    if let Some(post_to_edit) = posts_lock.iter_mut().find(|post| post.post_id == post_id_to_edit) {
        post_to_edit.title = form.title.clone();
        post_to_edit.description = form.description.clone();
    }*/

    let mut new_post = Post::default();
    new_post.title = form.title.clone();
    new_post.description = form.description.clone();

    let post_id = {
        let db_lock = data.database_post.lock().unwrap();
        db_lock.len() + 1
    };
    new_post.post_id = post_id as u64;

    match data.database_post.lock() {
        Ok(mut inner) => {
            inner.push(new_post);
        }
        _ => {}
    }

    let posts: Vec<Post> = data.database_post.lock().unwrap().clone();

    let template_arc = Arc::new(Mutex::new(html_template));

    let response = {
        let template_arc_clone = template_arc.clone();
        let template = template_arc_clone.lock().unwrap().clone();

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

        let template_parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let template = template_parser
            .parse(&template)
            .expect("Failed to parse template");

        let output = template
            .render(&context)
            .expect("Failed to render the template");

        HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/admin"))
            .finish()
    };

    Ok(response)
}

pub async fn edit_post_form(
    data: web::Data<AppState>,
    post_id: web::Path<u64>, // Assuming the post ID is passed in the URL
) -> Result<HttpResponse, actix_web::Error> {
    let data = data.get_ref();
    let post_id = *post_id;

    let html_template =
        fs::read_to_string("templates/edit_post.html").expect("Failed to read the file");

    let posts_lock = data.database_post.lock().unwrap();

    if let Some(post_to_edit) = posts_lock.iter().find(|post| post.post_id == post_id) {
        let mut context = liquid::Object::new();
        context.insert(
            "post".into(),
            liquid::model::Value::Object({
                let mut post_map = liquid::Object::new();
                post_map.insert(
                    "title".into(),
                    liquid::model::Value::scalar(post_to_edit.title.clone()),
                );
                post_map.insert(
                    "description".into(),
                    liquid::model::Value::scalar(post_to_edit.description.clone()),
                );
                post_map
            }),
        );

        let template_parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let template = template_parser
            .parse(&html_template)
            .expect("Failed to parse template");
        let output = template
            .render(&context)
            .expect("Failed to render the template");

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(output))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}



pub(crate) async fn delete_post_by_id(
    data: web::Data<Mutex<Vec<Post>>>,
    post_id: web::Path<u64>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut inner_data = data.lock().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to lock mutex"))?;

    if let Some(index) = inner_data.iter().position(|post| post.post_id == *post_id) {
        let deleted_post = inner_data.remove(index);
        Ok(HttpResponse::Ok().json(Some(deleted_post)))
    } else {
        Ok(HttpResponse::NotFound().json(None::<Post>))
    }
}