use actix_web::{web, HttpResponse, Responder};
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
) -> impl Responder {
    let (characters, relationships) = read_all(pool);

    if characters.iter().find(|c| c.name == q.name).is_none() {
        HttpResponse::Ok().body(format!("{:?} does not exists.", &q.name))
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

pub async fn handler_kill(
    pool: &'static Pool,
    q: web::Query<QueryParam>,
) -> impl Responder {
    let characters = read_characters(pool);

    if characters.iter().find(|c| c.name == q.name).is_none() {
        HttpResponse::Ok().body(format!("{:?} does not exists.", &q.name))
    } else {
        println!("Killing: {:?}", &q.name);
        kill_character(&q.name, pool);
        HttpResponse::Ok().body(format!("{:?} has been killed !", &q.name))
    }
}
