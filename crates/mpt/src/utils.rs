use std::{cmp, iter};

/// Converts a byte slice into a vector of nibbles.
///
/// A nibble is 4 bits or half of an 8-bit byte. This function takes each byte from the
/// input slice, splits it into two nibbles, and appends them to the resulting vector.
pub fn to_nibs(slice: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(2 * slice.len());
    for byte in slice {
        result.push(byte >> 4);
        result.push(byte & 0xf);
    }
    result
}

/// Encodes a slice of nibbles into a vector of bytes, with an additional prefix to
/// indicate the type of node (leaf or extension).
///
/// The function starts by determining the type of node based on the `is_leaf` parameter.
/// If the node is a leaf, the prefix is set to `0x20`. If the length of the nibbles is
/// odd, the prefix is adjusted and the first nibble is incorporated into it.
///
/// The remaining nibbles are then combined into bytes, with each pair of nibbles forming
/// a single byte. The resulting vector starts with the prefix, followed by the encoded
/// bytes.
pub fn to_encoded_path(mut nibs: &[u8], is_leaf: bool) -> Vec<u8> {
    let mut prefix = (is_leaf as u8) * 0x20;
    if nibs.len() % 2 != 0 {
        prefix += 0x10 + nibs[0];
        nibs = &nibs[1..];
    }
    iter::once(prefix).chain(nibs.chunks_exact(2).map(|byte| (byte[0] << 4) + byte[1])).collect()
}

/// Returns the length of the common prefix.
pub fn lcp(a: &[u8], b: &[u8]) -> usize {
    for (i, (a, b)) in iter::zip(a, b).enumerate() {
        if a != b {
            return i;
        }
    }
    cmp::min(a.len(), b.len())
}

pub fn prefix_nibs(prefix: &[u8]) -> Vec<u8> {
    let (extension, tail) = prefix.split_first().unwrap();
    // the first bit of the first nibble denotes the parity
    let is_odd = extension & (1 << 4) != 0;

    let mut result = Vec::with_capacity(2 * tail.len() + is_odd as usize);
    // for odd lengths, the second nibble contains the first element
    if is_odd {
        result.push(extension & 0xf);
    }
    for nib in tail {
        result.push(nib >> 4);
        result.push(nib & 0xf);
    }
    result
}
