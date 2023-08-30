#![allow(unused)]

mod controller;
mod model;

use crate::controller::admin::admin_categories::delete_category_by_id;
use crate::controller::admin::admin_posts::{delete_post_by_id, edit_post};
use crate::controller::posts::category_posts;
use crate::model::database::{init_categories, init_posts};
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::Mutex;

async fn _todo() -> HttpResponse {
    HttpResponse::Ok().body("TODO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let v = init_posts().await.unwrap();
    let v1 = init_categories().await.unwrap();
    let data = web::Data::new(controller::admin::admin_posts::AppState {
        database_post: Mutex::new(v.clone()),
        database_category: Mutex::new(v1.clone()),
    });

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(controller::posts::index))
            .service(
                web::scope("/posts")
                    .route("", web::get().to(controller::posts::index))
                    .route(
                        "/page/{page_number}",
                        web::get().to(controller::posts::pagination_index),
                    )
                    .route(
                        "/{post_id}",
                        web::get().to(controller::posts::specific_post),
                    )
                    .route(
                        "/category/{category_id}/page/{page_number}",
                        web::get().to(category_posts),
                    )
                    /* /posts/category/page*/
                    .route(
                        "/category/{category_id}",
                        web::get().to(controller::posts::category_posts),
                    ),
            )
            .service(web::resource("/login").route(web::get().to(controller::login::login::login)))
            .service(
                web::resource("/login_user")
                    .route(web::post().to(controller::login::login::login_user)),
            )
            .service(web::resource("/logout").to(_todo))
            .service(actix_files::Files::new("/assets", "assets/").show_files_listing())
            .service(
                web::scope("/admin")
                    .route("", web::get().to(controller::admin::admin_posts::homepage))
                    .route(
                        "/page/{page_number}",
                        web::get().to(controller::admin::admin_posts::pagination_homepage),
                    )
                    /*.route("/edit/{post_id}", web::get().to(edit_post_by_id))*/


                    .route(
                        "/edit",
                        web::get().to(controller::admin::admin_posts::edit_post_html),
                    )
                    .app_data(data.clone())
                    .route(
                        "/update/{post_id}",
                        web::post().to(controller::admin::admin_posts::edit_post),
                    )


                    .route("/delete/{post_id}", web::get().to(delete_post_by_id))
                    .route(
                        "/delete/category/{category_id}",
                        web::get().to(delete_category_by_id),
                    )
                    .route(
                        "/category",
                        web::get().to(controller::admin::admin_categories::admin_category),
                    )
                    .route(
                        "/category/page/{page_number}",
                        web::get()
                            .to(controller::admin::admin_categories::admin_category_pagination),
                    )
                    .route(
                        "/new",
                        web::get().to(controller::admin::admin_posts::new_post),
                    )
                    .app_data(data.clone())
                    .route(
                        "/create",
                        web::post().to(controller::admin::admin_posts::create_post),
                    )
                    .route(
                        "/new/category",
                        web::get().to(controller::admin::admin_categories::new_category),
                    )
                    .app_data(data.clone())
                    .route(
                        "/create/category",
                        web::post().to(controller::admin::admin_categories::create_category),
                    )
                    .route("/categories", web::get().to(controller::posts::index))
                    .route("/categories/page/{page_number}", web::get().to(_todo)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
