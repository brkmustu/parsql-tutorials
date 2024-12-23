use parsql_tutorials::{
    state::AppState,
    user_handlers::{insert_user, InsertUser, UserState},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new().await;
    let user = InsertUser {
        name: "Ali".to_string(),
        email: "ali@veli".to_string(),
        state: UserState::Active as i16,
    };
    let insert_result = insert_user(app_state, user).await;
    println!("insert_result: {:?}", insert_result);
    Ok(())
}
