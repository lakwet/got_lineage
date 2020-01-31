use mysql::{from_row, OptsBuilder, Pool};

use super::super::data::{Gender, Relationship};
use super::env::Config;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub sex: Gender,
    pub alive: bool,
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
            stmt.execute(params!{
                "parent_name" => &c.parent_name,
                "parent_sex" => if c.parent_sex == Gender::M { "M" } else { "F" },
                "child_name" => &c.child_name,
                "child_sex" => if c.child_sex == Gender::M { "M" } else { "F" },
            }).unwrap();
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
                "sex" => if c.parent_sex == Gender::M { "M" } else { "F" },
            })
            .unwrap();
            stmt.execute(params! {
                "name" => &c.child_name,
                "sex" => if c.child_sex == Gender::M { "M" } else { "F" },
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

pub fn read_all(pool: &Pool) -> (Vec<Character>, Vec<Relationship>) {
    let characters: Vec<Character> = pool
        .prep_exec("SELECT * FROM characters", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (name, sex, alive): (String, String, bool) =
                        from_row(row);
                    Character {
                        name,
                        sex: if sex == "M" { Gender::M } else { Gender::F },
                        alive,
                    }
                })
                .collect()
        })
        .unwrap();

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
                        parent_sex: if parent_sex == "M" {
                            Gender::M
                        } else {
                            Gender::F
                        },
                        child_name,
                        child_sex: if child_sex == "M" {
                            Gender::M
                        } else {
                            Gender::F
                        },
                    }
                })
                .collect()
        })
        .unwrap();

    (characters, relationships)
}
