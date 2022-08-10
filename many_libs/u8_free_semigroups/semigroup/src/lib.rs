use std::collections::HashMap;

#[allow(dead_code)]
type LetterSmallType = u8;
#[allow(dead_code)]
type LetterLargeType = char;
#[allow(dead_code)]

#[allow(dead_code)]
struct Alphabet {
  large2small_data : HashMap<LetterLargeType, LetterSmallType>,
  small2large_data : Vec<LetterLargeType>,
}

#[allow(dead_code)]
impl Alphabet {
  fn new() -> Alphabet {
    Alphabet { large2small_data : HashMap::new(), 
               small2large_data : Vec::new()}
  }
  fn set_large_small(&mut self, a : &LetterLargeType) {
    let len = self.small2large_data.len();
    self.large2small_data.insert(*a,len as u8);
    self.small2large_data.push(*a);
  }
  fn to_small(&self,x : LetterLargeType) -> LetterSmallType {
    *self.large2small_data.get(&x).unwrap()
  }
  fn to_large(&self,x : LetterSmallType) -> LetterLargeType {
    self.small2large_data[x as usize]
  }
  fn is_large_recorded(&self,x : LetterLargeType) -> bool {
    self.large2small_data.contains_key(&x)
  }
  fn is_small_recorded(&self,x : LetterSmallType) -> bool {
    (x as usize) < self.small2large_data.len()
  }
}

#[allow(dead_code)]
struct WordSlice<'a> {
  data : &'a [u8],
}

#[allow(dead_code)]
impl WordSlice<'_> {
  fn new(slice : &[u8]) -> WordSlice { 
    WordSlice { data : slice }
  }
}

struct Word {
  data : Vec<u8>,
}

#[allow(dead_code)]
impl Word {
  fn new(s : &[u8]) -> Word {
    let mut data = Vec::with_capacity(s.len());
    data.extend_from_slice(s);
    Word { data }
  }
  fn len(&self) -> usize {
    self.data.len()
  }
  fn letter(&self,n : usize) -> LetterSmallType {
    self.data[n]
  }
}

#[allow(dead_code)]
#[derive(Clone)]
struct Words {
  data : Vec<u8>,
  deg_cnt : Vec<(usize,usize)>,
  number_mono : usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct WordsIterator<'a> {
  data    : &'a [u8],
  deg_cnt : &'a [(usize,usize)],
  deg     : usize,
  cnt     : usize,
}

#[allow(dead_code)]
impl<'a> WordsIterator<'a> {
  pub fn new(x : &'a Words) -> WordsIterator {
    let cnt = 0;
    let deg = match x.deg_cnt.len() {
      0 => { 
        0
      }
      _ => { 
        x.deg_cnt[0].0
      }
    };
    WordsIterator { 
      data   : &x.data[..], 
      deg_cnt : &x.deg_cnt[..],
      deg,
      cnt,
    }
  }
}

#[allow(dead_code)]
impl<'a> Iterator for WordsIterator<'a> {
  type Item = &'a [u8];

  fn next(&mut self) -> Option<Self::Item> {
    println!("Before {:?}",self);
    if self.data.is_empty() {
      println!("Unchanged");
      Option::<Self::Item>::None
    } else {
      let it = Some(&self.data[0..self.deg]);
      self.data = &self.data[self.deg..];
      self.cnt += 1;
      if self.cnt==self.deg_cnt[0].1 {
        // Move on to the next chunk
        self.deg_cnt = &self.deg_cnt[1..];
        self.cnt = 0;
        if self.data.is_empty(){
          self.deg = 0;
        } else {
          self.deg = self.deg_cnt[0].0;
        }
      }
      println!("After {:?}",self);
      it
    }
  }
}

