//This is helper function for this project
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::{Read};

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

pub fn load_file_to_vec(resvec:&mut Vec<u8>, fname:&String, one_size:usize,
        ne:usize){
  
    let f = match OpenOptions::new().read(true).open(fname) {
                Err(why)=> panic!("could not open due to  {}", why),
                Ok(fkeys)=> fkeys,
            };

    match f.take((one_size*ne) as u64).read_to_end(resvec) {
        Err(why)=> panic!("cannot read from file due to {}",why),
        Ok(readsize)=> println!("Succefully read {} bytes", readsize ),
    };

}

pub fn flush_vec_to_file(u8vec:&mut Vec<u8>, fname:&String)
{
    //using scope to control file open range
    //Reading files
    let mut fvalues = match OpenOptions::new().write(true).open(&fname) {
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(fvalues)=> fvalues,
    };
    // Now trying to write vec<u8> to file 
    match fvalues.write_all(&u8vec) {
        Err(why)=> panic!("cannot write to file due to {}",why),
        Ok(())=> println!("Succefully written to file {}",fname),
    }
}
