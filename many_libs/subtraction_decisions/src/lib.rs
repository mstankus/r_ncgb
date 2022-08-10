// Mark Stankus (c) Sun Aug  7 18:14:24 PDT 2022

#[derive(Debug,Eq,PartialEq)]
pub enum FirstOrSecondCoefficient {
  UseFirst,
  UseSecond,
  ConsiderBoth,
}

#[derive(Debug,Eq,PartialEq)]
pub enum KeepVars {
  UseFirst,
  UseSecond,
  UseFirstSkipSecond,
  SkipBoth,
}
