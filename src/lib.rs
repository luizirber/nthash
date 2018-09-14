//! ntHash is a hash function tuned for genomic data.
//! It performs best when calculating hash values for adjacent k-mers in
//! an input sequence, operating an order of magnitude faster than the best
//! performing alternatives in typical use cases.
//!
//! [Scientific article with more details](https://doi.org/10.1093/bioinformatics/btw397)
//!
//! [Original implementation in C++](https://github.com/bcgsc/ntHash/)
//!
//! This crate is based on ntHash [1.0.4](https://github.com/bcgsc/ntHash/releases/tag/v1.0.4).

#[inline(always)]
fn h(c: u8) -> u64 {
    match c {
        b'A' => 0x3c8b_fbb3_95c6_0474,
        b'C' => 0x3193_c185_62a0_2b4c,
        b'G' => 0x2032_3ed0_8257_2324,
        b'T' => 0x2955_49f5_4be2_4456,
        b'N' => 0,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn rc(nt: u8) -> u64 {
    match nt {
        b'A' => 0x2955_49f5_4be2_4456,
        b'C' => 0x2032_3ed0_8257_2324,
        b'G' => 0x3193_c185_62a0_2b4c,
        b'T' => 0x3c8b_fbb3_95c6_0474,
        b'N' => 0,
        _ => unreachable!(),
    }
}

/// Calculate the hash for a k-mer in the forward strand of a sequence.
///
/// This is a low level function, more useful for debugging than for direct use.
///
/// ```
///    use nthash::ntf64;
///    let fh = ntf64(b"TGCAG", 0, 5);
///    assert_eq!(fh, 0xbafa6728fc6dabf);
/// ```
pub fn ntf64(s: &[u8], i: usize, k: usize) -> u64 {
    let mut out = h(s[i + k - 1]);
    for (idx, v) in s.iter().skip(i).take(k - 1).enumerate() {
        out ^= h(*v).rotate_left((k - idx - 1) as u32);
    }
    out
}

/// Calculate the hash for a k-mer in the reverse strand of a sequence.
///
/// This is a low level function, more useful for debugging than for direct use.
///
/// ```
///    use nthash::ntr64;
///    let rh = ntr64(b"TGCAG", 0, 5);
///    assert_eq!(rh, 0x8cf2d4072cca480e);
/// ```
pub fn ntr64(s: &[u8], i: usize, k: usize) -> u64 {
    let mut out = rc(s[i]);
    for (idx, v) in s.iter().skip(i + 1).take(k - 1).enumerate() {
        out ^= rc(*v).rotate_left(idx as u32 + 1);
    }
    out
}

/// Calculate the canonical hash (minimum hash value between the forward
/// and reverse strands in a sequence).
///
/// This is a low level function, more useful for debugging than for direct use.
///
/// ```
///    use nthash::ntc64;
///    let hash = ntc64(b"TGCAG", 0, 5);
///    assert_eq!(hash, 0xbafa6728fc6dabf);
/// ```
pub fn ntc64(s: &[u8], i: usize, ksize: usize) -> u64 {
    u64::min(ntr64(s, i, ksize), ntf64(s, i, ksize))
}

/// Takes a sequence and ksize and returns the canonical hashes for each k-mer
/// in a Vec. This doesn't benefit from the rolling hash properties of ntHash,
/// serving more for correctness check for the NtHashIterator.
pub fn nthash(seq: &[u8], ksize: usize) -> Vec<u64> {
    seq.windows(ksize).map(|x| ntc64(x, 0, ksize)).collect()
}

/// An efficient iterator for calculating hashes for genomic sequences.
///
/// Since it implements the `Iterator` trait it also
/// exposes many other useful methods. In this example we use `collect` to
/// generate all hashes and put them in a `Vec<u64>`.
/// ```
///     use nthash::NtHashIterator;
///
///     let seq = b"ACTGC";
///     let iter = NtHashIterator::new(seq, 3);
///     let hashes: Vec<u64> = iter.collect();
///     assert_eq!(hashes,
///                vec![0x9b1eda9a185413ce, 0x9f6acfa2235b86fc, 0xd4a29bf149877c5c]);
/// ```
/// or, in one line:
/// ```
///     use nthash::NtHashIterator;
///
///     assert_eq!(NtHashIterator::new(b"ACTGC", 3).collect::<Vec<u64>>(),
///                vec![0x9b1eda9a185413ce, 0x9f6acfa2235b86fc, 0xd4a29bf149877c5c]);
/// ```
pub struct NtHashIterator<'a> {
    seq: &'a [u8],
    k: usize,
    fh: u64,
    rh: u64,
    current_idx: usize,
    max_idx: usize,
}

impl<'a> NtHashIterator<'a> {
    /// Creates a new NtHashIterator with internal state properly initialized.
    pub fn new(seq: &'a [u8], k: usize) -> NtHashIterator<'a> {
        let mut fh = 0;
        for (i, v) in seq[0..k].iter().enumerate() {
            fh ^= h(*v).rotate_left((k - i - 1) as u32);
        }

        let mut rh = 0;
        for (i, v) in seq[0..k].iter().rev().enumerate() {
            rh ^= rc(*v).rotate_left((k - i - 1) as u32);
        }

        NtHashIterator {
            seq,
            k,
            fh,
            rh,
            current_idx: 0,
            max_idx: seq.len() - k + 1,
        }
    }
}

impl<'a> Iterator for NtHashIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current_idx == self.max_idx {
            return None;
        };

        if self.current_idx != 0 {
            let i = self.current_idx - 1;
            let seqi = self.seq[i];
            let seqk = self.seq[i + self.k];

            self.fh = self.fh.rotate_left(1) ^ h(seqi).rotate_left(self.k as u32) ^ h(seqk);

            self.rh = self.rh.rotate_right(1)
                ^ rc(seqi).rotate_right(1)
                ^ rc(seqk).rotate_left(self.k as u32 - 1);
        }

        self.current_idx += 1;
        Some(u64::min(self.rh, self.fh))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.max_idx, Some(self.max_idx))
    }
}
