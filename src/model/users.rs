use crate::controller::login::login::User;

pub(crate) async fn get_users() -> Result<Vec<User>, actix_web::Error> {
    let users = vec![
       User {
            username: "user1".to_string(),
            password: "pass1".to_string(),
        },
       User{
           username: "user2".to_string(),
           password: "pass2".to_string(),
        },
       User{
           username: "user3".to_string(),
           password: "pass3".to_string(),
        },
       User {
           username: "user4".to_string(),
           password: "pass4".to_string(),
        },
    ];
    Ok(users)
}

