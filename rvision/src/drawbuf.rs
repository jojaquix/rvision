

pub const MAX_VIEW_WIDTH :usize = 1024;

/**
 * From tvision docs
 * 
 * This class implements a video buffer.
 *
 * TDrawBuffer implements a simple, non-view buffer class with member
 * functions for moving characters, attributes, and strings to and from a draw
 * buffer.
 *
 * Every view uses at least one istance of this class in its draw() method.
 * The view draws itself using a TDrawBuffer object. Just before returning
 * from draw(), a call to one of the writeXXXX methods will write the video
 * buffer on the screen.
 * @see TView::draw
 *
 * Each member of the buffer is an attribute & character pair. The attribute
 * is a byte which stores information about foreground and background colors.
 *
 * The contents of a draw buffer are typically used with
 * @ref TView::writeBuf() or @ref TView::writeLine() to display text.
 * @see TView::writeChar
 * @see TView::writeStr
 *
 * Note: pay attention to the size of the buffer! It usually stores only a
 * line of the picture. Its default size is @ref maxViewWidth = 132 pairs.
 * @short Implements a video buffer
 */

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


  /**
   * not present in original tvision
   * just if later i learn a better way
   * to do this
   */

  pub fn get_char_at(&self, index: usize) -> char {
    return self.data[index];
  }

  pub fn get_attr_at(&self, index: usize) -> u8 {
    return self.attrs[index];
  }
  

  /**
   * Fills the buffer or part of the buffer with an uniform pattern.
   *
   * `indent' is the character position within the buffer where the data is
   * to go. `c' is the character to be put into the buffer. If `c' is 0 the
   * character is not written and the old character is preserved. `attr' is
   * the attribute to be put into the buffer. If `attr' is 0 the attribute
   * is not written and the old attribute is preserved. `count' is the
   * number of character/attribute pairs to put into the buffer.
   */

  pub fn move_char(&mut self, indent: u16, c: char, attr: u8, count: u8) { 
      
    if count==0 || (indent >= MAX_VIEW_WIDTH as u16) { return; }
    let mut count2: u16 = count as u16;
    if count as u16 + indent > MAX_VIEW_WIDTH as u16
      {  count2 = MAX_VIEW_WIDTH as u16 - indent; }

    let mut i: usize = 0;
  
      while count2 > 0 {
        count2 = count2 -1;
        self.data[indent as usize +i ] = c;
        if attr != 0 {
          self.attrs[indent as usize + i] = attr;
        }
        i = i + 1;
      }
  }

  pub fn move_string(&mut self, indent: u16, string: String, attr: u8) {
    
  }




}