use percent_encoding::percent_decode;

pub fn decode_uri(encoded: &str) -> String {
    percent_decode(encoded.as_bytes())
        .decode_utf8()
        .unwrap_or_default()
        .to_string()
}
