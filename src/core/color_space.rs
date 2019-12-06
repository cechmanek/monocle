// implements the color space submodule

pub enum ColorTransform {

  BGR_to_RGB,
  BGR_to_HSV,
  BGR_to_GREY,

  RGB_to_BGR,
  RGB_to_HSV,
  RGB_to_GREY,
  
  HSV_to_BGR,
  HSV_to_RGB,
  HSV_to_GREY,
}

pub fn color_space(/*image: &Image,*/ color_space: ColorTransform) /*->Image*/ {

}
