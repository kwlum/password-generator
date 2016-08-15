extern crate twox_hash;
extern crate rand;

use std::fs;
use std::hash::Hasher;
use std::io;
use std::io::Read;

use rand::isaac::Isaac64Rng;
use rand::{Rng, SeedableRng};

const ALPHA: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789{}[]-_=+()@#$^&<>?";

fn hash(p: &[u8], l: u64) -> u64 {
    let mut h = twox_hash::XxHash::with_seed(l);
    h.write(p);
    h.finish()
}

fn hash_file<T>(p: T, l: u64) -> Result<u64, io::Error>
    where T: AsRef<std::path::Path>
{
    let mut h = twox_hash::XxHash::with_seed(l);
    let mut buf = [0u8; 512];
    let mut f = try!(fs::File::open(p));

    loop {
        let sz = try!(f.read(&mut buf));

        if sz > 0 {
            h.write(&buf[..sz]);
        } else {
            break;
        }
    }

    Ok(h.finish())
}

fn encode(c: &[u8], v: &[u8], l: usize) -> String {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut a = 0u32;
    let c_l = c.len() as u32;
    let mut s: Vec<u8> = Vec::with_capacity(l);

    while i < l {
        a += v[j] as u32;

        while a >= c_l {
            let r = a % c_l;
            s.push(c[r as usize]);
            a /= c_l;
            i += 1;
        }

        j += 1;
    }

    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    let mut args = std::env::args();

    if args.len() < 4 {
        println!("Usage: pag <length> <name> <files>\n\nEXAMPLE:\nGenerate a password with 16 \
                  characters for github.com.\npag 16 github.com mypets.jpg holiday.pdf\n");
        return;
    }

    args.next();
    let length: u64 = std::str::FromStr::from_str(&args.next().unwrap()).unwrap();
    let name = args.next().unwrap();
    let mut file_hash = 0u64;

    while let Some(filename) = args.next() {
        let a = hash_file(filename, length).unwrap();
        file_hash ^= a;
    }

    let hs = file_hash ^ hash(name.as_bytes(), length);
    let mut rr = Isaac64Rng::from_seed(&[hs]);
    let mut buf = [0u8; 64];
    rr.fill_bytes(&mut buf);
    let s = encode(ALPHA, &buf, length as usize);
    println!("{}", s);
}
