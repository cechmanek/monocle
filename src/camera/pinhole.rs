//! implements the pinhole camera model structure

use crate::core::image::Channels;
use crate::core::image::Color;
use crate::core::image::Image;
use crate::geometry::points::Point;

pub struct PinHole {
  
  // focal length of the calibrated camera measured in units of pixels
  focal_length: Option<f64>,

  // wether or not the camera model instance is calibrated
  is_calibrated: bool,  

}

impl PinHole {

  pub fn new() -> PinHole {
    return PinHole{focal_length: None, is_calibrated: false};
  }

  pub fn calibrate(self, points: &Vec<Point>) -> bool {

    return self.is_calibrated;
  }

  pub fn project_to_plane(self, points: &Vec<Point>/*, plane: &Plane*/) /*-> Vec<Point>*/ {

  }

  pub fn project_to_image(self, points: &Vec<Point>, image: &Image) /*-> Image */ {
    /*
    let return_image = Image{ height: 0,
                              width: 0,
                              channels: Channels::Three,
                              color: Color::RGB(Channels::Three)}; 

    return return_image;
    */
  }


}