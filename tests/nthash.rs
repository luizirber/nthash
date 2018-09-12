extern crate nthash;

use nthash::{nthash, NtHashIterator};

#[test]
fn oracle_cmp() {
    assert_eq!(nthash(b"TGCAG", 5), vec![0xbafa6728fc6dabf]);
    assert_eq!(nthash(b"ACGTC", 5), vec![0x480202d54e8ebecd]);
    assert_eq!(
        nthash(b"ACGTCGTCAGTCGATGCAGT", 5),
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
        let iter = NtHashIterator::new(seq, ksize);
        println!("{:?}", s);
        assert_eq!(nthash(seq, ksize), iter.collect::<Vec<u64>>());
    }
}
