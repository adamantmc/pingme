use std::path::PathBuf;


const PINGME_DIR_NAME: &str = "pingme";
const PINGME_DB_NAME: &str = "db.sqlite";
const PINGME_DB_URL_ENV_VAR: &str = "PINGME_DB_URL";


pub fn get_default_db_path() -> PathBuf {
    home::home_dir().unwrap()
        .join(PathBuf::from(PINGME_DIR_NAME))
        .join(PathBuf::from(PINGME_DB_NAME))
}

pub fn get_db_path() -> PathBuf {
    let mut db_path = get_default_db_path();
    let db_path_env = std::env::var(PINGME_DB_URL_ENV_VAR);

    if db_path_env.is_ok() {
        db_path = PathBuf::from(db_path_env.unwrap());
    }

    return db_path;
}
