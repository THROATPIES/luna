use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::{SURREAL_CREDENTIALS, SURREAL_DATABASE, SURREAL_DB_HOST, SURREAL_NAMESPACE};

pub async fn gather_surreal_handler() -> Surreal<Client> {
    let db = Surreal::new::<Ws>(SURREAL_DB_HOST)
        .await
        .expect("Failed to connect to SurrealDB");
    db.signin(Root {
        username: SURREAL_CREDENTIALS.0,
        password: SURREAL_CREDENTIALS.1,
    })
    .await
    .expect("Failed to sign in to SurrealDB");
    db.use_ns(SURREAL_NAMESPACE)
        .use_db(SURREAL_DATABASE)
        .await
        .expect("Failed to select namespace and database");

    db
}