#[allow(dead_code)]
impl Words {
  pub fn new() -> Words {
    Words { data : vec![], deg_cnt : vec![], number_mono: 0usize }
  }
  pub fn extend_from_slices(&mut self,
                            slices : &[&[u8]]) {
    if slices.is_empty() { return;}
    let deg = slices[0].len();
    self.deg_cnt.push((deg,slices.len()));
    for i in slices.iter() {
      assert_eq!(deg,i.len());
      self.data.extend_from_slice(i);
      self.number_mono += 1;
    }
  }
  pub fn multiple_extend_from_slices(
      &mut self,many_slices : &[&[&[u8]]]) {
    for slices in many_slices.iter() {
      self.extend_from_slices(slices);
    }
  }
  // Test this function before moving on.
  pub fn multiply_on_left(&mut self,l : &[u8]) {
    let l_len = l.len();

    // Create newdata and assign it to `self.data`. 
    // Use the unmodified `self.deg_cnt`.
    let new_vlen = self.data.len()+self.number_mono*l_len;
    let mut newdata = Vec::with_capacity(new_vlen);
    let mut pos = 0;
    for i in self.deg_cnt.iter() {
      let (deg,cnt) = *i;
      for _ in 0..cnt {
        // Copy the left factor.
        newdata.extend_from_slice(l);
        // Copy the original monomial.
        for _ in 0..deg {
          newdata.push(self.data[pos]);
          pos += 1;
        }
      }
    }
    self.data = newdata;

    // Adjust deg_cnt
    for i in self.deg_cnt.iter_mut() {
      i.0 += l_len;
    }
  }
  pub fn multiply_on_right(&mut self,r : &[u8]) {
    let r_len = r.len();

    // Create newdata and assign it to `self.data`. 
    // Use the unmodified `self.deg_cnt`.
    let new_vlen = self.data.len()+self.number_mono*r_len;
    let mut newdata = Vec::with_capacity(new_vlen);
    let mut pos = 0;
    for i in self.deg_cnt.iter() {
      let (deg,cnt) = *i;
      for _ in 0..cnt {
        // Copy the original monomial.
        for _ in 0..deg {
          newdata.push(self.data[pos]);
          pos += 1;
        }
        // Copy the right factor.
        newdata.extend_from_slice(r);
      }
    }
    self.data = newdata;

    // Adjust deg_cnt
    for i in self.deg_cnt.iter_mut() {
      i.0 += r_len;
    }
  }
  pub fn iter(&self) -> WordsIterator {
    WordsIterator::new(self)
  }
  pub fn multiply_on_both(&mut self,l : &[u8],r : &[u8]) {
    let l_len = l.len();
    let r_len = r.len();
    let lr_len = l_len+r_len;

    // Create newdata and assign it to `self.data`. 
    // Use the unmodified `self.deg_cnt`.
    let new_vlen = self.data.len()+self.number_mono*lr_len;
    let mut newdata = Vec::with_capacity(new_vlen);
    let mut pos = 0;
    for i in self.deg_cnt.iter() {
      let (deg,cnt) = *i;
      for _ in 0..cnt {
        // Copy the left factor.
        newdata.extend_from_slice(l);
        // Copy the original monomial.
        for _ in 0..deg {
          newdata.push(self.data[pos]);
          pos += 1;
        }
        // Copy the right factor.
        newdata.extend_from_slice(r);
      }
    }
    self.data = newdata;

    // Adjust deg_cnt
    for i in self.deg_cnt.iter_mut() {
      i.0 += lr_len;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_alphabet() {
    let mut alpha = Alphabet::new();
    assert_eq!(alpha.large2small_data.len(),0);
    assert_eq!(alpha.small2large_data.len(),0);
    alpha.set_large_small(&'a');
    let small= alpha.to_small('a');
    assert_eq!('a',alpha.to_large(small));

    assert!(alpha.is_small_recorded(small));
    assert!(alpha.is_large_recorded('a'));
    assert!(!alpha.is_large_recorded('b'));
  }
  #[test]
  fn test_word() {
    let wd = Word::new(&[0,2,2]);
    assert_eq!(wd.len(),3);
    assert_eq!(wd.letter(0),0);
    assert_eq!(wd.letter(1),2);
    assert_eq!(wd.letter(2),2);
  }
  #[test]
  fn test_extend_from_slices_1() {
    let mut wds = Words::new();
    wds.extend_from_slices( &[   &[1,2], &[4,4], &[3,0]   ]);
    assert_eq!(wds.data,[1,2,4,4,3,0]);
    assert_eq!(wds.deg_cnt,[(2,3)]);
  }
  #[test]
  fn test_extend_from_slices_2() {
    let mut wds = Words::new();
    wds.extend_from_slices( &[   &[1,2], &[4,4], &[3,0]   ]);
    wds.extend_from_slices( &[   &[6,6,6]   ]);
    assert_eq!(wds.data,[1,2,4,4,3,0,6,6,6]);
    assert_eq!(wds.deg_cnt,[(2,3),(3,1)]);
  }
  #[test]
  fn test_multiple_extend_from_slices_1() {
    let mut wds = Words::new();
    wds.multiple_extend_from_slices(
      &[
         &[   &[1,2], &[4,4], &[3,0]   ],
         &[   &[6,6,6] ]
      ]
      );
    assert_eq!(wds.data,[1,2,4,4,3,0,6,6,6]);
    assert_eq!(wds.deg_cnt,[(2,3),(3,1)]);
  }
  #[test]
  fn test_multiply_on_left() {
    let mut wds = Words::new();
    wds.multiple_extend_from_slices(
      &[
         &[   &[1,2], &[4,4], &[3,0]   ],
         &[   &[6,6,6] ]
      ]
      );
    assert_eq!(wds.data,[1,2,4,4,3,0,6,6,6]);
    assert_eq!(wds.deg_cnt,[(2,3),(3,1)]);

    wds.multiply_on_left(&[5,5]);
    assert_eq!(wds.data,[5,5,1,2,5,5,4,4,5,5,3,0,5,5,6,6,6]);
    assert_eq!(wds.deg_cnt,[(4,3),(5,1)]);
  }
  #[test]
  fn test_multiply_on_right() {
    let mut wds = Words::new();
    wds.multiple_extend_from_slices(
      &[
         &[   &[1,2], &[4,4], &[3,0]   ],
         &[   &[6,6,6] ]
      ]
      );
    assert_eq!(wds.data,[1,2,4,4,3,0,6,6,6]);
    assert_eq!(wds.deg_cnt,[(2,3),(3,1)]);

    wds.multiply_on_right(&[5,5]);
    assert_eq!(wds.data,[1,2,5,5,4,4,5,5,3,0,5,5,6,6,6,5,5]);
    assert_eq!(wds.deg_cnt,[(4,3),(5,1)]);
  }
  #[test]
  fn test_multiply_on_both() {
    let mut wds = Words::new();
    wds.multiple_extend_from_slices(
      &[
         &[   &[1,2], &[4,4], &[3,0]   ],
         &[   &[6,6,6] ]
      ]
      );
    assert_eq!(wds.data,[1,2,4,4,3,0,6,6,6]);
    assert_eq!(wds.deg_cnt,[(2,3),(3,1)]);
    let mut i = wds.iter();
    let v = vec![1,2];
    assert_eq!(i.next(),Some(&v[..]));
    let v = vec![4,4];
    assert_eq!(i.next(),Some(&v[..]));
    let v = vec![3,0];
    assert_eq!(i.next(),Some(&v[..]));
    let v = vec![6,6,6];
    assert_eq!(i.next(),Some(&v[..]));
  }
}
