use crate::controller::posts::{Category, Post};
use actix_web::HttpResponse;

pub(crate) async fn get_posts() -> Result<Vec<Post>, actix_web::Error> {
    let posts = vec![
        Post {
            post_id: 1,
            title: "First Post".to_string(),
            description: "There are numerous benefits to travelling if we think about it. "
                .to_string(),
            category: "Travel".to_string(),
            category_id: 1,
        },
        Post {
            post_id: 2,
            title: "Second Post".to_string(),
            description: "Always dream and shoot higher than you know you can do. ".to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
        Post {
            post_id: 3,
            title: "Third Post".to_string(),
            description: " A home is made of hopes and dreams.".to_string(),
            category: "Home".to_string(),
            category_id: 3,
        },
        Post {
            post_id: 4,
            title: "Fourth Post".to_string(),
            description: "Be so happy that, when other people look at you, they become happy too."
                .to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
        Post {
            post_id: 5,
            title: "Fifth Post".to_string(),
            description: "Be so happy that, when other people look at you, they become happy too."
                .to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
        Post {
            post_id: 6,
            title: "Sixth Post".to_string(),
            description: "Be so happy that, when other people look at you, they become happy too."
                .to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
        Post {
            post_id: 7,
            title: "Seventh Post".to_string(),
            description: "Be so happy that, when other people look at you, they become happy too."
                .to_string(),
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
            name: "Travel".to_string(),
            num_of_posts: 20,
        },
        Category {
            id: 2,
            name: "Life".to_string(),
            num_of_posts: 10,
        },
        Category {
            id: 3,
            name: "Home".to_string(),
            num_of_posts: 14,
        },
        Category {
            id: 4,
            name: "India".to_string(),
            num_of_posts: 14,
        },
    ];
    Ok(categories)
}

pub async fn get_specific_post() -> Result<Vec<Post>, actix_web::Error> {
    // Select posts with limit and offset

    let posts = vec![
        Post {
            post_id: 1,
            title: "First Post".to_string(),
            description: "There are numerous benefits to travelling if we think about it. The first one being, we get to meet new people. When you meet new people, you get the opportunity to make new friends. It may be a fellow traveller or the local you asked for directions.
".to_string(),
            category: "Travel".to_string(),
            category_id: 2,
        },

        Post {
            post_id: 2,
            title: "Second Post".to_string(),
            description: "Always dream and shoot higher than you know you can do. Love your family, work super hard, live your passion".to_string(),
            category: "Life".to_string(),
            category_id: 2,
        },
        Post {
            post_id: 3,
            title: "Third Post".to_string(),
            description: " A home is made of hopes and dreams.Home is where love resides, memories are created, friends always belong, and laughter never ends.".to_string(),
            category: "Home".to_string(),
            category_id: 3,
        },
        Post {
            post_id: 4,
            title: "Fourth Post".to_string(),
            description: "Be so happy that, when other people look at you, they become happy too.The only way to do great work is to love what you do.".to_string(),
            category: "Life".to_string(),
            category_id: 3,
        },
    ];

    Ok(posts)
}
