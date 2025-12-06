use crate::utils; 

use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use axum::extract;
use axum::{
    http::StatusCode,
    Json,
};
use std::string::String;
use serde::{Deserialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct UserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub mobile: String,
    pub username: String,
    pub password: String,
}

// #[derive(FromRow, Serialize)]
// pub struct User {
//     pub id: i32,
//     pub firstname: String,
//     pub lastname: String,
//     pub email: String,
//     pub mobile: String,
//     pub username: String,
//     pub password: String,
//     pub roles: String,
//     pub isactivated: i32,

// }

// #[derive(Serialize, Clone, Debug)]
// pub struct UserResponse {
//     pub message: String,
// }

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<UserRequest>
) -> (StatusCode, Json<Value>) {

    let email_result: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM users WHERE email = $1"#)
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to fetch count");

    if email_result > 0 {
        
        let response = json!({
            "message": "Email Address is already taken."
        });            
        return (StatusCode::CONFLICT, Json(response))

    } else {

        let username_result: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM users WHERE username = $1"#)
        .bind(&payload.username)
        .fetch_one(&state.pool)
        .await
        .expect("Failed to fetch count");

        if username_result > 0 {

            let response = json!({
                "message": "Username is already taken."
            });            
            return (StatusCode::CONFLICT, Json(response))    
        }


    }

    let hashed_password = utils::hash_password(&payload.password).unwrap();
    
    let result = sqlx::query("INSERT INTO users (firstname,lastname,email,mobile,username,password) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&payload.firstname)
        .bind(&payload.lastname)
        .bind(&payload.email)
        .bind(&payload.mobile)
        .bind(&payload.username)
        .bind(&hashed_password)
        .execute(&state.pool)
        .await;



    match result {
        Ok(_) => {

            let response = json!({
                "message": "You have registered successfully."
            });            
            return (StatusCode::CREATED, Json(response));
        } 
        Err(e) => {
            let response2 = json!({
                "message": format!("Failed to create user: {}", e)
            });            
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response2));
            // Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create user: {}", e)))
        } 
    }    

    // let newuser = UserResponse {
    //     message: format!("You have registered successfully, your User ID is : {}", inserted_row)
    // };

    // return (StatusCode::CREATED, Json(newuser))
    


}