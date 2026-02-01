---
version: 0.1.0
updated: 2026-01-31
---

# Feature: Caesar Cipher (mod-23 byte shift)

**Status:** Complete\
**Started:** 2026-01-31\
**Completed:** 2026-01-31

---

## Problem

We need a first symmetric cipher to implement. It needs to be simple enough to understand completely —
every byte, every operation — while still being a real encrypt/decrypt cycle. The math is real;
the security is not. That's the point.

---

## Proposed Solution

A Caesar-style byte shift. The key is a single integer in the range 0–22 (mod 23). Encryption shifts
each plaintext byte forward by the key, wrapping at 256. Decryption shifts back.

- **Encrypt:** `ciphertext_byte = (plaintext_byte + key) mod 256`
- **Decrypt:** `plaintext_byte = (ciphertext_byte - key + 256) mod 256`

## Integration Points

This establishes the cipher module. Future ciphers will live alongside this one.
The CLI (see `02_cli.md`) is responsible for I/O, hex encoding, and arg parsing —
this module only deals in raw bytes.

### Key Behaviors

- `encrypt(plaintext_bytes, key)` → ciphertext bytes
- `decrypt(ciphertext_bytes, key)` → plaintext bytes
- Key must be in range 0–22; values outside this range are an error
- Round-trip: `decrypt(encrypt(input, key), key) == input` for any bytes and any valid key

---

## Success Criteria

- [x] Encrypt produces correct output for known inputs (verified by hand)
- [x] Decrypt is the exact inverse of encrypt
- [x] Round-trip holds for all byte values (0–255) and all keys (0–22)
- [x] Keys outside 0–22 are rejected

---

## Scope

### In Scope

- Encrypt and decrypt functions operating on byte slices
- Key validation (0–22 range)

### Out of Scope

- CLI, I/O, hex encoding — that's the CLI's job
- Breaking or attacking the cipher
- Multiple cipher types
- Anything asymmetric

---

## Important Considerations

- Key max is 22, but byte arithmetic wraps at 256. Byte 250 + key 22 = 16 (mod 256).
  Both encrypt and decrypt need to handle this correctly.
- Decryption adds 256 before subtracting the key to avoid underflow:
  `(byte + 256 - key) mod 256`. In Rust this means being mindful of u8 overflow.

---

## High-Level Todo

- [x] Scaffold and write tests for encrypt/decrypt logic
- [x] Implement Caesar cipher
- [x] Verify round-trip for all byte values and keys

---

## Notes & Context

### 2026-01-31 - Implementation

Implemented in [src/caesar.rs](../src/caesar.rs). Key decision: cast to `u16` before arithmetic to avoid overflow/underflow, then mod 256 and cast back to `u8`. Clean and straightforward — the entire cipher fits in ~25 lines of code.
