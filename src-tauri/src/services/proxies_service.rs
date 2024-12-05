pub fn get_random_proxy(proxies: &[String]) -> Option<&String> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    proxies.choose(&mut rng)
}

pub fn format_proxy(proxy: &str) -> Result<String, String> {
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

    Ok(format!("http://{}:{}@{}:{}", username, password, host, port))
}
