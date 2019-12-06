// implements geometry transformation submodule

pub struct Transform4x4 {
  // 4x4 array
}

impl Transform4x4 {
  pub fn magnitude(self) -> f64 {
    return 1.0;
  }

  pub fn inverse(self) -> Transform4x4 {
    return self;
  }
 
  pub fn transpose(self) -> Transform4x4 {
    return self;
  }
}


pub struct Transform3x3 {
  // 3x3 array
}

impl Transform3x3 {
   pub fn magnitude(self) -> f64 {
    return 1.0;
  }

  pub fn inverse(self) -> Transform3x3 {
    return self;
  }
  
  pub fn transpose(self) -> Transform3x3 {
    return self;
  }
}
