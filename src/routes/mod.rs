use super::DbPool;
use crate::models::TetraTelegram;

use actix_web::Responder;
use actix_web::{web, HttpRequest};
use diesel::RunQueryDsl;
use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    success: bool,
}

// /tetra/
pub async fn receiving_tetra(
    pool: web::Data<DbPool>,
    //app_state: web::Data<Mutex<ApplicationState>>,
    telegram: web::Json<TetraTelegram>,
    _req: HttpRequest,
) -> impl Responder {
    // getting the connection from the postgres connection pool
    let mut database_connection = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("cannot get connection from connection pool {:?}", e);
            return web::Json(Response { success: false });
        }
    };

    let unwrapped_telgram = telegram.into_inner();

    // writing telegram into database
    if let Err(e) = diesel::insert_into(crate::schema::tetra_data::table)
        .values(&unwrapped_telgram)
        .execute(&mut database_connection)
    {
        warn!(
            "Postgres Error {:?} with telegram: {:?}",
            e, &unwrapped_telgram
        );
    }

    web::Json(Response { success: true })
}
