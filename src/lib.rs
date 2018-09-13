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

pub fn ntf64(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = h(s[i + (k as usize) - 1]);
    for (idx, v) in s.iter().skip(i).take((k - 1) as usize).enumerate() {
        out ^= h(*v).rotate_left(k - (idx as u32 + 1));
    }
    out
}

pub fn ntr64(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = rc(s[i]);
    for (idx, v) in s.iter().skip(i + 1).take((k - 1) as usize).enumerate() {
        out ^= rc(*v).rotate_left(idx as u32 + 1);
    }
    out
}

pub fn ntc64(s: &[u8], i: usize, ksize: u8) -> u64 {
    u64::min(ntr64(s, i, u32::from(ksize)), ntf64(s, i, u32::from(ksize)))
}

pub fn nthash(seq: &[u8], ksize: u8) -> Vec<u64> {
    seq.windows(ksize as usize)
        .map(|x| ntc64(x, 0, ksize))
        .collect()
}

pub struct NtHashIterator<'a> {
    seq: &'a [u8],
    ksize: u8,
    fh: u64,
    rh: u64,
    current_idx: usize,
    max_idx: usize,
}

impl<'a> NtHashIterator<'a> {
    pub fn new(seq: &'a [u8], ksize: u8) -> NtHashIterator<'a> {
        NtHashIterator {
            seq,
            ksize,
            fh: 0,
            rh: 0,
            current_idx: 0,
            max_idx: seq.len() - ksize as usize + 1,
        }
    }
}

impl<'a> Iterator for NtHashIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current_idx == self.max_idx {
            None
        } else if self.current_idx == 0 {
            self.fh = ntf64(self.seq, self.current_idx, u32::from(self.ksize));
            self.rh = ntr64(self.seq, self.current_idx, u32::from(self.ksize));
            self.current_idx += 1;
            Some(u64::min(self.rh, self.fh))
        } else {
            let i = self.current_idx - 1;
            let k = self.ksize as usize;

            self.fh =
                self.fh.rotate_left(1) ^ h(self.seq[i]).rotate_left(k as u32) ^ h(self.seq[i + k]);

            self.rh = self.rh.rotate_right(1)
                ^ rc(self.seq[i]).rotate_right(1)
                ^ rc(self.seq[i + k]).rotate_left(k as u32 - 1);

            self.current_idx += 1;
            Some(u64::min(self.rh, self.fh))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.seq.len() - self.ksize as usize + 1;
        (n, Some(n))
    }
}
