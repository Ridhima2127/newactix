use actix_web::HttpResponse;
use crate::controller::posts::{Category, Post};

pub(crate) async fn get_posts() -> Result<Vec<Post>, actix_web::Error>{

    let posts = vec![
        Post {
            title: "First Post".to_string(),
            description: "Description of first post".to_string(),
        },
        Post {
            title: "Second Post".to_string(),
            description: "Description of second post".to_string(),
        },
    ];

    Ok(posts)
}

pub(crate) async fn get_categories() -> Result<Vec<Category>, actix_web::Error>{

    let categories = vec![
        Category {
            name: "First Category".to_string(),
        },
        Category {
            name: "Second Category".to_string(),

        },
    ];
    Ok(categories)
}


pub async fn get_specific_post(
) -> Result<Vec<Post>, actix_web::Error> {
    // Select posts with limit and offset

    let posts = vec![
        Post {
            title: "First Post".to_string(),
            description: "There are numerous benefits to travelling if we think about it. The first one being, we get to meet new people. When you meet new people, you get the opportunity to make new friends. It may be a fellow traveller or the local you asked for directions.
".to_string(),
        },
    ];

    Ok(posts)
}

