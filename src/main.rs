extern crate clap;
extern crate diesel;
extern crate r2d2;

mod models;
mod routes;
mod schema;
mod structs;

pub use routes::receiving_tetra;
use routes::tetra_failed_slots;
use structs::Args;

use actix_web::{web, App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use clap::Parser;
use diesel::{r2d2::ConnectionManager, PgConnection};
use log::{debug, info};
use r2d2::Pool;

use std::env;
use std::fs;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// connects to postgres database
/// default uri: postgres://tlms:{password}@localhost:5432/tlms
/// where the password is read from /run/secrets/postgres_password
pub fn create_db_pool() -> DbPool {
    let password_path = env::var("BORZOI_POSTGRES_PASSWORD_PATH")
        .expect("BORZOI_POSTGRES_PASSWORD_PATH is not set!");
    let password = fs::read_to_string(password_path).expect("cannot read password file!");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("BORZOI_POSTGRES_USER").expect("BORZOI_POSTGRES_USER is not set"),
        password,
        env::var("BORZOI_POSTGRES_HOST").expect("BORZOI_POSTGRES_HOST is not set"),
        env::var("BORZOI_POSTGRES_PORT").expect("BORZOI_POSTGRES_PORT is not set"),
        env::var("BORZOI_POSTGRES_DATABASE").expect("BORZOI_POSTGRES_DATABASE is not set")
    );

    debug!("Connecting to postgres database {}", &database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Failed to create pool.")
}

pub fn get_prometheus() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init();

    info!("Starting Data Collection Server ... ");
    let host = args.host.as_str();
    let port = args.port;

    let postgres_pool = web::Data::new(create_db_pool());

    debug!("Listening on: {}:{}", host, port);
    HttpServer::new(move || {
        let prometheus = get_prometheus();

        App::new()
            .wrap(prometheus)
            .app_data(postgres_pool.clone())
            .route("/tetra", web::post().to(receiving_tetra))
            .route("/tetra/failed_slots", web::post().to(tetra_failed_slots))
    })
    .bind((host, port))?
    .run()
    .await
}
