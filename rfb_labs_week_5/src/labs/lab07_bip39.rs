//! Lab 07 — validate BIP39 recovery words and derive seeds safely.

use bip39::Mnemonic;

use crate::model::{MnemonicReport, PassphraseComparison};
use crate::{LabError, LabResult};

/// Validate an English mnemonic and report its entropy/checksum structure.
pub fn inspect_mnemonic(mnemonic: &str) -> LabResult<MnemonicReport> {
    let parsed: Mnemonic = mnemonic
        .parse()
        .map_err(|e: bip39::Error| LabError::InvalidMnemonic(e.to_string()))?;

    let word_count = parsed.word_count();
    // BIP39: ENT + CS = word_count * 11, CS = ENT / 32
    // => ENT * (1 + 1/32) = word_count * 11 => ENT = word_count * 11 * 32 / 33
    let entropy_bits = word_count * 11 * 32 / 33;
    let checksum_bits = entropy_bits / 32;

    Ok(MnemonicReport {
        word_count,
        entropy_bits,
        checksum_bits,
    })
}

/// Derive the 512-bit BIP39 seed from words plus an optional passphrase.
pub fn mnemonic_seed_hex(mnemonic: &str, passphrase: &str) -> LabResult<String> {
    let parsed: Mnemonic = mnemonic
        .parse()
        .map_err(|e: bip39::Error| LabError::InvalidMnemonic(e.to_string()))?;
    let seed = parsed.to_seed(passphrase);
    Ok(hex::encode(seed))
}

/// Demonstrate that the same words with a different passphrase make a different seed.
pub fn compare_passphrases(
    mnemonic: &str,
    protected_passphrase: &str,
) -> LabResult<PassphraseComparison> {
    let empty_passphrase_seed_hex = mnemonic_seed_hex(mnemonic, "")?;
    let protected_seed_hex = mnemonic_seed_hex(mnemonic, protected_passphrase)?;
    let seeds_differ = empty_passphrase_seed_hex != protected_seed_hex;

    Ok(PassphraseComparison {
        empty_passphrase_seed_hex,
        protected_seed_hex,
        seeds_differ,
    })
}

/// Recognize the public BIP39 test mnemonic used in the class labs.
pub fn is_public_test_mnemonic(mnemonic: &str) -> bool {
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    if words.len() != 12 {
        return false;
    }
    for word in words.iter().take(11) {
        if *word != "abandon" {
            return false;
        }
    }
    words[11] == "about"
}
