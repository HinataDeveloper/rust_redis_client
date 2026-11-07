// lib.rs
// Date: Sat Nov 07 2026
// Educational and Practice Rust Programming Language Code

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

use redis::ConnectionLike;

/// # Errors
pub fn redis_test_connection() -> Result<String, String> {
    let client_result = redis::Client::open("redis://127.0.0.1/");
    let client = match client_result {
        Ok(cl) => cl,
        Err(err) => return Err(err.to_string()),
    };

    let connection_result = client.get_connection();
    let connection = match connection_result {
        Ok(cn) => cn,
        Err(err) => return Err(err.to_string()),
    };

    if connection.is_open() {
        Ok(String::from("Redis Connection is Open ..."))
    } else {
        Err(String::from(
            "I am sorry to say Redis connection is closed ...",
        ))
    }
}
