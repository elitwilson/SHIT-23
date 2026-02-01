---
version: 0.1.0
updated: 2026-01-31
---

# Feature: CLI (encrypt/decrypt commands)

**Status:** Not Started\
**Started:** —\
**Completed:** —

---

## Problem

The Caesar cipher is pure math — bytes in, bytes out. We need a way for a human to actually
use it from a terminal. That means parsing arguments, converting between text and bytes,
and representing ciphertext in a readable format.

---

## Proposed Solution

A binary called `shit` with two subcommands. Ciphertext is hex-encoded so every byte
is visible and nothing is hidden.

- `shit encrypt <key> <plaintext>` → prints hex-encoded ciphertext to stdout
- `shit decrypt <key> <hex-ciphertext>` → prints plaintext to stdout

## Integration Points

- Uses the Caesar cipher module (`01_caesar_cipher.md`) for the actual encrypt/decrypt logic
- Establishes the CLI pattern and subcommand structure that future ciphers will plug into
- Hex encoding/decoding lives here — it's an I/O concern, not a cipher concern

### Key Behaviors

- Parses `<key>` as an integer and passes it to the cipher (cipher handles range validation)
- Converts plaintext string to bytes before encrypting
- Hex-encodes ciphertext bytes for output (two hex chars per byte)
- Parses hex input back to bytes before decrypting
- Converts decrypted bytes back to a string for output
- Reports errors clearly: missing arguments, non-numeric key, invalid hex

---

## Success Criteria

- [ ] `shit encrypt 5 "hello"` produces correct hex output
- [ ] `shit decrypt 5 <that hex>` produces `hello`
- [ ] End-to-end round-trip works from the command line
- [ ] Bad input (missing args, garbage key, invalid hex) fails with a clear error message

---

## Scope

### In Scope

- Binary with `encrypt` and `decrypt` subcommands
- Argument parsing and validation (types, count)
- Hex encoding (plaintext bytes → hex string) and decoding (hex string → bytes)
- Error messages for bad input

### Out of Scope

- File I/O (args only for now)
- Key range validation — that's the cipher's job
- Breaking or attacking
- Supporting multiple cipher types (just Caesar for now)

---

## Important Considerations

- Hex output is two characters per byte — ciphertext is always exactly 2× the length
  of the plaintext in characters. Useful for sanity-checking output.
- Hex decoding on decrypt needs to reject odd-length strings and non-hex characters.
- The CLI doesn't need to know what a valid key range is. It parses the key as an integer
  and hands it to the cipher. If the cipher rejects it, the CLI surfaces that error.

---

## High-Level Todo

- [ ] Scaffold and write tests for hex encoding/decoding
- [ ] Implement CLI argument parsing and subcommands
- [ ] Wire up Caesar cipher encrypt/decrypt
- [ ] End-to-end verification
