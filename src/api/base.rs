pub(crate) const POKE_API_BASE_URL: &'static str = "https://pokeapi.co/api";

pub(crate) static POKE_API_CLIENT_NAME: &str = concat!(
    "rs-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);