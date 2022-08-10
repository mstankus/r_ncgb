// Mark Stankus (c) Sun Aug  7 17:19:55 PDT 2022
use subtraction_decisions::{FirstOrSecondCoefficient,KeepVars};

pub struct Numbers {
  v : Vec<bool>,
}

impl Numbers {
  fn internal_number_via_slice(x : &str) -> bool {
    let mut it = x;
    while !it.is_empty() &&  (it.starts_with('+') || it.starts_with('-')) {
      it = &it[1..];
    }
    if it.is_empty() { return true;}
    println!("{:?}",it);
    match it.parse::<usize>() {
      Ok(n) => { n%2!=0 },
      Err(_) => { panic!("Having trouble interpreting {} as a string.",it); }
    }
  }
  fn internal_number_via_slices(x : &[&str]) -> bool {
    let mut result = false;
    for i in x.iter() {
      if Numbers::internal_number_via_slice(i) {
        result = !result;
      }
    }
    result
  }
  pub fn from_strings(v : &[&[&str]]) -> Numbers {
    let len = v.len();
    let mut result1 = Vec::with_capacity(len);
    for i in v.iter() {
      result1.push(Numbers::internal_number_via_slices(i))
    }
    Numbers { v : result1 }
  }
  pub fn nonzero_number_positions(&self) -> Vec<usize> {
    let mut result = Vec::with_capacity(self.v.len());
    for (i,flag) in self.v.iter().enumerate() {
      if *flag { result.push(i); } 
    }
    result
  }
  pub fn create_nonzero_numbers(&self) -> Numbers {
    let mut len = 0;
    for flag in self.v.iter() {
      if *flag { len += 1; } 
    }
    Numbers { v : vec![true;len] }
  }
  pub fn make_monic(&mut self) -> Numbers {
    let mut len = 0;
    for flag in self.v.iter() {
      if *flag { len += 1; } 
    }
    Numbers { v : vec![true;len] }
  }
  pub fn sub(x : bool, y : bool) -> bool {
    x!=y
  }
  pub fn is_zero(b : bool) -> bool {
    !b
  } 
  pub fn generate_numbers_from_first_or_second_coefficient(
       &self, 
       other : &Self,
       dec : &Vec<FirstOrSecondCoefficient>,
       ) -> (Numbers, Vec<KeepVars>) {
    let len = dec.len();
    let mut result1 = Vec::with_capacity(len);
    let mut result2 = Vec::with_capacity(len);
    let mut i1 = 0;
    let mut i2 = 0;
    for item in dec.iter() {
      match item {
        FirstOrSecondCoefficient::UseFirst => { 
          result1.push(self.v[i1]);
          i1 += 1;
          result2.push(KeepVars::UseFirst);
        }
        FirstOrSecondCoefficient::UseSecond => { 
          result1.push(other.v[i2]);
          i2 += 1;
          result2.push(KeepVars::UseSecond);
        }
        FirstOrSecondCoefficient::ConsiderBoth => {
          let diff = Numbers::sub(self.v[i1],other.v[i2]);
          if Numbers::is_zero(diff) {
            result2.push(KeepVars::SkipBoth);
          } else {
            result1.push(true);
            result2.push(KeepVars::UseFirstSkipSecond);
          }
          i1 += 1;
          i2 += 1;
        }
      }
    }
    (Numbers { v: result1 } ,result2)
  }
}

