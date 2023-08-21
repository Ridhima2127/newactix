#![allow(unused)]

mod controller;
mod model;

use crate::controller::posts::category_posts;
use actix_web::{web, App, HttpResponse, HttpServer};


async fn _todo() -> HttpResponse {
    HttpResponse::Ok().body("TODO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
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
                    .route("/", web::get().to(controller::admin::admin_posts::homepage))
                    .route(
                        "/page/{page_number}",
                        web::get().to(controller::admin::admin_posts::pagination_homepage),
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
                    .route("/new", web::get().to(controller::admin::new_post::new_post))


                    .route("/page/{page_number}", web::get().to(_todo))
                    .route(
                        "/posts/{post_id}/edit",
                        web::get().to(controller::posts::edit_post),
                    )

                    .route(
                        "/posts/post_id/delete",
                        web::get().to(controller::posts::delete_post),
                    )
                    .route("/categories", web::get().to(controller::posts::index))
                    .route("/categories/page/{page_number}", web::get().to(_todo))
                    .route(
                        "/categories/category_id/edit",
                        web::get().to(controller::category::edit_category),
                    )
                    .route(
                        "/categories/new",
                        web::get().to(controller::category::new_category),
                    )
                    .route(
                        "/categories/{category_id}/delete",
                        web::get().to(controller::category::delete_category),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
