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

#[cfg(test)]
mod tests {
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
}
