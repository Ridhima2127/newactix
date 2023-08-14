use crate::controller::posts::{Category, Post};
use actix_web::HttpResponse;

pub(crate) async fn get_posts() -> Result<Vec<Post>, actix_web::Error> {
    let posts = vec![
        Post {
            title: "First Post".to_string(),
            description: "Description of first post".to_string(),
            category: "Travel".to_string(),
            category_id: 1,
        },
        Post {
            title: "Second Post".to_string(),
            description: "Description of second post".to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
    ];

    Ok(posts)
}

pub(crate) async fn get_categories() -> Result<Vec<Category>, actix_web::Error> {
    let categories = vec![
        Category {
            id: 1,
            name: "First Category".to_string(),
            num_of_posts: 20,
        },
        Category {
            id: 2,
            name: "Second Category".to_string(),
            num_of_posts: 10,
        },
    ];
    Ok(categories)
}

pub async fn get_specific_post() -> Result<Vec<Post>, actix_web::Error> {
    // Select posts with limit and offset

    let posts = vec![
        Post {
            title: "First Post".to_string(),
            description: "There are numerous benefits to travelling if we think about it. The first one being, we get to meet new people. When you meet new people, you get the opportunity to make new friends. It may be a fellow traveller or the local you asked for directions.
".to_string(),
            category: "Travel".to_string(),
            category_id: 2,
        },
    ];

    Ok(posts)
}
