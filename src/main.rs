extern crate actix_rt;
extern crate actix_web;
extern crate csv;
#[macro_use]
extern crate mysql;
extern crate serde;

pub mod data;
pub mod logic;
pub mod server;

use actix_web::{web, App, HttpServer};
use std::net;
use std::process;
use std::str::FromStr;

use data::read_raw_input;
use server::db::{create_tables, fill_tables_with_raw_input, get_pool};
use server::handler::{handler_kill, handler_next, QueryParam};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = match server::env::read_config_from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("Missing env variable: {}", err);
            process::exit(1);
        }
    };

    let pool = match get_pool(&config) {
        Ok(pool) => pool,
        Err(err) => {
            println!("Unable to connect to database: {}", err);
            process::exit(1);
        }
    };
    let pool = Box::leak(pool.into()) as &'static mysql::Pool;

    match create_tables(&pool) {
        Ok(_) => (),
        Err(err) => {
            println!("Unable to create tables: {}", err);
            process::exit(1);
        }
    }

    if config.reset_characters {
        match read_raw_input() {
            Ok(relationship) => {
                match fill_tables_with_raw_input(&config, &pool, relationship) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Unable to fill tables: {}", err);
                        process::exit(1);
                    }
                }
            }
            Err(err) => {
                println!("Unable to read CSV file: {}", err);
                process::exit(1);
            }
        }
    }

    let bind_addr = net::SocketAddr::from_str(&format!(
        "127.0.0.1:{}",
        &config.listen_port
    ))
    .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .route(
                "/api/next",
                web::get().to(move |q: web::Query<QueryParam>| {
                    handler_next(&pool, q)
                }),
            )
            .route(
                "/api/kill",
                web::get().to(move |q: web::Query<QueryParam>| {
                    handler_kill(&pool, q)
                }),
            )
    });

    println!("Server is running: http://127.0.0.1:9898");

    server.bind(&bind_addr)?.run().await
}
