// implements the IO display submodule

pub fn show(/*image: &Image*/ window_name: &str) {

}

pub fn refresh() {
  // refresh image windows

  // don't sleep any threads. Not equivalent to wait(0) 
}

pub fn wait(time: u128) {
  // refresh image windows

  if time > 0 {
    // sleep current thread for 'time'
  }
  else {
    // wait until keypress
  }
}