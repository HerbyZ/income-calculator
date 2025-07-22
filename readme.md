# Income calculator

App for calculating income of any investments. Made in [Rust](https://www.rust-lang.org/).

## Installation
### Building from sources
To build application you'll need to install [cargo](https://www.rust-lang.org/tools/install) (tested on version 1.83.0).


```
git clone https://github.com/HerbyZ/income-calculator
cd income-calculator
cargo build --release
```

When compilation is complete, the binary will be built to target/release/ directory. You can run it with

```
./target/release/income-calc
```

Rename it as you wish and move it anywhere you like, just be careful with files named __*.json__ in the same directory, they may be overwritten.

Currently, app needs __options.json__ and __storage.json__ files (it will create them automatically) in the same as binary directory, to store options and data about saved positions. Also, by default their paths are relative and set to `"./options.json"` and `"./storage.json"` so it's necessary to run app in its directory (where the binary and json files are stored).

## Configuring

If you want to change the path of __options.json__ file, you have to change file `src/constants.rs` and set `OPTIONS_FILE_PATH` as you wish, for example:

```rust
// src/constants.rs
const OPTIONS_FILE_PATH: &str = "/home/username/.config/income-calculator/options.json"
```

__Note, that file will be created automatically, but directory should exist before launch.__

Then rebuild the application.

If you want to change the path of __storage.json__ file, you have to run app for the first time. It will create the __options.json__ file at path, that you set in previous step, by default it's equal to `"./options"`. Than you need to change `storage_file_path` option (`"./storage"` by default), for example:

```json
{
    ...,
    "options_file_path": "/home/username/.config/income-calculator/storage.json"
}
```

__Note, that file will be created automatically, but directory should exist before launch.__

Changes in __options.json__ do not require rebuilding the application.

### Available options
* POSITIONS_PER_PAGE: amount of positions that will be shown on a single page
* ORDERS_PER_PAGE: same as previous but affects orders
* STORAGE_FILE_PATH: path, where app will create and look for the file with saved data about positions and sorting.
