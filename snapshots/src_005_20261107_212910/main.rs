// main.rs
// Date: Sat Nov 07 2026
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

use redis::TypedCommands;

fn main() -> Result<(), String> {
    println!("\n");

    let client_result = redis::Client::open("redis://127.0.0.1/");
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
        Ok(pc) => pc,
        Err(err) => return Err(err.to_string()),
    };

    match conn.set(1001, "Raphael") {
        Ok(()) => println!("Set command one is down successfully ..."),
        Err(err) => println!("--->>> {err}"),
    }

    match conn.set(1002, "Samuel") {
        Ok(()) => println!("Set command one is down successfully ..."),
        Err(err) => println!("--->>> {err}"),
    }

    match conn.set(1003, "Daniel") {
        Ok(()) => println!("Set command one is down successfully ..."),
        Err(err) => println!("--->>> {err}"),
    }

    match conn.set(1004, "Albert") {
        Ok(()) => println!("Set command one is down successfully ..."),
        Err(err) => println!("--->>> {err}"),
    }

    match conn.set(1005, "Robert") {
        Ok(()) => println!("Set command one is down successfully ..."),
        Err(err) => println!("--->>> {err}"),
    }

    // ----------------------------------------------

    match conn.get(1003) {
        Ok(value) => match value {
            Some(v) => println!("value is {v}"),
            None => println!("There is no value ..."),
        },
        Err(err) => println!("--->>> {err}"),
    }

    println!("\nThe End ...\n");
    Ok(())
}
