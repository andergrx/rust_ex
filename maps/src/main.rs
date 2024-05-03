use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;


#[derive(Clone)]
struct MyMap<K,V> {
    hmap: HashMap<K,V>,
    bmap: BTreeMap<K,V>,
}

impl<K:std::cmp::Eq + std::fmt::Display + std::hash::Hash + std::cmp::Ord + Copy,
     V:std::fmt::Display + Copy> MyMap<K,V> {

//impl<K,V> MyMap<K,V>
    fn print_hmap(&self) {
        for key in self.hmap.keys() {
            println!("MyMap<Hash> -> {}: {}", key, self.hmap[key]);
        }        
    }
    
    fn print_bmap(&self) {
        for key in self.bmap.keys() {
            println!("MyMap<BTree> -> {}: {}", key, self.bmap[key]);
        }        
    }
    
//impl<K,V> MyMap<K,V> {
    
    fn new() -> MyMap<K,V> {
        MyMap {
            hmap: HashMap::new(),
            bmap: BTreeMap::new()
        }
    }
    
    //#[derive(Copy, Clone)]
    fn insert(&mut self, key: K, val: V) {
        self.hmap.insert(key,val);
        self.bmap.insert(key,val);
    }
}

#[derive(Clone)]
struct Braves {
    sport: String,
    city: String,
    
}
impl Braves {

    fn copy(&mut self, input: &Braves) {
        
        self.sport =  input.sport.clone();
        self.city =  input.city.clone();
    }
    
    fn new() -> Braves {
        Braves {
            sport: String::new(),
            city: String::new(),
        }   
    }
}

impl fmt::Display for Braves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.sport, self.city)
    }
    
}

fn main() {
    let mut map = HashMap::new();
    
    map.insert("Gabe".to_string(), 42);
    
    print_map(&map);
    
    for val in map.values_mut() {
        *val = *val + 1
    }
    
    print_map(&map);
    
    map.insert("Anderson".to_string(), 99);
    
    for (key, val) in map.iter() {
        println!("key,val: {key}, {val}");
    }
    
    let mut bmap = BTreeMap::new();
    bmap.insert(1, "One".to_string());
    bmap.insert(2, "Two".to_string());
    print_bmap(&bmap);
    for (key, val) in bmap.iter() {
        println!("key,val: {key}, {val}");
    }
    
    let hmap = HashMap::from([
        ("Dude".to_string(), 3),
        ("Bro".to_string(), 7)
    ]);
    
    mprint(&hmap);
    
    let gmap = HashMap::from([
        (5, 4.7),
        (9, 3.2)
    ]);
    
    mprint(&gmap);
    
    let mut z = 3;
    let g = &mut z;
    
    *g = *g + 3;
    
    println!("G={g}");
    
    let mut mmap: MyMap<&str,u32> = MyMap::new();
    mmap.insert("abc", 44);
    mmap.insert("Duder", 33);
    mmap.insert("Bruh", 11);
    
    mmap.print_hmap();
    mmap.print_bmap();
    
    let mut tmap: MyMap<u32,f64> = MyMap::new();
    tmap.insert(7,4.435436);
    tmap.insert(13,43242.11129);
    tmap.insert(15,-2342897.778787);
    
    tmap.print_hmap();
    tmap.print_bmap();
    
    let brave = Braves{ sport: "baseball".to_string(), city: "Atlanta".to_string() };
    
    let b2 = brave.clone();
    let mut b3 = Braves::new();
    b3.copy(&b2);
    println!("brave: {}, {}, {}", brave, b2, b3);
}

fn mprint<K:std::cmp::Eq + std::fmt::Display + std::hash::Hash,
          V:std::fmt::Display>(map: &HashMap<K,V>) {
          
    for key in map.keys() {
        println!("{}: {}", key, map[key]);
    }     
}

fn print_map(map: &HashMap<String, u32>) {
    for key in map.keys() {
        println!("{key}: {}", map[key]);
    }  
}

fn print_bmap(map: &BTreeMap<u32, String>) {
    for key in map.keys() {
        println!("{key}: {}", map[key]);
    }
}