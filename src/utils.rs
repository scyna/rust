pub fn subscribe_url(url: &str) -> String {
    url.to_string().replace("/", ".")
}
