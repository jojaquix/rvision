

const MAX_VIEW_WIDTH :usize = 1024;

// 
// may be a lot of memory use ???
// it would be use only by draw methods anyway
// remember char is 4 bytes 
pub struct TDrawBuffer {
  data: [char; MAX_VIEW_WIDTH],
  attrs: [u8; MAX_VIEW_WIDTH]
}


impl TDrawBuffer {
  pub fn new() -> Self {
      TDrawBuffer { data: [' '; MAX_VIEW_WIDTH], attrs: [0u8; MAX_VIEW_WIDTH] }
  }

  pub fn move_char(&mut self, indent: u16, c: char, attr: u8, count: u8) { 
      
    if count==0 || (indent >= MAX_VIEW_WIDTH as u16) { return; }
    let mut count2 = count;
    if count as u16 + indent > MAX_VIEW_WIDTH as u16
      { let count2 = MAX_VIEW_WIDTH as u16 - indent; }

    let mut i: usize = 0;
    if attr != 0 {
      //TODO: how to use attribs ??
    }
    else {
      while count2 > 0 {
        let count2 = count2 -1;
        self.data[indent as usize +i] = c;
        let i = i + 1;
      }
    }
  }
}