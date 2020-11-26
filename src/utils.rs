use url::form_urlencoded::byte_serialize;

pub fn url_encode(text: &str) -> String {
    let encoded_text: String = byte_serialize(text.as_bytes()).collect();
    encoded_text
        .replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}
