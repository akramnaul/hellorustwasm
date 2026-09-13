```rust
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
fn handle_hello_world(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello Rust WASM</title>
</head>

<body>
    <h1>Hello, World!</h1>

    <p>
        Hello, World ! ... Rust .WASM in the Browser
    </p>

</body>
</html>
"#,
        )
        .build())
}
```
