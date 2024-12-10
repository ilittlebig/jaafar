use reqwest::Proxy;

///
///
///
pub fn get_random_proxy(proxies: &[String]) -> Result<&String, String> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();

    proxies
        .choose(&mut rng)
        .ok_or_else(|| "No proxies available".to_string())
}

///
///
///
pub fn format_proxy(proxy: &str, for_proxy_server: bool) -> Result<String, String> {
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

    if for_proxy_server {
        Ok(format!("{}:{}@{}:{}", username, password, host, port))
    } else {
        Ok(format!("http://{}:{}@{}:{}", username, password, host, port))
    }
}

///
///
///
pub fn transform_string_to_proxy(proxy: &str) -> Result<Proxy, String> {
    let formatted_proxy = format_proxy(proxy, false)?;
    reqwest::Proxy::http(&formatted_proxy)
        .map_err(|e| format!("Failed to create Proxy object: {}", e))
}
