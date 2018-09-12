fn h(c: u8) -> u64 {
    match c {
        b'A' => 0x3c8bfbb395c60474,
        b'C' => 0x3193c18562a02b4c,
        b'G' => 0x20323ed082572324,
        b'T' => 0x295549f54be24456,
        b'N' => 0,
        _ => unreachable!(),
    }
}

fn rc(nt: u8) -> u64 {
    match nt {
        b'A' => 0x295549f54be24456,
        b'C' => 0x20323ed082572324,
        b'G' => 0x3193c18562a02b4c,
        b'T' => 0x3c8bfbb395c60474,
        b'N' => 0,
        _ => unreachable!(),
    }
}

pub fn ntf64(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = h(s[i + (k as usize) - 1]);
    for (idx, v) in s.iter().skip(i).take((k - 1) as usize).enumerate() {
        out = out ^ h(*v).rotate_left(k - (idx as u32 + 1));
    }
    out
}

pub fn ntr64(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = rc(s[i]);
    for (idx, v) in s.iter().skip(i + 1).take((k - 1) as usize).enumerate() {
        out = out ^ rc(*v).rotate_left(idx as u32 + 1);
    }
    out
}

fn ntc64(s: &[u8], ksize: u8) -> u64 {
    u64::min(ntr64(s, 0, ksize as u32), ntf64(s, 0, ksize as u32))
}

pub fn nthash(seq: &[u8], ksize: u8) -> Vec<u64> {
    seq.windows(ksize as usize)
        .map(|x| ntc64(x, ksize))
        .collect()
}

struct ntHashIterator<'a> {
    seq: &'a [u8],
    ksize: u8,
    fh: u64,
    rh: u64,
    current_idx: usize,
    max_idx: usize,
}

impl<'a> ntHashIterator<'a> {
    fn new(seq: &'a [u8], ksize: u8) -> ntHashIterator<'a> {
        ntHashIterator {
            seq,
            ksize,
            fh: 0,
            rh: 0,
            current_idx: 0,
            max_idx: seq.len() - ksize as usize + 1,
        }
    }
}

impl<'a> Iterator for ntHashIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current_idx == self.max_idx {
            None
        } else if self.current_idx == 0 {
            self.fh = ntf64(self.seq, self.current_idx, self.ksize as u32);
            self.rh = ntr64(self.seq, self.current_idx, self.ksize as u32);
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

#[cfg(test)]
mod tests {
    use ntHashIterator;
    use nthash;

    #[test]
    fn oracle_cmp() {
        assert_eq!(nthash("TGCAG".as_bytes(), 5), vec![0xbafa6728fc6dabf]);
        assert_eq!(nthash("ACGTC".as_bytes(), 5), vec![0x480202d54e8ebecd]);
        assert_eq!(
            nthash("ACGTCGTCAGTCGATGCAGT".as_bytes(), 5),
            vec![
                0x480202d54e8ebecd,
                0xa997bdc628b4c98e,
                0x8c6d7ab20911b216,
                0x5ddcb09390aafeef,
                0x25ff3ac4bc92382f,
                0x9bda9a5c35603946,
                0x82d449e5b3710ccd,
                0x1e926ce7780ab812,
                0x2f6ed7b226473a86,
                0xd1865edfeb55b037,
                0x38b57494189a8afe,
                0x1b235fc5ecacf386,
                0x1eab5d82920fda13,
                0x2c8d1574673bdcd,
                0xbafa6728fc6dabf,
                0x14a33bb928277bed,
            ]
        )
    }

    #[test]
    fn iter_cmp() {
        let ksize = 5;
        for s in &vec!["TGCAG", "ACGTC", "ACGTCGTCAGTCGATGCAGT"] {
            let seq = s.as_bytes();
            let iter = ntHashIterator::new(seq, ksize);
            println!("{:?}", s);
            assert_eq!(nthash(seq, ksize), iter.collect::<Vec<u64>>());
        }
    }
}
