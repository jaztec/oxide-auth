#[macro_use]
extern crate serde_derive;

pub mod db_service;
pub mod primitives;

#[cfg(test)]
fn requires_redis_and_should_skip() -> bool {
    match std::env::var("OXIDE_AUTH_SKIP_REDIS") {
        Err(_) => false,
        Ok(st) => match st.as_str() {
            "1" | "yes" => true,
            _ => false,
        },
    }
}

fn redis_hostname() -> String {
    match std::env::var("OXIDE_REDIS_HOSTNAME") {
        Err(_) => "redis://localhost/3".parse().unwrap(),
        Ok(st) => st
    }
}