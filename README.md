# rust-chromedriver-autoinstaller
A rewriting of Yeongbin Jo's [python-chromedriver-autoinstaller](https://github.com/yeongbin-jo/python-chromedriver-autoinstaller) in Rust.

Usage:
```rust
use chromedriver_autoinstaller as cda;

fn main() {
    // print Chrome version
    println!("Installed Chrome version: {}", cda::get_chrome_version().unwrap());

    // automatically install Chromedriver
    // use cwd == false
    // specify directory == false
    // no_ssl == false
    cda::install(false, None, false).unwrap();
}
```

I have only tested the library on Windows. If you've got a Mac or Linux computer and find an error, please open an issue or submit a PR!