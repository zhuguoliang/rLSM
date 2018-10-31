//LSM树实现
//组件是LSM树实现中的核心数据结构
use std::fs::File;
use std::fs;
use std::io;
use test::Bencher;
use helper;
use component::Component;
use bloom_filter::BloomFilter;

// // First version: finit number of components
// typedef struct LSM_tree {
//     char *name;
//     // C0 and buffer are in main memory
//     component *C0;
//     component *buffer;
//     int Ne; // Total number of key/value tuples stored
//     int Nc; // Number of file components, ie components on disk
//     int value_size; // Upper bound on the value size (in number of chars)
//     int filename_size; // Size of the name, will be used to mainpulate filename
//     // TODO: linked list for infinite number oc components?
//     int *Cs_Ne; // List of number of elements per component: [C0, buffer, C1, C2,...]
//     int *Cs_size; // List of number of elements per component: [C0, buffer, C1, C2,...]
//     bloom_filter_t *bloom;
// } LSM_tree;

#[repr(align(64))]
#[derive(Debug)]
pub struct LSM_tree{
    pub name:String,
    pub C0:Component,
    pub buffer:Component,
    pub ne:usize,//total KV pair count 
    pub nc:usize,
    pub value_size:usize,
    pub filename_size:usize,
    pub Cs_Ne:Vec<u64>, //kv pair count in each component in sequence [C0, buffer, C1, C2,...]
    pub bloom:BloomFilter //size of component in sequence [C0, buffer, C1, C2,...]
}


// // Init first elements of an LSM_tree:
// //  - name
// //  - buffer
// //  - C0
// //  - filename_size (size of name)
// //  - Ne (initliazed to 0)
// void init_lsm(LSM_tree *lsm, char* name, int filename_size){
//     lsm->name = (char*)calloc(filename_size, sizeof(char));
//     strcpy(lsm->name, name);
//     lsm->buffer = (component *) malloc(sizeof(component));
//     lsm->C0 = (component *) malloc(sizeof(component));
//     lsm->filename_size = filename_size;
//     lsm->Ne = 0;
//     if (BLOOM_ON) lsm->bloom = (bloom_filter_t *) malloc(sizeof(bloom_filter_t));
// }

impl LSM_tree{
// pub name:String,
//     pub C0:Component,
    // pub buffer:Component,
    // pub ne:usize,
    // pub nc:usize,
    // pub value_size:usize,
    // pub filename_size:usize
    // pub Cs_Ne:Vec<u64>,
    // bloom:BloomFilter
    pub fn init(&mut self, name:&String, filename_size:usize){
        let self.name = name;
        let self.buffer = 
    }
    pub fn new(&mut self, name:String, nc:usize, ne:usize,cs_size:usize, 
        value_size:usize, filename_size:usize) ->LSM_tree{
        LSM_tree{
            name:name,
            C0:Component::new(cs_size,value_size,1,"C0"),
            buffer:Component::new();

        Component::new(component_size, VALUE_SIZE, ne, 3.to_string());

        }
    }

}


// void build_lsm(LSM_tree *lsm, char* name, int Nc, int* Cs_size, int value_size,
//                int filename_size){
//     // Allocate memory and create lsm
//     create_lsm(lsm, name, Nc, Cs_size, value_size, filename_size);

//     // Initialize C0 and buffer (on memory)
//     init_component(lsm->C0, Cs_size, value_size, lsm->Cs_Ne, "C0");
//     init_component(lsm->buffer, Cs_size + 1, value_size, lsm->Cs_Ne + 1,  "buffer");

#[test]
test_init(){
    let name = "test".to_string();
    let filename_size:usize=32;

}