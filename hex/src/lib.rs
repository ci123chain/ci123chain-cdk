pub use core::fmt::Write;

pub fn encode(raw: &[u8]) -> String {
    let mut s = String::with_capacity(raw.len() * 2);
    for b in raw.iter() {
        write!(s, "{:02x}", *b).unwrap();
    }
    s
}

pub fn decode(raw: &[u8]) -> Result<Vec<u8>, &str> {
    if raw.len() < 2 || raw.len() % 2 == 1 {
        return Err("unexpected hex encoding length");
    }

    let e = "unexpected hex encoding char";

    let mut result = vec![0u8; raw.len() / 2];
    let (mut i, mut j) = (0, 1);
    while j < raw.len() {
        let (a, b);

        if let Some(res) = from_hex_u8(raw[j - 1]) {
            a = res;
        } else {
            return Err(e);
        }

        if let Some(res) = from_hex_u8(raw[j]) {
            b = res;
        } else {
            return Err(e);
        }

        result[i] = (a << 4) | b;

        i += 1;
        j += 2;
    }

    Ok(result)
}

pub(crate) fn from_hex_u8(c: u8) -> Option<u8> {
    if '0' as u8 <= c && c <= '9' as u8 {
        Some(c - '0' as u8)
    } else if 'a' as u8 <= c && c <= 'f' as u8 {
        Some(c - 'a' as u8 + 10)
    } else if 'A' as u8 <= c && c <= 'F' as u8 {
        Some(c - 'A' as u8 + 10)
    } else {
        None
    }
}
