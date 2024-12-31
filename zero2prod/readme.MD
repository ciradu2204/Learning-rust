- `cargo watch -x` check to allow continous watching of the project
- `cargo watch -x check -x test -x run` this will chain. So it will first do check, test and then run
- ` cargo clippy` run clippy in the project 
- `cargo clippy -- -D warnings`
- `#[allow(clippy::lint_name)] ` mute warning
- `cargo fmt` display format issues in the project
- `cargo install cargo-audit` to discover vulnerabilities in other crates or dependencies 
- HTTP server helps us to know which socket we should be listening to. A socket has an ip address and a port
  - How many connections can we allow concurrently
  - And if we can allow tls 
- App is the component whose job is to take an incoming request as input and spit our response



