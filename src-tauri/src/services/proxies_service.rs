use reqwest::Proxy;

/// Selects a random proxy from the provided list of proxies and formats it.
///
/// # Arguments
/// - `proxies`: A slice of proxy strings to choose from.
///
/// # Returns
/// A formatted proxy string if successful, or an error message if the list is empty or the proxy is invalid.
///
/// # Errors
/// Returns an error if no proxies are available or if the chosen proxy is invalid.
pub fn get_random_proxy(proxies: &[String]) -> Result<String, String> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();

    let proxy = proxies
        .choose(&mut rng)
        .expect("No proxies available");
    format_proxy(proxy)
}

/// Converts a proxy string into a `reqwest::Proxy` object.
///
/// # Arguments
/// - `proxy`: The proxy string, which can include the following formats:
///   - `host:port` (unauthenticated proxy)
///   - `username:password@host:port` (authenticated proxy)
///
/// # Returns
/// A `reqwest::Proxy` object if the proxy string is valid.
pub fn transform_string_to_proxy(proxy: &str) -> Result<Proxy, String> {
    reqwest::Proxy::http(proxy)
        .map_err(|e| format!("Failed to create Proxy object: {}", e))
}

/// Parses a proxy string into its components: host, port, username, and password.
///
/// # Arguments
/// - `proxy`: The proxy string, which can include the following formats:
///   - `host:port` (unauthenticated proxy)
///   - `username:password@host:port` (authenticated proxy)
///
/// # Returns
/// A tuple `(host, port, username, password)` if successful.
pub fn parse_proxy(proxy: &str) -> Result<(String, u16, Option<String>, Option<String>), String> {
    let auth_split: Vec<&str> = proxy.rsplitn(2, '@').collect();
    let (auth_part, address_part) = if auth_split.len() == 2 {
        (Some(auth_split[1]), auth_split[0])
    } else {
        (None, auth_split[0])
    };

    let address_split: Vec<&str> = address_part.split(':').collect();
    if address_split.len() != 2 {
        return Err(format!("Invalid proxy format: {}", proxy));
    }

    let host = address_split[0].to_string();
    let port = address_split[1].parse::<u16>().map_err(|_| format!("Invalid port: {}", address_split[1]))?;

    let (username, password) = if let Some(auth) = auth_part {
        let creds: Vec<&str> = auth.split(':').collect();
        if creds.len() != 2 {
            return Err(format!("Invalid authentication format in proxy: {}", proxy));
        }
        (Some(creds[0].to_string()), Some(creds[1].to_string()))
    } else {
        (None, None)
    };

    Ok((host, port, username, password))
}

fn format_proxy(proxy: &str) -> Result<String, String> {
    let parts: Vec<&str> = proxy.split(':').collect();
    if parts.len() != 4 {
        return Err(format!("Invalid proxy format: {}", proxy));
    }

    let host = parts[0];
    let port = parts[1];
    let username = parts[2];
    let password = parts[3];

    if let Err(_) = port.parse::<u16>() {
        return Err(format!("Invalid port in proxy: {}", proxy));
    }

    Ok(format!("{}:{}@{}:{}", username, password, host, port))
}
