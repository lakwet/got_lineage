use actix_web::{web, HttpResponse};
use mysql::Pool;
use serde::Deserialize;

use super::super::logic::next_heir;
use super::db::{kill_character, read_all, read_characters};

#[derive(Deserialize)]
pub struct QueryParam {
    pub name: String,
}

pub async fn handler_next(
    pool: &'static Pool,
    q: web::Query<QueryParam>,
) -> HttpResponse {
    match read_all(pool) {
        Ok((characters, relationships)) => {
            if characters.iter().find(|c| c.name == q.name).is_none() {
                HttpResponse::NoContent()
                    .body(format!("{:?} does not exists.", &q.name))
            } else {
                match next_heir(&q.name, &characters, &relationships) {
                    Some(character) => HttpResponse::Ok().body(format!(
                        "{:?} next heir is: {:?}",
                        &q.name, &character.name
                    )),
                    None => HttpResponse::Ok()
                        .body(format!("{:?} has no heir anymore.", &q.name)),
                }
            }
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Internal Server Error: {}", err)),
    }
}

pub async fn handler_kill(
    pool: &'static Pool,
    q: web::Query<QueryParam>,
) -> HttpResponse {
    match read_characters(pool) {
        Ok(characters) => {
            if characters.iter().find(|c| c.name == q.name).is_none() {
                HttpResponse::NoContent()
                    .body(format!("{:?} does not exists.", &q.name))
            } else {
                println!("Killing: {:?}", &q.name);
                match kill_character(&q.name, pool) {
                    Ok(_) => HttpResponse::Ok()
                        .body(format!("{:?} has been killed !", &q.name)),
                    Err(err) => HttpResponse::InternalServerError()
                        .body(format!("Internal Server Error: {}", err)),
                }
            }
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Internal Server Error: {}", err)),
    }
}
