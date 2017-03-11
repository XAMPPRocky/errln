# errln

Two convenience macros for writing to `stderr`.

## Example usage

```rust
#[macro_use]
extern crate errln;

use std::{env,process};

fn main() {
   if env::args_os().len() <= 1 {
       errln!("This program doesn't take any arguments or options.");
       process::exit(1)
   }
}
```

```rust
err!("Error ");
let error_code = 5;
errln!("CODE: {}", error_code);
```

## Changelog

* **0.2.0**: Ignore IO errors and allow passing zero arguments to `errln!()`.
* **0.1.0**: Initial release.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

