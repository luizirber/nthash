extern crate nthash;
#[macro_use]
extern crate quickcheck;
extern crate rand;

use quickcheck::{Arbitrary, Gen};
use rand::Rng;

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
    );
    assert_eq!(
        nthash(b"ACGTCGANNGTA", 5),
        vec![
            0x480202d54e8ebecd,
            0xa997bdc628b4c98e,
            0xd1865edfeb55b037,
            0xe0159f5a89f59b7b,
            0xe6409a0f689e64e4,
            0x7a054a39df661723,
            0x6d74fee702835974,
            0xb74444dd9a94cbf3,
        ]
    );
}

#[test]
fn iter_cmp() {
    let ksize = 5;
    for s in &vec!["TGCAG", "ACGTC", "ACGTCGTCAGTCGATGCAGT", "ACGTCGANNGTA"] {
        let seq = s.as_bytes();
        let iter = NtHashIterator::new(seq, ksize);
        println!("{:?}", s);
        assert_eq!(nthash(seq, ksize), iter.collect::<Vec<u64>>());
    }
}

#[derive(Clone, Debug)]
struct Seq(String);

impl Arbitrary for Seq {
    fn arbitrary<G: Gen>(g: &mut G) -> Seq {
        let choices = ['A', 'C', 'G', 'T', 'N'];
        let size = {
            let s = g.size();
            g.gen_range(0, s)
        };
        let mut s = String::with_capacity(size);
        for _ in 0..size {
            s.push(*g.choose(&choices).expect("Not a valid nucleotide"));
        }
        Seq { 0: s }
    }
}

quickcheck! {
  fn oracle_quickcheck(s: Seq) -> bool {
     let seq = s.0.as_bytes();
     (1..(seq.len())).all(|ksize| {
       let iter = NtHashIterator::new(seq, ksize as u8);
       nthash(seq, ksize as u8) == iter.collect::<Vec<u64>>()
     })
  }
}
