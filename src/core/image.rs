// implements the image structure

pub enum Channels {
  One, // for grayscale and binary images
  Three, // for BGR or RBG images
}

pub enum Color {
  RGB(Channels), // must be 3 channels
  BGR(Channels), // must be 3 channels 
  Binary(Channels), // must be 1 channel
  Gray(Channels), // must be 1 channel 
}

pub struct Image {
  height: u64,
  width: u64,
  channels: Channels,
  color: Color, 
}

impl Image {
  pub fn channels(self) -> Channels {
    return self.channels;
  } 

  pub fn color(self) -> Color {
    return self.color;
  }
}