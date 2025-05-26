use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    // TODO
}

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| envy::from_env::<Config>().expect("failed to get config from env vars"));
