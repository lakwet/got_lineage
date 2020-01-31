use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub mysql_user: &'static str,
    pub mysql_password: &'static str,
    pub mysql_host: &'static str,
    pub mysql_db_name: &'static str,
    pub mysql_port: u16,
    pub reset_characters: bool,
    pub listen_port: String,
}

fn missing_variable_error(name: &str) -> Box<dyn Error> {
    format!("[Environment] Missing variable for key {}", name).into()
}

fn empty_variable_error(name: &str) -> Box<dyn Error> {
    format!("[Environment] Empty variable for key {}", name).into()
}

fn read_variable(name: &str) -> Result<&'static str, Box<dyn Error>> {
    let value = std::env::var(name)
        .map_err(|_| missing_variable_error(name))?;

    if value.is_empty() {
        Err(empty_variable_error(name))
    } else {
        Ok(Box::leak(value.into_boxed_str()) as &'static str)
    }
}

fn read_optional_variable<T: FromStr>(name: &str) -> Option<T> {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<T>().ok())
}

pub fn read_config_from_env() -> Result<&'static Config, Box<dyn Error>> {
    let mysql_user = read_variable("MYSQL_USER")?;
    let mysql_password = read_variable("MYSQL_PASSWORD")?;
    let mysql_host = read_variable("MYSQL_HOST")?;
    let mysql_db_name = read_variable("MYSQL_DATABASE")?;
    let mysql_port = read_variable("MYSQL_PORT")?;
    let reset_characters = read_variable("GAME_OF_THRONE_RESET_CHARACTER")?;
    let listen_port = read_optional_variable::<String>("SERVER_PORT")
        .unwrap_or_else(|| "9898".to_string());

    let config = Config {
        mysql_user,
        mysql_password,
        mysql_host,
        mysql_db_name,
        mysql_port: mysql_port.parse::<u16>().unwrap(),
        reset_characters: reset_characters == "true",
        listen_port,
    };

    Ok(Box::leak(config.into()) as &'static Config)
}
