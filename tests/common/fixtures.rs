use serde_json::{Value, json};

pub fn valid_create_reservation_json() -> Value {
    json!({
        "full_name": "Temeu Teekkari",
        "address": "Konemiehentie 2",
        "email": "teemu.teekkari@aalto.fi",
        "phone_number": "+358931580001",
        "hetu": hetu::generate_by_pattern_with_any_checksum(&hetu::SsnPattern::default()).unwrap(),
        "start_time": "2026-10-12T15:00:00Z",
        "end_time": "2026-10-12T18:00:00Z",
        "internal": false,
        "used_outside_finland": false
    })
}
