# Twoface

`twoface::Error` raps a Rust error type with a user-facing description. This stops users from seeing
your internal errors, which might contain sensitive implementation details that should be kept
private.