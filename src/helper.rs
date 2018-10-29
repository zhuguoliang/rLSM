//This is helper function for this project
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::{Read,BufReader,BufWriter};
//use std::fs::File;
pub fn get_files_name(name:&String, component_id:&String, 
    component_type:&str, filename_size:usize) ->String {
    let mut filename = String::with_capacity(filename_size + 8);
   // let mut filename = String::new();
    filename.push_str(name);
    filename.push_str("/"); 
    filename.push_str(component_type);
    filename.push_str(&component_id);

    filename
}

//one key/value a line
pub fn load_file_to_vec(resvec:&mut Vec<Vec<u8>>, fname:&String, one_size:usize,
        ne:usize){

    println!("load filename is {}",fname );
    let f = match OpenOptions::new().read(true).open(fname) {
                Err(why)=> panic!("could not open due to  {}", why),
                Ok(fkeys)=> fkeys,
            };

    let file = BufReader::new(&f);

    let mut rawvec:Vec<u8> = Vec::with_capacity(one_size*ne);
    //take unit is bytes
    match file.take((one_size * ne)as u64).read_to_end(&mut rawvec) {
       Err(why)=> panic!("cannot read from file due to {}",why),
       Ok(readsize)=> println!("Succefully read {} bytes", readsize ),
    };
    for i in 0..ne {
        resvec.push(rawvec[i*one_size..(i*one_size+one_size)].to_vec());
        //println!("i is {}",i );
    }
}

pub fn load_file_to_vec1(resvec:&mut Vec<u8>, fname:&String, one_size:usize,
        ne:usize){
    let f = match OpenOptions::new().read(true).open(fname) {
                Err(why)=> panic!("could not open due to  {}", why),
                Ok(fkeys)=> fkeys,
            };
    let f =BufReader::new(f);

    match f.take((one_size*ne) as u64).read_to_end(resvec) {
        Err(why)=> panic!("cannot read from file due to {}",why),
        Ok(readsize)=> println!("Succefully read {} bytes", readsize ),
    }; 

    //match  
}


pub fn flush_vec_to_file(u8vec:&mut Vec<Vec<u8>>, fname:&String)
{
       //using scope to control file open range
    //Reading files
    let f = match OpenOptions::new().append(true).open(&fname) {
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(f)=> f,
    };
    println!("flush filename is {}",fname );
    let mut f=BufWriter::new(f); 

    for lvec in u8vec {
        match f.write_all(&lvec){
            Err(why)=> panic!("Could not write due to {}",why),
            Ok(_writtensize)=> (),
        }
    }
}

pub fn flush_vec_to_file0(u8vec:&mut Vec<u8>, fname:&String)
{
    //using scope to control file open range
    //Reading files
    let mut f = match OpenOptions::new().write(true).open(&fname) {
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(f)=> f,
    };
    // Now trying to write vec<u8> to file 
    match f.write_all(&u8vec) {
        Err(why)=> panic!("cannot write to file due to {}",why),
        Ok(())=> println!("Succefully written to file {}",fname),
    }
}
//buffered version
pub fn flush_vec_to_file1(u8vec:&mut Vec<u8>, fname:&String)
{
    //using scope to control file open range
    //Reading files
    let f = match OpenOptions::new().write(true).open(&fname) {
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(f)=> f,
    };
    let mut f=BufWriter::new(f);
    match f.write_all(&u8vec) {
        Err(why)=> panic!("cannot write to file due to {}",why),
        Ok(())=> println!("Succefully written to file {}",fname),
    } 
     match f.flush() {
        Err(why)=> panic!("cannot flush to file due to {}",why),
        Ok(())=> println!("Succefully flush to file {}",fname),
    } 

}

pub fn append_last_n_to_file(u8vec:&mut Vec<Vec<u8>>, fname:&String,n:usize)
{
    assert!(n>=u8vec.len(),"There is no more than {} element in component",n);
    let f = match OpenOptions::new().append(true).open(&fname) {
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(f)=> f,
    };
    let mut f=BufWriter::new(f);
    for i in u8vec.len()-n..u8vec.len() {
        match f.write_all(&u8vec[i]){
            Err(why)=> panic!("Could not write due to {}",why),
            Ok(_writtensize)=> (),
        }
    } 
}

// fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
//     let b1 : u8 = ((x >> 24) & 0xff) as u8;
//     let b2 : u8 = ((x >> 16) & 0xff) as u8;
//     let b3 : u8 = ((x >> 8) & 0xff) as u8;
//     let b4 : u8 = (x & 0xff) as u8;
//     return [b1, b2, b3, b4]
// }


// fn transform_u64_to_array_of_u8(x:u64) -> [u8;8] {
//     let b1 : u8 = ((x >> 56) & 0xff) as u8;
//     let b2 : u8 = ((x >> 48) & 0xff) as u8;
//     let b3 : u8 = ((x >> 40) & 0xff) as u8;
//     let b4 : u8 = ((x >> 32) & 0xff) as u8;
//     let b5 : u8 = ((x >> 24) & 0xff) as u8;
//     let b6 : u8 = ((x >> 16) & 0xff) as u8;
//     let b7 : u8 = ((x >> 8) & 0xff) as u8;
//     let b8 : u8 = (x & 0xff) as u8; 
//     return [b1, b2, b3, b4, b5, b6, b7, b8]
// }

// fn transform_array_of_u8_to_u64(u8vec:Vec<u8>) -> u64 {
//     (u8vec[0] as u64) << 56 |(u8vec[1] as u64) << 48|
//     (u8vec[2] as u64) << 40 |(u8vec[3] as u64) << 32|
//     (u8vec[4] as u64) << 24 |(u8vec[5] as u64) << 16|
//     (u8vec[6] as u64) <<  8 |(u8vec[7] as u64)
// }
