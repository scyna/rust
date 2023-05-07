pub fn subscribe_url(url: &str) -> String {
    "API".to_string() + &url.to_string().replace("/", ".")
}

pub fn publish_url(url: &str) -> String {
    "API".to_string() + &url.to_string().replace("/", ".")
}
