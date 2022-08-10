
use std::collections::{HashMap,HashSet};

#[allow(dead_code)]
#[derive(Debug)]
struct DoWeHaveAMultiple {
  old_map : HashMap<usize,Vec<(Vec<u8>,usize)>>,
  add_map : HashMap<usize,Vec<(Vec<u8>,usize)>>,
  add_degs : Vec<usize>,
}

#[allow(dead_code)]
impl DoWeHaveAMultiple {
  pub fn new() -> DoWeHaveAMultiple {
    DoWeHaveAMultiple { old_map : HashMap::new(),add_map: HashMap::new(),add_degs: Vec::new() }
  }
  pub fn simple_supermatch(sup: &Vec<u8>,sub : &Vec<u8>) -> bool {
    let len2 = sup.len();
    let len1 = sub.len();
    //eprintln!("sup:{:?},sub:{:?}",sup,sub);
    for i in 0..=(len2-len1) {
      //println!("Comparing {:?} against {:?}",&sup[i..(i+len1)],sub);
      if &sup[i..(i+len1)]==sub {
        //println!("Match");
        return true;
      } 
    }
    false
  }
  pub fn has_supermatch(&self,v : &Vec<u8>) -> bool {
    let len = v.len();
    for i in self.old_map.iter() {
      let deg = *i.0;
      //println!("deg:{:?},len:{:?},try:{:?}",deg,len,deg<=len);
      if deg <= len {
        let list = i.1;
        for k in list {
          if DoWeHaveAMultiple::simple_supermatch(v,&k.0) {
            return true;
          }
        }
      }
    }
    false
  }
  pub fn add(&mut self,new_slices : &[(&[u8],usize)]) {
    // Sort new stuff by degree
    let mut h = HashSet::new();
    for i in new_slices.iter() {
      let len = i.0.len();
      let mut v = Vec::with_capacity(i.0.len());
      v.extend_from_slice(i.0);
      self.add_map.entry(len).or_insert(vec![]).push((v,i.1));
      h.insert(len);
    }
    self.add_degs.reserve(h.len());
    for i in h.iter() {
      self.add_degs.push(*i) ;
    }
    self.add_degs.sort();
    println!("yo:{:?}",self.add_degs);
    for i in self.add_map.iter_mut() {
      i.1.sort();
      i.1.dedup();
    }
    println!("end add:{:?}",self);
  }
  pub fn run(& mut self) -> Option<usize> {
    for i in self.add_degs.iter() {
      let deg : usize = *i;
      loop {
        let val = self.add_map.get_mut(&deg).unwrap().pop();
        if val.is_none() { break;}
        let it = val.unwrap();
        if !self.has_supermatch(&it.0) {
          let mut v = Vec::with_capacity(it.0.len());
          v.extend_from_slice(&it.0[..]);
          self.old_map.entry(deg).or_insert(vec![]).push((v,it.1));
          return Some(it.1);
        }
      }
    }
    self.add_map.clear();
    self.add_degs.clear();
    None
  }
}

#[allow(dead_code)]
fn test_do_we_have_a_multiple() {
  let mut it = DoWeHaveAMultiple::new();
  let a1 = vec![1,2,3];
  let a2 = vec![3,2,1];
  let b = vec![4,1,2,3];
  let v = &[
    ( &a1[..], 100),
    ( &a2[..], 101),
    ( &b[..], 102),
    ( &a1[..], 100),
  ];
  it.add(v);
  while let Some(n) = it.run() {
    println!("obtained {:?}:",n);
  }
}

fn main() {
  test_do_we_have_a_multiple();
}
