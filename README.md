# Twoface

`twoface::Error` wraps a Rust error type with a user-facing description. This stops users from seeing
your internal errors, which might contain sensitive implementation details that should be kept
private.

```rust
    use twoface::{AnyhowExt, Error};

    fn read_private_file() -> Result<String, Error<&'static str>> {
        // Do not leak this path to users!
        let secret_path = "/secrets/user01/profile.txt";
        std::fs::read_to_string(secret_path).map_err(|e|e.describe("Could not get profile"))
    }

    /// Show the user their profile (or a user-friendly error message).
    fn show_profile() -> String {
        match read_private_file() {
            Ok(s) => format!("Your profile: {}", s),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                e.to_string()
            }
        }
    }
```
