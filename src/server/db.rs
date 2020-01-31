use mysql::{from_row, OptsBuilder, Pool};

use super::super::data::{Gender, Relationship};
use super::env::Config;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub sex: Gender,
    pub alive: bool,
}

fn string_to_gender(s: String) -> Gender {
    if s == "M" {
        Gender::M
    } else {
        Gender::F
    }
}

fn gender_to_string(g: Gender) -> String {
    match g {
        Gender::M => "M".into(),
        Gender::F => "F".into(),
    }
}

pub fn get_pool(config: &Config) -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(Some(config.mysql_host))
        .db_name(Some(config.mysql_db_name))
        .user(Some(config.mysql_user))
        .pass(Some(config.mysql_password))
        .tcp_port(config.mysql_port);

    Pool::new(builder).unwrap()
}

pub fn create_tables(pool: &Pool) {
    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS relationship (
        parent_name VARCHAR(100) NOT NULL,
        parent_sex ENUM('M','F') NOT NULL,
        child_name VARCHAR(100) NOT NULL,
        child_sex ENUM('M','F') NOT NULL
    )",
        (),
    )
    .unwrap();

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS characters (
        name VARCHAR(100) UNIQUE NOT NULL,
        sex ENUM('M','F') NOT NULL,
        alive BOOLEAN NOT NULL
    )",
        (),
    )
    .unwrap();
}

pub fn fill_tables_with_raw_input(
    config: &Config,
    pool: &Pool,
    relationships: Vec<Relationship>,
) {
    if !config.reset_characters {
        return;
    }

    // first, clean tables
    pool.prep_exec(r"TRUNCATE TABLE characters", ()).unwrap();
    pool.prep_exec(r"TRUNCATE TABLE relationship", ()).unwrap();

    // then insert values
    for mut stmt in pool
        .prepare(
            r"INSERT INTO relationship
            (parent_name, parent_sex, child_name, child_sex)
            VALUES
            (:parent_name, :parent_sex, :child_name, :child_sex)",
        )
        .into_iter()
    {
        for c in relationships.iter() {
            stmt.execute(params! {
                "parent_name" => &c.parent_name,
                "parent_sex" => gender_to_string(c.parent_sex),
                "child_name" => &c.child_name,
                "child_sex" => gender_to_string(c.child_sex),
            })
            .unwrap();
        }
    }

    for mut stmt in pool
        .prepare(
            r"INSERT IGNORE INTO characters
            (name, sex, alive)
            VALUES
            (:name, :sex, TRUE)",
        )
        .into_iter()
    {
        for c in relationships.iter() {
            stmt.execute(params! {
                "name" => &c.parent_name,
                "sex" => gender_to_string(c.parent_sex),
            })
            .unwrap();
            stmt.execute(params! {
                "name" => &c.child_name,
                "sex" => gender_to_string(c.child_sex),
            })
            .unwrap();
        }
    }
}

pub fn kill_character(name: &str, pool: &Pool) {
    pool.prep_exec(
        "UPDATE characters SET alive = FALSE WHERE name = :name",
        params! {
            "name" => name,
        },
    )
    .unwrap();
}

pub fn read_characters(pool: &Pool) -> Vec<Character> {
    pool.prep_exec("SELECT * FROM characters", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (name, sex, alive): (String, String, bool) =
                        from_row(row);
                    Character {
                        name,
                        sex: string_to_gender(sex),
                        alive,
                    }
                })
                .collect()
        })
        .unwrap()
}

pub fn read_all(pool: &Pool) -> (Vec<Character>, Vec<Relationship>) {
    let characters = read_characters(pool);

    let relationships: Vec<Relationship> = pool
        .prep_exec("SELECT * FROM relationship", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (parent_name, parent_sex, child_name, child_sex): (
                        String,
                        String,
                        String,
                        String,
                    ) = from_row(row);
                    Relationship {
                        parent_name,
                        parent_sex: string_to_gender(parent_sex),
                        child_name,
                        child_sex: string_to_gender(child_sex),
                    }
                })
                .collect()
        })
        .unwrap();

    (characters, relationships)
}