#[cfg(test)]
mod tests {
  use subtraction_decisions::{FirstOrSecondCoefficient,KeepVars};
  use super::Numbers;
  #[test]
  fn test_from_strings_et_al() {
    let one1 = "+";
    let one2 = "1";
    let zero = "0";
    assert!(Numbers::internal_number_via_slice(one1));
    assert!(Numbers::internal_number_via_slice(one2));
    assert!(!Numbers::internal_number_via_slice(zero));

    let one_slice = &[one1];
    assert!(Numbers::internal_number_via_slices(one_slice));
    assert!(Numbers::internal_number_via_slices(&[one1]));
    assert!(Numbers::internal_number_via_slices(&[one2]));

    let one_one = &[one1,one1];
    let one_one_zero = &[one1,one1,zero];
    let zero_zero = &[zero,zero];
    let one_zero = &[one1,zero];
    assert!(!Numbers::internal_number_via_slices(one_one));
    assert!(!Numbers::internal_number_via_slices(one_one_zero));
    assert!(!Numbers::internal_number_via_slices(zero_zero));
    assert!(Numbers::internal_number_via_slices(one_zero));
    assert!(!Numbers::internal_number_via_slices(&[one1,one2]));
    assert!(Numbers::internal_number_via_slices(&[one1,zero]));
    assert!(!Numbers::internal_number_via_slices(&[zero,zero]));

    let x : Numbers = Numbers::from_strings(&[one_one,one_one]);
    assert_eq!(x.v,[false,false]);
    let x : Numbers = Numbers::from_strings(&[one_one,one_zero]);
    assert_eq!(x.v,[false,true]);
  }
  #[test]
  fn test_nonzero_number_positions() {
    let it = Numbers { v: vec![true,false,false,true]};
    assert_eq!(it.nonzero_number_positions(),[0,3]);
    let it = Numbers { v: vec![true,true,true,true]};
    assert_eq!(it.nonzero_number_positions(),[0,1,2,3]);
    let it = Numbers { v: vec![]};
    assert_eq!(it.nonzero_number_positions(),[]);

    let it = Numbers { v: vec![false,false,false,false]};
    assert_eq!(it.nonzero_number_positions(),[]);
  }
  #[test]
  fn test_create_nonzero_numbers() {
    let it = Numbers { v: vec![false,false,false,false]};
    let ans = it.create_nonzero_numbers();
    assert!(ans.v.is_empty());

    let it = Numbers { v: vec![true,true,true,true]};
    let ans = it.create_nonzero_numbers();
    assert_eq!(ans.v.len(),4);
    assert!(ans.v[0]);
    assert!(ans.v[1]);
    assert!(ans.v[2]);
    assert!(ans.v[3]);
  }
  #[test]
  fn test_sub() {
    assert!(!Numbers::sub(true,true));
    assert!(Numbers::sub(true,false));
    assert!(Numbers::sub(false,true));
    assert!(!Numbers::sub(false,false));
  }
  #[test]
  fn test_is_zero() {
    assert!(!Numbers::is_zero(true));
    assert!(Numbers::is_zero(false));
  }
  #[test]
  fn test_generate_numbers_from_first_or_second_coefficient() {
    let it1 = Numbers{ v: vec![true,true] };
    let it2 = Numbers{ v: vec![false,false] };
    let it3 = Numbers{ v: vec![true,false] };
    let it4 = Numbers{ v: vec![true,true,true] };
    let dec = vec![];
    let (nums, keeps) = it1.generate_numbers_from_first_or_second_coefficient(
      &it2,
      &dec);
    assert!(nums.v.is_empty());
    assert!(keeps.is_empty());

    let dec = vec![FirstOrSecondCoefficient::UseFirst,
                   FirstOrSecondCoefficient::UseSecond];
    let (nums, keeps) = it1.generate_numbers_from_first_or_second_coefficient(
      &it2,
      &dec);
    assert_eq!(nums.v,[true,false]);
    assert_eq!(keeps.len(),2);
    assert_eq!(keeps[0],KeepVars::UseFirst);
    assert_eq!(keeps[1],KeepVars::UseSecond);

    let dec = vec![FirstOrSecondCoefficient::ConsiderBoth];
    let (nums, keeps) = it1.generate_numbers_from_first_or_second_coefficient(
      &it2,
      &dec);
    assert_eq!(nums.v,[true]);
    assert_eq!(keeps.len(),1);
    assert_eq!(keeps[0],KeepVars::UseFirstSkipSecond);

    let dec = vec![FirstOrSecondCoefficient::ConsiderBoth];
    let (nums, keeps) = it1.generate_numbers_from_first_or_second_coefficient(
      &it3,
      &dec);
    assert_eq!(nums.v,[]);
    assert_eq!(keeps.len(),1);
    assert_eq!(keeps[0],KeepVars::SkipBoth);

    let dec = vec![FirstOrSecondCoefficient::ConsiderBoth,
    FirstOrSecondCoefficient::UseFirst];
    let (nums, keeps) = it1.generate_numbers_from_first_or_second_coefficient(
      &it3,
      &dec);
    assert_eq!(nums.v,[true]);
    assert_eq!(keeps.len(),2);
    assert_eq!(keeps[0],KeepVars::SkipBoth);
    assert_eq!(keeps[1],KeepVars::UseFirst);

    let dec = vec![FirstOrSecondCoefficient::ConsiderBoth,
    FirstOrSecondCoefficient::UseFirst,
    FirstOrSecondCoefficient::UseFirst];
    let (nums, keeps) = it4.generate_numbers_from_first_or_second_coefficient(
      &it3,
      &dec);
    assert_eq!(nums.v,[true,true]);
    assert_eq!(keeps.len(),3);
    assert_eq!(keeps[0],KeepVars::SkipBoth);
    assert_eq!(keeps[1],KeepVars::UseFirst);
    assert_eq!(keeps[2],KeepVars::UseFirst);

    let dec = vec![FirstOrSecondCoefficient::ConsiderBoth,
    FirstOrSecondCoefficient::UseFirst,
    FirstOrSecondCoefficient::UseFirst,
    FirstOrSecondCoefficient::UseSecond];
    let (nums, keeps) = it4.generate_numbers_from_first_or_second_coefficient(
      &it3,
      &dec);
    assert_eq!(nums.v,[true,true,false]);
    assert_eq!(keeps.len(),4);
    assert_eq!(keeps[0],KeepVars::SkipBoth);
    assert_eq!(keeps[1],KeepVars::UseFirst);
    assert_eq!(keeps[2],KeepVars::UseFirst);
    assert_eq!(keeps[3],KeepVars::UseSecond);
  }
}
