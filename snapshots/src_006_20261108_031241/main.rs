// main.rs
// Date: Sun Nov 08 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Redis
// Goal: ...
// Dependency: Redis

// rustc 1.101.0-nightly (249c1e4e3 2026-11-05)
// binary: rustc
// commit-hash: 249c1e4e31111b54ad2483cfd4cea06c7020c653
// commit-date: 2026-11-05
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.5

// cargo 1.101.0-nightly (3c747ea3e 2026-11-03)
// release: 1.101.0-nightly
// commit-hash: 3c747ea3ead23951c1b43a5edfcbdb449135ddeb
// commit-date: 2026-11-03
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.15-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use redis::{Commands, RedisError};
use redis_client::Person;

fn main() -> Result<(), String> {
    println!("\n");

    let client_result = redis::Client::open("redis://127.0.0.1");
    let client = match client_result {
        Ok(cl) => cl,
        Err(err) => return Err(err.to_string()),
    };

    let pool_result = r2d2::Pool::builder().build(client);
    let pool = match pool_result {
        Ok(pool) => pool,
        Err(err) => return Err(err.to_string()),
    };

    let conn_result = pool.get();
    let mut conn = match conn_result {
        Ok(cn) => cn,
        Err(err) => return Err(err.to_string()),
    };

    let raphael = Person::default();
    let raphael_json_result = serde_json::to_string(&raphael);
    let raphael_json = match raphael_json_result {
        Ok(json_resp) => json_resp,
        Err(err) => return Err(err.to_string()),
    };

    let set_result: Result<(), RedisError> = conn.set("raphael:1", raphael_json);
    match set_result {
        Ok(()) => println!("Person insert to the redis successfully ..."),
        Err(err) => return Err(err.to_string()),
    }

    println!("\nThe End ...\n");
    Ok(())
}
