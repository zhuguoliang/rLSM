//LSM树实现
//组件是LSM树实现中的核心数据结构
use std::fs::File;
use std::fs;
use std::io;
use test::Bencher;
use helper;
use component::Component;
use bloom_filter::BloomFilter;
use rand::Rng;
use global_conf;
use bit_vec::BitVec;


#[repr(align(64))]
#[derive(Debug)]
pub struct LSM_tree{
    pub name:String,
    pub ne:usize,//total KV pair count 
    pub nc:usize,
    pub value_size:usize,
    pub filename_size:usize,
    pub Cs_size:Vec<u64>, //size of component in sequence [C0, buffer, C1, C2,...]
    //pub Cs_Ne:Vec<u64>, //kv pair count in each component in sequence [C0, buffer, C1, C2,...]
    pub C0:Component,
    pub buffer:Component,
    pub bloom:BloomFilter 
}

// Initialize C0 and buffer (on memory)
// init_component(lsm->C0, Cs_size, value_size, lsm->Cs_Ne, "C0");
// init_component(lsm->buffer, Cs_size + 1, value_size, lsm->Cs_Ne + 1,  "buffer");
impl LSM_tree{
    //by default we recommend use new
    pub fn init(&mut self, name:&String, Cs_size:Vec<u64>, nc:usize,
    filename_size:usize,value_size:usize){
        //let NE:usize = 0;
        let cs_ne_size = nc + 2;
        //assert size is consistent in Cs_size Cs_Ne and nc

        //let buffer_size =
        self.name = name.to_string();
        self.ne = 0;
        self.nc = nc;
        self.filename_size = filename_size;
        self.value_size = value_size;
        self.Cs_size = Cs_size.clone();
        //self.Cs_Ne = Vec::with_capacity(cs_ne_size);
        //self.Cs_Ne = Vec::with_capacity(cs_ne_size);
        self.C0 = Component::new(&(Cs_size[0] as usize), value_size, "C0".to_owned());
        self.buffer = Component::new(&(Cs_size[1] as usize), value_size,  "buffer".to_owned());
        self.bloom = BloomFilter{
            hashes:global_conf::HASHES,
            size:global_conf::BLOOM_SIZE,
            count:0,
            table:BitVec::from_elem(global_conf::BLOOM_SIZE as usize,false)
        };
    }
    pub fn new(name:&String, Cs_size:Vec<u64>, nc:usize,
    filename_size:usize,value_size:usize)->LSM_tree{
        let cs_ne_size = nc + 2; 
        LSM_tree{
            name:name.to_string(),
            nc:nc,
            ne:0,
            filename_size:filename_size,
            value_size:value_size,
            Cs_size:Cs_size.clone(),
            C0:Component::new(&(Cs_size[0] as usize),value_size,"C0".to_owned()),
            buffer:Component::new(&(Cs_size[1] as usize), value_size,  "buffer".to_owned()),
            bloom: BloomFilter{
                hashes:global_conf::HASHES,
                size:global_conf::BLOOM_SIZE,
                count:0,
                table:BitVec::from_elem(global_conf::BLOOM_SIZE as usize,false)
            }
        //Component::new(component_size, VALUE_SIZE, ne, 3.to_string());
        }
    }

}


// Create LSM Tree with a fixed number of component Nc with
// initialization of the components on memory & on disk (create
// the files inside the folder).
// Check if folder exists and clean it if needed.
// Save metadata and memory component for recovery.
// TODO: check the validity of the args
//void build_lsm(LSM_tree *lsm, char* name, int Nc, int* Cs_size, int value_size,
//               int filename_size){

//}

#[test]
fn test_init_lsmt(){
    let name = "test".to_string();
    let filename_size:usize=32;
    let SIZE:u64 = 1000;
    let Cs_size = vec![SIZE, 3*SIZE, 9*SIZE, 27*SIZE, 
    81*SIZE, 3*81*SIZE, 729*SIZE, 3*729*SIZE,1000000000];
    //let Cs_ne = vec![0,0,0,0,0,0,0,0,0]
    let Nc = 7;
    let ml = LSM_tree::new(&name, Cs_size,Nc,filename_size,global_conf::VALUE_SIZE);
}

// Generate a LSMT with num_elements (keys [0, num_elements[, value: 'aa..aa_{key%1000}' )
// arg: sorted too insert keys in sorted order or not.
// Return the execution time
#[test]
fn test_LSMTree_generation(){
    /*
let sorted:usize = 0;
let num_elements:usize = 10000;
let mut rng = rand::thread_rng();
    if sorted==0 {
        for i in 0..num_elements {
            array[i] = i;
        }
        //randomize array for test 
        for i in 0..num_elements {
            let tmp = array[i];
            let randIndex:usize = rng.gen();
            array[i] = array[randIndex];
            array[randIndex] = tmp;
        }
    }

*/


}