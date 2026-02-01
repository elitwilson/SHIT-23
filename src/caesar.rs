use anyhow::{anyhow, Result};

const MAX_KEY: u8 = 22;

fn validate_key(key: u8) -> Result<()> {
    if key > MAX_KEY {
        Err(anyhow!("key must be in range 0-{}", MAX_KEY))
    } else {
        Ok(())
    }
}

pub fn encrypt(plaintext: &[u8], key: u8) -> Result<Vec<u8>> {
    validate_key(key)?;
    // Cast to u16 so addition can't overflow a u8, then mod 256 brings it back into range.
    Ok(plaintext
        .iter()
        .map(|&byte| {
            let shifted = (byte as u16 + key as u16) % 256;
            shifted as u8
        })
        .collect())
}

pub fn decrypt(ciphertext: &[u8], key: u8) -> Result<Vec<u8>> {
    validate_key(key)?;
    // Add 256 before subtracting so we never go negative, then mod 256 wraps back into range.
    Ok(ciphertext
        .iter()
        .map(|&byte| {
            let shifted = (byte as u16 + 256 - key as u16) % 256;
            shifted as u8
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_known_input() {
        // h(104)+5=109, e(101)+5=106, l(108)+5=113, l(108)+5=113, o(111)+5=116
        let plaintext = b"hello";
        let result = encrypt(plaintext, 5).unwrap();
        assert_eq!(result, vec![109, 106, 113, 113, 116]);
    }

    #[test]
    fn test_decrypt_known_input() {
        let ciphertext = vec![109, 106, 113, 113, 116];
        let result = decrypt(&ciphertext, 5).unwrap();
        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_round_trip() {
        let plaintext = b"the quick brown fox";
        let encrypted = encrypt(plaintext, 13).unwrap();
        let decrypted = decrypt(&encrypted, 13).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_round_trip_wrapping() {
        // Bytes 250–255 with key 10 will all wrap past 256 on encrypt.
        // Decrypt has to add 256 before subtracting to avoid underflow.
        let plaintext: Vec<u8> = (250..=255).collect();
        let encrypted = encrypt(&plaintext, 10).unwrap();
        let decrypted = decrypt(&encrypted, 10).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_invalid_key_rejected() {
        let data = b"test";
        assert!(encrypt(data, 23).is_err());
        assert!(encrypt(data, 255).is_err());
        assert!(decrypt(data, 23).is_err());
        assert!(decrypt(data, 255).is_err());
    }
}
