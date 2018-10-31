//Component
//组件是LSM树实现中的核心数据结构
use std::fs::File;
use std::fs;
use std::io;
use test::Bencher;
use helper;

pub type DataT= Vec<Vec<u8>>;
//size of u64
const KEY_SIZE: usize = 8;
const VALUE_SIZE:usize = 8;
const FILENAME_SIZE:usize = 32; 
const COMPONENT_NUM:usize= 6;
const COMPONENT_SIZE:usize = 32;
//align to make rust add padding so that every component will fit in cache
#[repr(align(64))]
#[derive(Debug)]
pub struct Component{
    keys:DataT,     //key   Vec<u8>
    values:DataT,   //value Vec<u8>
    pub ne:usize,   //number of KV pair 
    pub s:usize,    //means capacity of components
    pub component_id:String
}
//key_size u64 (value_size is for each value vec<u8>)
impl Component{
    
    pub fn new(component_size:usize, _value_size:usize, 
                ne:usize, component_id:String) ->Component{
        Component{
            //keys:Vec::with_capacity(component_size*(mem::size_of::<u64> as usize)),
            keys:Vec::with_capacity(component_size),
            values:Vec::with_capacity(component_size),
            //values:vec![0u8,(component_size*(mem::size_of::<u64> as usize)) as u8],
            ne:ne,
            s:component_size,
            component_id:component_id
        }
    }
    pub fn init(&mut self, component_size:usize, _value_size:usize, 
                ne:usize, component_id:&String) {
            self.keys=Vec::with_capacity(ne);
            self.values=Vec::with_capacity(ne);
            self.ne=ne;
            self.s=component_size;
            self.component_id=component_id.to_owned();
    }

    pub fn push_key(&mut self, key:Vec<u8>) {
        assert!(key.len()==KEY_SIZE);//SIZE IN BYTES
        self.keys.push(key);
    }

    pub fn push_value(&mut self, value:Vec<u8>) {
        assert!(value.len()==VALUE_SIZE);
        self.values.push(value);
    }
    //TODO: this func can be made faster using char array?
    pub fn create_disk_component(&mut self, name:String, nc:usize,filename_size:usize)
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
        helper::load_file_to_vec(&mut self.values, &filename_values, VALUE_SIZE, ne);

        //assert!(ne<=self.values.len());
        //assert!(ne<=self.keys.len());
    }

//value_size:usize is unnecessary because rust has write_all
    pub fn write_disk_component(&mut self, name:String, ne:usize,
                            filename_size:usize){

        let filename_keys=helper::get_files_name(&name,&self.component_id,"k",filename_size);
        let filename_values=helper::get_files_name(&name,&self.component_id,"v",filename_size);

        assert!(ne<=self.values.len());
        assert!(ne<=self.keys.len());

        helper::flush_vec_to_file(&mut self.keys, &filename_keys);
        helper::flush_vec_to_file(&mut self.values, &filename_values);
    }

// Append to a component on disk the last n keys/values 
    pub fn append_on_disk(&mut self, n:usize,name:String,
    value_size:usize,filename_size:usize) {
        assert!(value_size==VALUE_SIZE);
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
    let component_size=10;
    let ne=2;
    let mut c:Component = Component::new(component_size, VALUE_SIZE, ne, 3.to_string());
    let _created = match c.create_disk_component("hello".to_string(), COMPONENT_NUM, FILENAME_SIZE){
        Err(why)=> panic!("could not create due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"), 
    };

}


#[test]
fn test_write_component()
{
    let component_size=10;
    let ne=2;
    let mut c:Component = Component::new(component_size, VALUE_SIZE, ne, 3.to_string());
    //let mut c:Component = Component::new(10, 10, 2, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), COMPONENT_NUM, FILENAME_SIZE){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };

    for _entry in 0..ne{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+VALUE_SIZE) {
            value.push(j as u8);
        }
        c.push_key(key);
        c.push_value(value);
    }
    c.write_disk_component("hello".to_string(), ne,FILENAME_SIZE);
}


#[test]
fn test_read_component()
{
    let ne=2;
    let mut c:Component = Component::new(COMPONENT_SIZE, VALUE_SIZE, ne, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), COMPONENT_NUM, FILENAME_SIZE){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };

    for _entry in 0..ne{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+VALUE_SIZE) {
            value.push(j as u8);
        }
        c.push_key(key);
        c.push_value(value);
    }
    c.write_disk_component("hello".to_string(), ne,FILENAME_SIZE);
    c.read_disk_component("hello".to_string(), ne, 3.to_string(), COMPONENT_SIZE, VALUE_SIZE, FILENAME_SIZE);
    
}

#[test]
fn test_append_n_kvpair()
{
    let ne=2;
    let mut c:Component = Component::new(COMPONENT_SIZE, VALUE_SIZE, ne, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), COMPONENT_NUM, FILENAME_SIZE){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };
    for _entry in 0..ne{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+VALUE_SIZE) {
            value.push(j as u8);
        }
        c.push_key(key);
        c.push_value(value);
    } 
    c.write_disk_component("hello".to_string(), ne,FILENAME_SIZE);
    c.read_disk_component("hello".to_string(), ne, 3.to_string(), COMPONENT_SIZE, VALUE_SIZE, FILENAME_SIZE);
    c.append_on_disk(2, "hello".to_string(), VALUE_SIZE, FILENAME_SIZE);
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
