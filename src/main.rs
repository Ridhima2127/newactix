#![allow(unused)]

mod controller;
mod model;

use actix_web::{web, App, HttpResponse, HttpServer};

async fn _todo() -> HttpResponse {
    HttpResponse::Ok().body("TODO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/posts")
                    .route("/", web::get().to(controller::posts::index))
                    .route("/display", web::get().to(controller::posts::posts_display))

                    .route(
                        "/{page_number}",
                        web::get().to(controller::posts::specific_post),
                    )
                    .route(
                        "/{post_id}",
                        web::get().to(controller::posts::specific_post),
                    )
                    .route("/category/{category_id}", web::get().to(_todo))
                    .route(
                        "/category/{category_id}/page/{page_number}",
                        web::get().to(_todo),
                    ),
            )

            .service(
                web::resource("/login")
                    .route(web::get().to(_todo))
                    .route(web::post().to(_todo)),
            )
            .service(web::resource("/logout").to(_todo))
            .service(actix_files::Files::new("/assets", "assets/").show_files_listing())
            .service(
                web::scope("/admin")
                    .route("/", web::get().to(controller::posts::index))
                    .route("/page/{page_number}", web::get().to(_todo))


                    .route("/cat", web::get().to(controller::category::categories_display))
                    .route(
                        "/posts/{post_id}/edit",
                        web::get().to(controller::posts::edit_post),
                    )
                    .route("/posts/new", web::get().to(controller::posts::new_post))
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
