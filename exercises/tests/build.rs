use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
       .duration_since(UNIX_EPOCH)
       .unwrap()
       .as_secs();

    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    if let Ok(bin_name) = env::var("CARGO_BIN_NAME") {
        if bin_name == "tests8" {
            // 启用 pass 特性
            println!("cargo:rustc-cfg=feature=\"pass\"");
        }
    }
}
