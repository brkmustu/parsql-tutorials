use parsql::{
    core::{Deleteable, Insertable, Queryable, Updateable},
    macros::{Deleteable, Insertable, Queryable, Updateable},
    tokio_postgres::{delete, get, get_all, insert, update, SqlParams},
};

use tokio_postgres::types::ToSql;

use crate::state::AppState;

pub enum UserState {
    Active = 0,
    Disabled = 1,
}

#[derive(Insertable)]
#[table_name("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

#[derive(Updateable)]
#[table_name("users")]
#[update_clause("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

#[derive(Updateable)]
#[table_name("users")]
#[update_clause("state")]
#[where_clause("id = $")]
pub struct UpdateUserState {
    pub id: i64,
    pub state: i16,
}

#[derive(Deleteable, Debug)]
#[table_name("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

#[derive(Queryable, Debug)]
#[table_name("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Queryable, Debug)]
#[table_name("users")]
#[where_clause("state = $")]
pub struct GetActiveUsers {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetActiveUsers {
    pub fn new(state: i16) -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            email: Default::default(),
            state,
        }
    }
}

pub async fn insert_user(
    state: AppState,
    user: InsertUser,
) -> Result<u64, tokio_postgres::Error> {
    let db = state.db.as_ref(); 
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten sonra "insert" fonksiyonunu çağırıyoruz
    insert(db, user).await
}

pub async fn update_user(
    state: AppState,
    user: UpdateUser,
) -> Result<bool, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "update" fonksiyonunu çağırıyoruz
    update(db, user).await
}

pub async fn set_user_active(
    state: AppState,
    id: i64,
) -> Result<bool, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "update" fonksiyonunu çağır
    let user = UpdateUserState { id, state: UserState::Active as i16 };
    update(db, user).await
}

pub async fn set_user_disabled(
    state: AppState,
    id: i64,
) -> Result<bool, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "update" fonksiyonunu çağır
    let user = UpdateUserState { id, state: UserState::Disabled as i16 };
    update(db, user).await
}

pub async fn delete_user(
    state: AppState,
    user: DeleteUser,
) -> Result<u64, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "delete" fonksiyonunu çağırıyoruz
    delete(db, user).await
}

pub async fn get_user(
    state: AppState,
    id: i64,
) -> Result<GetUser, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "get" fonksiyonunu çağırıyoruz
    let user = GetUser::new(id);
    get(db, user, |row| {
        Ok(GetUser {
            id: row.get(1),
            name: row.get(2),
            email: row.get(3),
            state: row.get(4),
        })
    })
    .await
}

pub async fn get_active_users(
    state: AppState,
) -> Result<Vec<GetActiveUsers>, tokio_postgres::Error> {
    let db = state.db.as_ref();
    // burada kullanıcı üzerinde gelen modeldeki verileri manipüle ettikten (ya da kontrollerden) sonra "get_all" fonksiyonunu çağırıyoruz
    let active_user_state = 0;
    let user = GetActiveUsers::new(active_user_state);
    get_all(db, user, |row| {
        Ok(GetActiveUsers {
            id: row.get(1),
            name: row.get(2),
            email: row.get(3),
            state: row.get(4),
        })
    })
    .await
}
