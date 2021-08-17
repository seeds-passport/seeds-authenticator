// Use as name_bytes_to_u64("string".bytes())
// result needs to be unwraped
pub fn name_bytes_to_u64<I>(mut iter: I) -> Result<u64, String>
where
    I: Iterator<Item = u8>,
{
    let mut value = 0_u64;
    let mut len = 0_u64;

    // Loop through up to 12 characters
    while let Some(c) = iter.next() {
        let v = char_to_value(c).ok_or_else(|| "Error-Value".to_string())?;
        value <<= 5;
        value |= u64::from(v);
        len += 1;

        if len == 12 {
            break;
        }
    }

    if len == 0 {
        return Ok(0);
    }

    value <<= 4 + 5 * (12 - len);

    // Check if we have a 13th character
    if let Some(c) = iter.next() {
        let v = char_to_value(c).ok_or_else(|| "Error-Value".to_string())?;

        // The 13th character can only be 4 bits, it has to be between letters
        // 'a' to 'j'
        if v > 0x0F {
            return Err("Error-Value".to_string());
        }

        value |= u64::from(v);

        // Check if we have more than 13 characters
        if iter.next().is_some() {
            return Err("Name is too long.".to_string());
        }
    }

    Ok(value)
}

fn char_to_value(c: u8) -> Option<u8> {
    if c == b'.' {
        Some(0)
    } else if c >= b'1' && c <= b'5' {
        Some(c - b'1' + 1)
    } else if c >= b'a' && c <= b'z' {
        Some(c - b'a' + 6)
    } else {
        None
    }
}