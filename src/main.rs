mod caesar;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn hex_decode(hex: &str) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(anyhow!("hex string must have even length"));
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|_| anyhow!("invalid hex character"))
        })
        .collect()
}

fn encrypt_command(plaintext: &str, key: u8) -> Result<String> {
    let ciphertext_bytes = caesar::encrypt(plaintext.as_bytes(), key)?;
    Ok(hex_encode(&ciphertext_bytes))
}

fn decrypt_command(hex_ciphertext: &str, key: u8) -> Result<String> {
    let ciphertext_bytes = hex_decode(hex_ciphertext)?;
    let plaintext_bytes = caesar::decrypt(&ciphertext_bytes, key)?;
    String::from_utf8(plaintext_bytes).map_err(|e| anyhow!("invalid UTF-8: {}", e))
}

#[derive(Parser)]
#[command(name = "shit")]
#[command(about = "Simple encryption/decryption using Caesar cipher")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encrypt plaintext and output hex-encoded ciphertext
    Encrypt {
        /// Encryption key (0-255)
        key: u8,
        /// Plaintext to encrypt
        plaintext: String,
    },
    /// Decrypt hex-encoded ciphertext and output plaintext
    Decrypt {
        /// Decryption key (0-255)
        key: u8,
        /// Hex-encoded ciphertext to decrypt
        ciphertext: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Encrypt { key, plaintext } => encrypt_command(&plaintext, key),
        Command::Decrypt { key, ciphertext } => decrypt_command(&ciphertext, key),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run<I, T>(args: I) -> Result<String>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let args: Vec<String> = args.into_iter().map(|s| s.into()).collect();
        let cli = Cli::try_parse_from(args)?;

        match cli.command {
            Command::Encrypt { key, plaintext } => encrypt_command(&plaintext, key),
            Command::Decrypt { key, ciphertext } => decrypt_command(&ciphertext, key),
        }
    }

    #[test]
    fn test_hex_encode() {
        // Why: Ciphertext is raw bytes, some non-printable. Hex encoding makes every byte
        // visible as two characters (0-9, a-f). This is how we'll display ciphertext to users.
        // Example: byte 109 (0x6d) becomes the string "6d"
        let bytes = vec![109, 106, 113];
        let result = hex_encode(&bytes);
        assert_eq!(result, "6d6a71");
    }

    #[test]
    fn test_hex_decode_valid() {
        // Why: Users provide hex strings on the command line. We need to convert them back
        // to bytes before decrypting. This is the inverse of hex_encode.
        let result = hex_decode("6d6a71").unwrap();
        assert_eq!(result, vec![109, 106, 113]);
    }

    #[test]
    fn test_hex_decode_odd_length() {
        // Why: Each byte is exactly 2 hex characters. An odd-length string can't represent
        // whole bytes, so we reject it with a clear error instead of silently truncating.
        let result = hex_decode("6d6a7");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "hex string must have even length");
    }

    #[test]
    fn test_hex_decode_invalid_char() {
        // Why: Only 0-9 and a-f are valid hex. 'g' isn't valid, so we need to catch it
        // and tell the user what went wrong instead of panicking or producing garbage.
        let result = hex_decode("6d6g71");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid hex character"));
    }

    #[test]
    fn test_encrypt_command() {
        // Why: This is the end-to-end encrypt path. String in, hex out. It combines:
        // string → bytes, encrypt bytes, bytes → hex. This is what `shit encrypt` does.
        let result = encrypt_command("hello", 5).unwrap();
        assert_eq!(result, "6d6a717174");
    }

    #[test]
    fn test_decrypt_command() {
        // Why: The inverse of encrypt_command. Hex in, string out. It combines:
        // hex → bytes, decrypt bytes, bytes → string. This is what `shit decrypt` does.
        let result = decrypt_command("6d6a717174", 5).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_round_trip_via_cli() {
        // Why: The core contract of symmetric encryption via the CLI: encrypt then decrypt
        // gets you back where you started. This tests the entire pipeline end-to-end.
        let plaintext = "the quick brown fox";
        let key = 13;
        let encrypted = encrypt_command(plaintext, key).unwrap();
        let decrypted = decrypt_command(&encrypted, key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    // CLI argument parsing tests

    #[test]
    fn test_cli_encrypt() {
        // Why: The CLI needs to parse "encrypt <key> <plaintext>" and route it to
        // encrypt_command(). This verifies the argument parsing works and produces
        // the correct hex output. The user types plaintext, gets hex ciphertext back.
        let args = vec!["shit", "encrypt", "5", "hello"];
        let result = run(args.into_iter().map(String::from)).unwrap();
        assert_eq!(result, "6d6a717174");
    }

    #[test]
    fn test_cli_decrypt() {
        // Why: The inverse of encrypt. CLI parses "decrypt <key> <hex>" and routes to
        // decrypt_command(). Verifies hex input is correctly decoded and decrypted back
        // to the original plaintext string.
        let args = vec!["shit", "decrypt", "5", "6d6a717174"];
        let result = run(args.into_iter().map(String::from)).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_cli_missing_args() {
        // Why: Users will inevitably forget arguments. The CLI should catch this and
        // return a clear error, not panic or produce garbage. Clap handles this for us,
        // but we verify it works as expected.
        let args = vec!["shit", "encrypt", "5"];
        let result = run(args.into_iter().map(String::from));
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_invalid_key() {
        // Why: The key must be a number (u8). If the user types "abc", clap's parser
        // should reject it with a clear error before we ever call the cipher. This tests
        // that type validation happens at the CLI layer.
        let args = vec!["shit", "encrypt", "abc", "hello"];
        let result = run(args.into_iter().map(String::from));
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_invalid_hex() {
        // Why: Decrypt expects valid hex input. If the user provides malformed hex
        // (odd length, invalid characters like 'g'), hex_decode will fail. This verifies
        // that error propagates up through the CLI layer with a clear message.
        let args = vec!["shit", "decrypt", "5", "6d6g71"];
        let result = run(args.into_iter().map(String::from));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid hex"));
    }
}
