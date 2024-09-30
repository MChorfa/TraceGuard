use traceguard::security::encryption::{Encryptor, EncryptionType};
use traceguard::security::key_rotation::KeyRotationManager;
use traceguard::security::secret_management::MockSecretManager;

#[tokio::test]
async fn test_aes256_encryption() {
    let key = [0u8; 32];
    let encryptor = Encryptor::new(EncryptionType::AES256, &key);
    let plaintext = b"Hello, world!";
    let ciphertext = encryptor.encrypt(plaintext).unwrap();
    let decrypted = encryptor.decrypt(&ciphertext).unwrap();
    assert_eq!(plaintext, &decrypted[..]);
}

#[tokio::test]
async fn test_post_quantum_encryption() {
    let key = [0u8; 32];
    let encryptor = Encryptor::new(EncryptionType::PostQuantum, &key);
    let plaintext = b"Hello, quantum world!";
    let ciphertext = encryptor.encrypt(plaintext).unwrap();
    let decrypted = encryptor.decrypt(&ciphertext).unwrap();
    assert_eq!(plaintext, &decrypted[..]);
}

#[tokio::test]
async fn test_homomorphic_encryption() {
    let key = [0u8; 32];
    let encryptor = Encryptor::new(EncryptionType::Homomorphic, &key);
    let plaintext = b"Hello, homomorphic world!";
    let ciphertext = encryptor.encrypt(plaintext).unwrap();
    let decrypted = encryptor.decrypt(&ciphertext).unwrap();
    assert_eq!(plaintext, &decrypted[..]);
}

#[tokio::test]
async fn test_key_rotation() {
    let mock_secret_manager = MockSecretManager::new();
    let key_rotation_manager = KeyRotationManager::new(mock_secret_manager);
    let tenant_id = uuid::Uuid::new_v4();
    let key_id = "test_key";

    // Set initial key
    key_rotation_manager.secret_manager.set_secret(key_id, "initial_key", tenant_id).await.unwrap();

    // Rotate key
    key_rotation_manager.rotate_key(key_id, tenant_id).await.unwrap();

    // Check if new key exists
    let new_key = key_rotation_manager.secret_manager.get_secret(&format!("{}_new", key_id), tenant_id).await.unwrap();
    assert!(new_key.len() > 0);

    // Check if old key is marked as rotated
    let old_key = key_rotation_manager.secret_manager.get_secret(key_id, tenant_id).await.unwrap();
    assert!(old_key.contains("|"));

    // Get active key (should be old key for 7 days)
    let active_key = key_rotation_manager.get_active_key(key_id, tenant_id).await.unwrap();
    assert_eq!(active_key, "initial_key");
}