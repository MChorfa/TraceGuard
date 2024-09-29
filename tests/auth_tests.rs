use traceguard::auth;

#[tokio::test]
async fn test_validate_token() {
    let user_id = "test_user";
    let token = auth::create_token(user_id).unwrap();
    let result = auth::validate_token(&token).await;
    assert!(result.is_ok());
    let auth_user = result.unwrap();
    assert_eq!(auth_user.user_id, user_id);
}

#[tokio::test]
async fn test_invalid_token() {
    let result = auth::validate_token("invalid_token").await;
    assert!(result.is_err());
}