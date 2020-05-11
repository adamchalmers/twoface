# Twoface

`twoface::Error` wraps a Rust error type with a user-facing description. This stops users from seeing
your internal errors, which might contain sensitive implementation details that should be kept
private.

```rust
use twoface::{ResultExt, Error};

fn read_private_file() -> Result<String, Error<&'static str>> {
    // Do not leak this path to users!
    let secret_path = "/secrets/user01/profile.txt";
    // Use `describe_err` to wrap the result's Err value into a twoface::Error.
    std::fs::read_to_string(secret_path).describe_err("Could not get profile")
}

/// Returns the user's profile (or a user-friendly error message).
fn get_user_response() -> String {
    match read_private_file() {
        Ok(s) => format!("Your profile: {}", s),
        Err(e) => {
            // Log the internal error
            eprintln!("ERROR: {:?}", e.internal);
            // Return the external error to users.
            e.to_string()
        }
    }
}
```
