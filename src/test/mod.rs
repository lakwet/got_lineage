use actix_web::{http, web};

use super::data::read_raw_input;
use super::server::db::{create_tables, fill_tables_with_raw_input, get_pool};
use super::server::env::read_config_from_env;
use super::server::handler::{handler_kill, handler_next, QueryParam};

#[actix_rt::test]
async fn test_handler_next() {
    let config = read_config_from_env().unwrap();
    let pool = get_pool(&config).unwrap();
    let pool = Box::leak(pool.into()) as &'static mysql::Pool;
    create_tables(&pool).unwrap();
    let inputs = read_raw_input().unwrap();
    fill_tables_with_raw_input(&config, &pool, inputs).unwrap();

    let query: web::Query<QueryParam> =
        web::Query::from_query("name=toto").unwrap();
    let resp = handler_next(&pool, query).await;
    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

    let query: web::Query<QueryParam> =
        web::Query::from_query("name=toto").unwrap();
    let resp = handler_kill(&pool, query).await;
    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

    let query: web::Query<QueryParam> =
        web::Query::from_query("name=Aerys Targaryen").unwrap();
    let resp = handler_kill(&pool, query).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
