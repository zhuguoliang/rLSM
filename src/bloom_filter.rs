extern crate bit_vec;
extern crate rand;
//use std::rand::Rng;
use rand::prelude::*;
use bit_vec::BitVec;

pub type IndexT= u64;
#[derive(Debug)]
pub struct BloomFilter{
    pub hashes:IndexT,
    pub size:IndexT,
    pub count:IndexT,
    //pub table:&'a IndexT
    pub table:bit_vec::BitVec
}

impl BloomFilter {

    pub fn set_bit(&mut self, i:IndexT)
    {
        assert!(i<self.size);

        self.table.set(i as usize,true);
    }

    pub fn get_bit(&mut self,i:IndexT ) -> Option<bool>
    {
        assert!(i<self.size);

        self.table.get(i as usize)
    }

    //from https://gist.github.com/badboy/6267743
    pub fn hash1(&mut self, k:IndexT) -> IndexT{
        let mut key = k;
        //key = (!key) + (key << 21); // key = (key << 21) - key - 1;
        key = (!key).wrapping_add(key << 21); // key = (key << 21) - key - 1;
        key = key ^ (key >> 24);
        key = (key.wrapping_add(key << 3)).wrapping_add(key << 8); // key * 265
        key = key ^ (key >> 14);
        key = (key.wrapping_add(key << 2)).wrapping_add(key << 4); // key * 21
        key = key ^ (key >> 28);
        key = key.wrapping_add(key << 31);
        key
    }

    // suggested by Knuth in "The Art of Computer Programming"
    pub fn hash2(&mut self, k:IndexT)->IndexT{
        k.wrapping_mul(11400714819323198549)
    }

    pub fn hash3(&mut self, k:IndexT)->IndexT{
        // using FNV1-hash
        //https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function   
        let mut hash:IndexT =0xcbf29ce484222325;
        let fnv_prime:IndexT=0x100000001b3;
        let mut mask=0xff;
        let mut pos = 0;
        while mask!=0 {
            //hash = hash * fnv_prime;
            hash = hash.wrapping_mul(fnv_prime);
            hash = hash ^ ((k & mask) >> pos);
            pos+=8;
            mask<<=8; 
        }
        hash
    }

    pub fn bloom_check(&mut self,k:IndexT)->bool{
        for i in 0..self.hashes {
            let hash =(self.hash1(k).wrapping_add(i.wrapping_mul(self.hash3(k))))% self.size;
            if self.get_bit(hash)==Some(false) {
                return false;
            }
        }
        return true;
    }


    pub fn bloom_add(&mut self, k:IndexT){
        for i in 0..self.hashes{
            let hash:IndexT =(self.hash1(k).wrapping_add(i.wrapping_mul(self.hash3(k))))% self.size;
            if self.get_bit(hash)==Some(false) {
                self.set_bit(hash);
                self.count=self.count+1;
            } 
        }
    }

}

#[test]
fn test_bloom(){
    let mut b = BloomFilter{
        hashes:3,
        size:128,
        count:1,
        table:BitVec::from_elem(128,false)
    };
    b.set_bit(0);    
    let temp:Option<bool>=b.get_bit(0);
    let temp1:Option<bool>=b.get_bit(1);
    // print!("{:?}",temp);
    assert!(temp==Some(true));
    assert!(temp1==Some(false));
}

#[test]
fn test_hash(){
    //esting hash
    let test_keys:[IndexT;6]=[0,1,2,3,13,97];
    let mut b = BloomFilter{
        hashes:3,
        size:128,
        count:1,
        table:BitVec::from_elem(128,false)
    };
    b.hash1(1); 
    assert_eq!(b.hash1(test_keys[0]),8633297058295171728);
    assert_eq!(b.hash1(test_keys[5]),14582706179898628597);

}

#[test]
fn test_bloom_basic(){
    let mut b:BloomFilter=BloomFilter{
            hashes:3,
            size:1000,
            count:1,
            //table:BitVec::with_capacity(bloom_size)
            table:BitVec::from_elem(1000,false)
    };
    for k in 0..71 {
        b.bloom_add(k);
    }
    println!("Test of check: \n key present  {0} \n key absent {1} \n",b.bloom_check(1),b.bloom_check(71));   
}

#[test]
fn test_bloom_occupancy(){
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
}


#[test]
fn test_bloom_false_positive(){
let _size_bits:IndexT=1000000;
    let mut b:BloomFilter=BloomFilter{
            hashes:7,
            size:_size_bits,
            count:1,
            //table:BitVec::with_capacity(bloom_size)
            table:BitVec::from_elem(_size_bits as usize,false)
    };
    let rand_max:IndexT = 100000000;
    let top:IndexT=100000;
    let mut test_occurences:IndexT=0;
    let mut occurences:IndexT=0;
    let mut rng=thread_rng();
    for _i in 1..(top+1) {
        let r:IndexT=rng.gen_range(0,rand_max);
        b.bloom_add(r);
        if b.bloom_check(r) {
            test_occurences = test_occurences + 1;
        }
    }
    println!("Test occurences : {0} / 100", test_occurences);

    for _i in 1..(top+1) {
        let r:IndexT = rng.gen_range(0,rand_max);
        if b.bloom_check(r) {occurences = occurences + 1;} 
    }

    println!("Occupancy: {0} bits false positives: {1}", b.count, occurences);
}

