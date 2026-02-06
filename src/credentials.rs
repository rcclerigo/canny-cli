use anyhow::Result;
use security_framework::passwords::{
    delete_generic_password, get_generic_password, set_generic_password,
};

const KEYCHAIN_SERVICE: &str = "canny-cli";
const KEYCHAIN_ACCOUNT_API_KEY: &str = "api-key";
const KEYCHAIN_ACCOUNT_API_URL: &str = "api-url";

/// Resolve the API key using the following priority:
///
/// 1. Explicit key (from --api-key flag or CANNY_API_KEY env var)
/// 2. Stored key from macOS Keychain (via `canny auth`)
pub fn resolve_api_key(explicit_key: Option<String>) -> Result<String> {
    if let Some(key) = explicit_key {
        return Ok(key);
    }

    get_stored_api_key().ok_or_else(|| {
        anyhow::anyhow!(
            "API key not found. Run `canny auth` to configure, or provide --api-key / set CANNY_API_KEY."
        )
    })
}

/// Resolve the API URL using the following priority:
///
/// 1. Explicit URL (from --api-url flag, if different from default)
/// 2. Stored URL from macOS Keychain (via `canny auth`)
/// 3. Falls back to None (caller should use its default)
pub fn resolve_api_url(explicit_url: Option<&str>, default_url: &str) -> Option<String> {
    // If the user passed a non-default --api-url, use it
    if let Some(url) = explicit_url {
        if url != default_url {
            return Some(url.to_string());
        }
    }

    // Try the keychain
    if let Some(url) = get_stored_api_url() {
        return Some(url);
    }

    None
}

/// Store the API key permanently in the macOS Keychain
pub fn store_api_key(api_key: &str) -> Result<()> {
    // Delete existing entry if present (set_generic_password fails if it exists)
    let _ = delete_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_KEY);

    set_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_KEY, api_key.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to store API key in Keychain: {}", e))
}

/// Store the API URL permanently in the macOS Keychain
pub fn store_api_url(api_url: &str) -> Result<()> {
    let _ = delete_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_URL);

    set_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_URL, api_url.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to store API URL in Keychain: {}", e))
}

/// Clear all stored credentials from the macOS Keychain
pub fn clear_stored_credentials() -> Result<()> {
    let mut errors = Vec::new();

    if let Err(e) = delete_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_KEY) {
        errors.push(format!("API key: {}", e));
    }
    if let Err(e) = delete_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_URL) {
        errors.push(format!("API URL: {}", e));
    }

    // Also clean up the old "default" account from pre-auth versions
    let _ = delete_generic_password(KEYCHAIN_SERVICE, "default");

    if errors.len() == 2 {
        anyhow::bail!("No stored credentials to clear");
    }

    Ok(())
}

fn get_stored_api_key() -> Option<String> {
    let data = get_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_KEY).ok()?;
    String::from_utf8(data.to_vec()).ok()
}

fn get_stored_api_url() -> Option<String> {
    let data = get_generic_password(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT_API_URL).ok()?;
    String::from_utf8(data.to_vec()).ok()
}
