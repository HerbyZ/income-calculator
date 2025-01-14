# Income calculator

App for calculating income of any investments.

Note, that app needs __storage.json__ file (creates automatically), be careful with files stored in the same directory.

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

Rename it as you wish and move it anywhere you like, just be careful with files named __*.json__ in the same directory, they may be overritten.

Currently, app needs __storage.json__ file (it will create it automatically) in the same as binary directory, to store data about saved positions.

If you want to change path of __settings.json__ file, you should edit `src/storage/mod.rs` and set constraint `STORAGE_FILE_PATH` to your preferred path, for example:

```rust
// src/storage/mod.rs
const STORAGE_FILE_PATH: &str = "/home/username/.config/income-calculator"
```

__Note, that file will be created automatically, but directory should exist before launch.__
