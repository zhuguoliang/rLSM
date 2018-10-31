#![feature(test)]
extern crate bit_vec;
extern crate rand;
extern crate test;
use bit_vec::BitVec;
pub mod bloom_filter;
pub mod component;
pub mod helper;
pub mod global_conf;
use bloom_filter::IndexT;
use bloom_filter::BloomFilter;
static BLOOM_SIZE:IndexT=128;
macro_rules! say_hello {
    () => {
        println!("hello!" );
    };
}
fn main() {
    //use bloom_filter::set_bit;
    println!("Hello, world!");
    say_hello!();
    //let a:bloom_filter::bloom_filter::index_t = 1;
    let a:IndexT = 1;
    let mut b:BloomFilter = BloomFilter{
            hashes:3,
            size:BLOOM_SIZE,
            count:1,
            //table:BitVec::with_capacity(bloom_size)
            table:BitVec::from_elem(BLOOM_SIZE as usize,false)
        };
    b.set_bit(a);
    println!("{:?}",b);
    let test_keys:[IndexT;6]=[0,1,2,3,13,97];

    for i in 0..6{
        println!("Key: {0} , Hash1: {1}",test_keys[i],b.hash1(test_keys[i]));
        println!("Key: {0} , Hash2: {1}",test_keys[i],b.hash2(test_keys[i]));
    }
    //test bloom basics
       for k in 0..71 {
        b.bloom_add(k);
    }
    println!("Test of check: \n key present  {0} \n key absent {1} \n",b.bloom_check(1),b.bloom_check(71));   

    //test bloom occupency
    let _size_bits:IndexT=1000000;
    let mut b:BloomFilter=BloomFilter{
            hashes:7,
            size:_size_bits,
            count:1,
            //table:BitVec::with_capacity(bloom_size)
            table:BitVec::from_elem(_size_bits as usize,false)
    };
    for k in 1.._size_bits{
        b.bloom_add(k);
    }
    println!("number of bits set {0} / {1}\n",b.count,b.size);

}
