use traceguard::auth;

#[tokio::test]
async fn test_login() {
    let result = auth::login("testuser", "testpassword").await;
    assert!(result.is_ok());
    let token = result.unwrap();
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_invalid_login() {
    let result = auth::login("testuser", "wrongpassword").await;
    assert!(result.is_err());
}