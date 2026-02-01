#[cfg(test)]
mod tests {
    #[test]
    fn test_encrypt_known_input() {
        // encrypt("hello", key=5) should produce [109, 106, 113, 113, 116]
        // h(104)+5=109, e(101)+5=106, l(108)+5=113, l(108)+5=113, o(111)+5=116
        assert!(false);
    }

    #[test]
    fn test_decrypt_known_input() {
        // decrypt([109, 106, 113, 113, 116], key=5) should produce "hello"
        assert!(false);
    }

    #[test]
    fn test_round_trip() {
        // encrypt then decrypt with same key returns original input
        assert!(false);
    }

    #[test]
    fn test_round_trip_wrapping() {
        // round-trip works when byte arithmetic wraps at 256
        // e.g., byte 250 + key 10 = 4 (mod 256), then 4 - 10 + 256 = 250
        assert!(false);
    }

    #[test]
    fn test_invalid_key_rejected() {
        // keys outside 0–22 should be rejected
        assert!(false);
    }
}
