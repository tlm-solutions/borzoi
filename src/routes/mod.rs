use super::DbPool;
/*use tlms::management::Station;
use tlms::telegrams::{
    r09::{R09ReceiveTelegram, R09SaveTelegram},
    raw::{RawReceiveTelegram, RawSaveTelegram},
    AuthenticationMeta, TelegramMetaInformation,
};
*/
use crate::models::TetraTelegram;

use actix_web::Responder;
use actix_web::{web, HttpRequest};
use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use chrono::Utc;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Response {
    success: bool,
}

/*
struct QueryResponse {
    pub telegram_meta: TelegramMetaInformation,
    pub approved: bool,
}

async fn authenticate(conn: &mut PgConnection, auth: &AuthenticationMeta) -> Option<QueryResponse> {
    let station;
    {
        use tlms::schema::stations::dsl::stations;
        use tlms::schema::stations::id;

        match stations.filter(id.eq(auth.station)).first::<Station>(conn) {
            Ok(data) => {
                station = data;
            }
            Err(e) => {
                error!("Err: {:?}", e);
                return None;
            }
        };
    }
    if station.id != auth.station
        || station.token != Some(auth.token.clone())
        || station.deactivated
    {
        // authentication for telegram failed !
        return None;
    }

    Some(QueryResponse {
        telegram_meta: TelegramMetaInformation {
            time: Utc::now().naive_utc(),
            station: station.id,
            region: station.region,
        },
        approved: station.approved,
    })
}
*/

// /tetra/
pub async fn receiving_tetra(
    pool: web::Data<DbPool>,
    //app_state: web::Data<Mutex<ApplicationState>>,
    telegram: web::Json<TetraTelegram>,
    _req: HttpRequest,
) -> impl Responder {
    /*if app_state.is_poisoned() {
        error!("cannot unwrap app state because the lock is poisenous");
        return web::Json(Response { success: false });
    }

    info!(
        "Tetra Telegram! {} {:?}",
        &telegram.auth.station, &telegram
    );
    */

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
