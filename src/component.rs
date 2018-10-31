//Component
//组件是LSM树实现中的核心数据结构
use std::fs::File;
use std::fs;
use std::io;
use test::Bencher;
use helper;
use global_conf;

//align to make rust add padding so that every component will fit in cache
#[repr(align(64))]
#[derive(Debug)]
pub struct Component{
    keys:global_conf::DataT,     //key   Vec<u8>
    values:global_conf::DataT,   //value Vec<u8>
    value_size:usize, //value size in byte means in Vec<Vec<u8>>, the sizeof each inner vec 
    pub ne:usize,   //number of KV pair 
    pub s:usize,    //means capacity of components
    pub component_id:String
}
impl Component{
    pub fn new(component_size:usize, value_size:usize, 
                component_id:String) ->Component{
        Component{
            //keys:Vec::with_capacity(component_size*(mem::size_of::<u64> as usize)),
            keys:Vec::with_capacity(component_size),
            values:Vec::with_capacity(component_size),
            value_size:value_size,
            //values:vec![0u8,(component_size*(mem::size_of::<u64> as usize)) as u8],
            ne:0,
            s:component_size,
            component_id:component_id
        }
    }
    pub fn init(&mut self, component_size:usize, _value_size:usize, 
                component_id:&String) {
            self.keys=Vec::with_capacity(component_size);
            self.values=Vec::with_capacity(component_size);
            self.ne=0;
            self.s=component_size;
            self.component_id=component_id.to_owned();
    }

    pub fn push_kv(&mut self, key:Vec<u8>, value:Vec<u8>){
        self.push_key(key);
        self.push_value(value);
        self.ne = self.ne + 1;
    }

    fn push_key(&mut self, key:Vec<u8>) {
        assert!(key.len()==global_conf::KEY_SIZE);//SIZE IN BYTES
        self.keys.push(key);
    }

    fn push_value(&mut self, value:Vec<u8>) {
        assert!(value.len()==self.value_size);
        self.values.push(value);
    }
    //TODO: this func can be made faster using char array?
    pub fn create_disk_component(&mut self, name:String, nc:usize)
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
                    &component_type[_k],global_conf::FILENAME_SIZE)
                )?;
            }
        }

        Ok(())
    }
    pub fn read_disk_component(&mut self, name:String, component_id:String){

        let filename_keys=helper::get_files_name(&name,&component_id,"k",global_conf::FILENAME_SIZE);
        let filename_values=helper::get_files_name(&name,&component_id,"v",global_conf::FILENAME_SIZE);

        helper::load_file_to_vec(&mut self.keys, &filename_keys, global_conf::KEY_SIZE, self.ne);
        helper::load_file_to_vec(&mut self.values, &filename_values, self.value_size, self.ne);
        //println!("read finished current ne is {}",self.ne );
        assert!(self.ne<=self.values.len());
        assert!(self.ne<=self.keys.len());
    }

//value_size:usize is unnecessary because rust has write_all
    pub fn write_disk_component(&mut self, name:String){

        let filename_keys=helper::get_files_name(&name,&self.component_id,"k",global_conf::FILENAME_SIZE);
        let filename_values=helper::get_files_name(&name,&self.component_id,"v",global_conf::FILENAME_SIZE);

        assert!(self.ne<=self.values.len());
        assert!(self.ne<=self.keys.len());

        helper::flush_vec_to_file(&mut self.keys, &filename_keys);
        helper::flush_vec_to_file(&mut self.values, &filename_values);
    }

// Append to a component on disk the last n keys/values 
    pub fn append_on_disk(&mut self, n:usize,name:String) {
        let filename_keys=helper::get_files_name(&name,&self.component_id,"k",global_conf::FILENAME_SIZE);
        let filename_values=helper::get_files_name(&name,&self.component_id,"v",global_conf::FILENAME_SIZE);
        helper::append_last_n_to_file(&mut self.keys, &filename_keys, n);
        helper::append_last_n_to_file(&mut self.values, &filename_values, n);
    }

}

#[test]
fn test_init(){
    let c_size:usize = 100;
    let ne:usize=10;
    let c_id:String="012312".to_string();
    let value_size = 32;
    let _c:Component = Component{
        keys:Vec::with_capacity(c_size as usize),
        values:Vec::with_capacity(c_size as usize),
        ne:ne,
        value_size:value_size,
        s:c_size,
        component_id:c_id
    };
}

#[test]
fn test_create_disk_component()
{
    let val_size=8;
    let component_size=10;
    let cpt_num = 10;
    let mut c:Component = Component::new(component_size, val_size, 3.to_string());
    let _created = match c.create_disk_component("hello".to_string(), cpt_num){
        Err(why)=> panic!("could not create due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"), 
    };

}


#[test]
fn test_write_component()
{
    let val_size = 8;
    let component_size=10;
    let cpt_num = 10;
    let mut c:Component = Component::new(component_size, val_size, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), cpt_num){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };

    for _entry in 0..2{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+global_conf::KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+val_size) {
            value.push(j as u8);
        }
        c.push_kv(key, value);
    }
    c.write_disk_component("hello".to_string());
}


#[test]
fn test_read_component()
{
    let cpt_size=100;
    let cpt_num = 10;
    let val_size = 8;
    let mut c:Component = Component::new(cpt_size, val_size, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), cpt_num){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };

    for _entry in 0..2{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+global_conf::KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+val_size) {
            value.push(j as u8);
        }
        c.push_kv(key, value);
    }
    c.write_disk_component("hello".to_string());
    c.read_disk_component("hello".to_string(), 3.to_string());
    
}

#[test]
fn test_append_n_kvpair()
{
    let cpt_size=100;
    let cpt_num = 10;
    let val_size = 8;
    let mut c:Component = Component::new(cpt_size, val_size, 3.to_string());
    let _createres = match c.create_disk_component("hello".to_string(), cpt_num){
        Err(why)=> panic!("could not open due to  {}", why),
        Ok(())=> println!("Succefully created component on disk"),
    };
    for _entry in 0..3{
        let mut key = Vec::new();
        let mut value = Vec::new();
        for i in 100..(100+global_conf::KEY_SIZE) {
            key.push(i as u8);
        }
        for j in 200..(200+val_size) {
            value.push(j as u8);
        }
        c.push_kv(key, value);
    } 
    c.write_disk_component("hello".to_string());
    c.read_disk_component("hello".to_string(), 3.to_string());
    c.append_on_disk(2, "hello".to_string());
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
