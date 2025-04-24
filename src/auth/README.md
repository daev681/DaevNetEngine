ex)
```rust
fn handle_auth_packet(data: &[u8]) {
    // 예: JSON으로 받은 요청 가정
    let req: AuthRequest = serde_json::from_slice(data).unwrap();

    let service = AuthService::new();
    let res = service.authenticate(req);

    println!("Auth result: success={}, token={:?}", res.success, res.token);
}
```