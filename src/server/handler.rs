use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use mysql::Pool;

use super::db::{read_all, kill_character};
use super::super::logic::next_heir;

#[derive(Deserialize)]
pub struct QueryParam {
    pub name: String,
}

pub async fn handler_next(pool: &'static Pool, q: web::Query<QueryParam>) -> impl Responder {
    let (characters, relationships) = read_all(pool);

    if characters.iter().find(|c| c.name == q.name).is_none() {
        HttpResponse::Ok().body(format!("{:?} does not exists.", &q.name))
    } else {
        match next_heir(&q.name, &characters, &relationships) {
            Some(character) => HttpResponse::Ok().body(format!("{:?} next heir is: {:?}", &q.name, &character.name)),
            None => HttpResponse::Ok().body(format!("{:?} has no heir anymore.", &q.name))
        }
    }
}

pub async fn handler_kill(pool: &'static Pool, q: web::Query<QueryParam>) -> impl Responder {
    println!("Killing: {:?}", &q.name);
    kill_character(&q.name, pool);
    HttpResponse::Ok().body(format!("{:?} has been killed !", &q.name))
}
