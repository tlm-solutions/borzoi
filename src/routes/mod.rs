use super::DbPool;
use crate::models::{TetraFailedSlots, TetraTelegram};

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

        return web::Json(Response { success: false });
    }

    web::Json(Response { success: true })
}

// /tetra/failed_slots
pub async fn tetra_failed_slots(
    pool: web::Data<DbPool>,
    slots: web::Json<TetraFailedSlots>,
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

    let unwrapped_slots = slots.into_inner();

    // writing failed slots into database
    if let Err(e) = diesel::insert_into(crate::schema::tetra_failed_slots::table)
        .values(&unwrapped_slots)
        .execute(&mut database_connection)
    {
        warn!(
            "Postgres Error {:?} with failed slots: {:?}",
            e, &unwrapped_slots
        );

        return web::Json(Response { success: false });
    }

    web::Json(Response { success: true })
}
