use crate::config::AppConfig;
use crate::errors::HWSystemError;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};

/// 哈希密码
pub fn hash_password(password: &str) -> Result<String, HWSystemError> {
    let config = AppConfig::get();
    let params = Params::new(
        config.argon2.memory_cost,
        config.argon2.time_cost,
        config.argon2.parallelism,
        None,
    )
    .map_err(|e| HWSystemError::validation(format!("Argon2 参数错误: {e}")))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| HWSystemError::validation(format!("密码哈希失败: {e}")))?;
    Ok(hash.to_string())
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
