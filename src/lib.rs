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

pub fn f(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = h(s[i + (k as usize) - 1]);
    for (idx, v) in s.iter().skip(i).take((k - 1) as usize).enumerate() {
        out = out ^ h(*v).rotate_left(k - (idx as u32 + 1));
    }
    out
}

pub fn r(s: &[u8], i: usize, k: u32) -> u64 {
    let mut out = rc(s[i]);
    for (idx, v) in s.iter().skip(i + 1).take((k - 1) as usize).enumerate() {
        out = out ^ rc(*v).rotate_left(idx as u32 + 1);
    }
    out
}

fn nthash_init(s: &[u8], ksize: u8) -> u64 {
    u64::min(r(s, 0, ksize as u32), f(s, 0, ksize as u32))
}

pub fn nthash(seq: &[u8], ksize: u8) -> Vec<u64> {
    let mut out = Vec::with_capacity(seq.len() - (ksize as usize) + 1);
    let v = nthash_init(seq.get(0..ksize as usize).unwrap(), ksize);
    out.push(v);

    out
}

#[cfg(test)]
mod tests {
    use nthash;

    #[test]
    fn oracle_cmp() {
        assert_eq!(nthash("TGCAG".as_bytes(), 5), vec![0xbafa6628fc6dab7]);
        assert_eq!(nthash("ACGTC".as_bytes(), 5), vec![0x480202d34e8ebece]);
        assert_eq!(
            nthash("ACGTCGTCAGTCGATGCAGT".as_bytes(), 5),
            vec![
                0x480202d34e8ebece,
                0xa997bdd428b4c987,
                0x8c6d7aa40911b21d,
                0x5ddcb09590aafeec,
                0x25ff3ad6bc923826,
                0x9bda9a443560394a,
                0x82d449fbb3710cc2,
                0x1e926cf9780ab81d,
                0x2f6ed7ac26473a89,
                0xd1865ec7eb55b03b,
                0x38b57486189a8af7,
                0x1b235fddecacf38a,
                0x1eab5d82920fda13,
                0x2c8d1474673bdc5,
                0xbafa6628fc6dab7,
                0x14a33bbb28277bec,
            ]
        )
    }
}
