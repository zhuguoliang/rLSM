#![feature(test)]
//Component
//组件是LSM树实现中的核心数据结构
use std::fs::File;
use std::fs;
use std::io;
use test::Bencher;
use helper;

//size of u64
const KEY_SIZE: usize = 8;
//align to make rust add padding so that every component will fit in cache
#[repr(align(64))]
#[derive(Debug)]
pub struct Component{
    pub keys:Vec<u8>,//int array
    pub values:Vec<u8>,//int array
    pub ne:usize,
    pub s:usize,//u64个数
    pub component_id:String
}
//key_size u64 value_size可变
impl Component{
    pub fn new(component_size:usize, value_size:usize, 
                ne:usize, component_id:String) ->Component{
        Component{
            //keys:Vec::with_capacity(component_size*(mem::size_of::<u64> as usize)),
            keys:vec![0u8,(component_size*KEY_SIZE) as u8],
            values:Vec::with_capacity(component_size*value_size),
            //values:vec![0u8,(component_size*(mem::size_of::<u64> as usize)) as u8],
            ne:ne,
            s:component_size,
            component_id:component_id
        }
    }
    pub fn init(&mut self, component_size:usize, value_size:usize, 
                ne:usize, component_id:&String) {
            self.keys=Vec::with_capacity(component_size);
            self.values=Vec::with_capacity(component_size*value_size);
            self.ne=ne;
            self.s=component_size;
            self.component_id=component_id.to_owned();
    }

    //TODO: this func can be made faster using char array?
    pub fn create_disk_component(&mut self, name:String, nc:u64,filename_size:usize)
                ->io::Result<()>
    {
        let component_type:[String;2]=['k'.to_string(),'v'.to_string()];
        let mut component_id = String::new();
        for _i in 1..nc {
            component_id.clear();
            //component_id.push_str("C");//string pushs &str into itself
            component_id.push_str(&_i.to_string());
            for _k in 0..2 {
                if fs::metadata(&name).is_err(){
                    match fs::create_dir_all(&name){
                        Err(why)=> panic!("could not open due to  {}", why),
                        Ok(())=> println!("Succefully created dir" ),   
                    };
                }
                File::create(
                    &helper::get_files_name(&name,&component_id,
                    &component_type[_k],filename_size)
                )?;
            }
        }

        Ok(())
    }
    pub fn read_disk_component(&mut self, name:String, ne:usize,component_id:String,
           component_size:usize,value_size:usize,filename_size:usize){

        self.init(component_size,value_size,ne,&component_id);
        let filename_keys=helper::get_files_name(&name,&component_id,"k",filename_size);
        let filename_values=helper::get_files_name(&name,&component_id,"v",filename_size);

        helper::load_file_to_vec(&mut self.keys, &filename_keys, KEY_SIZE, ne);
        helper::load_file_to_vec(&mut self.values, &filename_values, value_size, ne);

    }

//value_size:usize is unnecessary because rust has write_all
    pub fn write_disk_component(&mut self, name:String,
                            filename_size:usize){

        let filename_keys=helper::get_files_name(&name,&self.component_id,"k",filename_size);
        let filename_values=helper::get_files_name(&name,&self.component_id,"v",filename_size);
        helper::flush_vec_to_file(&mut self.keys, &filename_keys);
        helper::flush_vec_to_file(&mut self.values, &filename_values);
    }

// Append to a component on disk the last n keys/values 
    pub fn append_on_disk(&mut self, n:usize,name:String,
    value_size:usize,filename_size:usize) {
        let filename_keys=helper::get_files_name(&name,&self.component_id,"k",filename_size);
        let filename_values=helper::get_files_name(&name,&self.component_id,"v",filename_size);
        helper::append_last_n_to_file(&mut self.keys, &filename_keys, n);
        helper::append_last_n_to_file(&mut self.values, &filename_values, n);
    }
}




#[test]
fn test_init(){
    let c_size:usize = 100;
    let ne:usize=10;
    let c_id:String="012312".to_string();
    let _c:Component = Component{
        keys:Vec::with_capacity(c_size as usize),
        values:Vec::with_capacity(c_size as usize),
        ne:ne,
        s:c_size,
        component_id:c_id
    };
}

#[test]
fn test_create_disk_component()
{
    let mut c:Component = Component::new(10, 10, 2, 3.to_string());
    let _created = match c.create_disk_component("hello".to_string(), 10, 32){
        Err(why)=> panic!("could not create due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"), 
    };
    //assert_eq!(c.create_disk_component("hello".to_string(), 10, 30),);

}


#[test]
fn test_write_component()
{
    let mut c:Component = Component::new(10, 10, 2, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), 10, 30){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };
    c.keys.insert(0, 100);
    c.values.insert(0, 101);
    c.write_disk_component("hello".to_string(), 20);
}


#[test]
fn test_read_component()
{
    let mut c:Component = Component::new(10, 10, 2, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), 10, 30){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };
    c.keys.insert(0, 100);
    c.values.insert(0, 101);
    c.write_disk_component("hello".to_string(), 20);
    //c.read_disk_component("hello".to_string(), 20);
    c.read_disk_component("hello".to_string(), 10, 3.to_string(), 10, 10, 20);
    
}


#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn benchmark_read_component(b: &mut Bencher)
{
    b.iter(|| test_read_component());
}
