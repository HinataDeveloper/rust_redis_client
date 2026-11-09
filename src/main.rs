// main.rs
// Date: Mon Nov 09 2026
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

use redis_client::Person;
use redis_client::choose;
use redis_client::remove;
use redis_client::save_or_update;
use redis_client::test_connection;

fn main() {
    println!("\n");

    if test_connection() {
        println!("Connection is open ....");
    } else {
        println!("I am sorry to say connection is closed ...");
    }

    //--------------------------------------

    let raphael = Person::default();
    println!("raphael is: {raphael}");

    let save_result = save_or_update("raphael:1", &raphael);
    if save_result {
        println!("Rahael save into Redis Successfully ...");
    } else {
        println!("Saver Operation was failed ...");
    }

    //--------------------------------------

    let person_option = choose("raphael:1");
    let raphael = match person_option {
        Some(raph) => raph,
        None => Person::new(0, "", "", 0),
    };

    println!("retured person from redis is: {raphael}");

    //--------------------------------------

    let result_remove = remove("raphael:1");
    if result_remove {
        println!("person was removed from Redis successfully ...");
    } else {
        println!("remove operation was failed ...");
    }

    println!("\nThe End ...\n");
}
