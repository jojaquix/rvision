use winapi::shared::minwindef::{ WORD, DWORD, HMODULE, FARPROC, BOOL, TRUE, FALSE, };
use winapi::shared::wtypesbase::{ SHORT };
use winapi::um::wincon::{ GetConsoleScreenBufferInfo, 
                          FillConsoleOutputCharacterW,
                          FillConsoleOutputAttribute,
                          SetConsoleCursorPosition,
                          WriteConsoleOutputCharacterW,
                          WriteConsoleOutputCharacterA,
                          CONSOLE_SCREEN_BUFFER_INFO, 
                          COORD, };
use winapi::um::winnt::{ HANDLE, CHAR, WCHAR };
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::processenv::{GetStdHandle};

use std::ffi::OsString;
use std::os::windows::prelude::*;

use std::mem;


pub fn say_hi() {
  println!("Hi from Windows");
}

pub fn clear() {
  let mut csbi : CONSOLE_SCREEN_BUFFER_INFO =  unsafe { std::mem::zeroed() }; 
  let success: BOOL;
  let coordScreen: COORD = unsafe { std::mem::zeroed() };  // home for the cursor 
  let mut cCharsWritten: DWORD = 0u32 ;    
  let dwConSize: DWORD ;
  
  unsafe {
      let h_stdout: HANDLE;
      h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
      success = GetConsoleScreenBufferInfo(h_stdout, &mut csbi);
      dwConSize = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;
      
      FillConsoleOutputCharacterW( h_stdout,   // Handle to console screen buffer 
          ' ' as WCHAR,                        // Character to write to the buffer
          dwConSize,                          // Number of cells to write 
          coordScreen,                        // Coordinates of first cell 
          &mut cCharsWritten);                // Receive number of characters written        

      FillConsoleOutputAttribute( h_stdout,         // Handle to console screen buffer 
          csbi.wAttributes, // Character attributes to use
          dwConSize,        // Number of cells to set attribute 
          coordScreen,      // Coordinates of first cell 
          &mut cCharsWritten ); // Receive number of characters written

      SetConsoleCursorPosition( h_stdout, coordScreen );
      }
}


pub fn get_rows() -> u16 {
  let size = get_size();
  let (rows,_) = size;
  return rows;
}

pub fn get_cols() -> u16 {
  let size = get_size();
  let (_, cols) = size;
  return cols;
}


pub fn set_cursor_pos(x: u16, y: u16 ) {
  let coordScreen: COORD = COORD {X:x as i16, Y:y as i16};  // pos for the cursor
  unsafe {
          let h_stdout: HANDLE;
          h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
          SetConsoleCursorPosition( h_stdout, coordScreen );
      }  
}

pub fn get_cursor_pos() -> (u16, u16) {
  let mut console_scren_inf : CONSOLE_SCREEN_BUFFER_INFO =  unsafe { std::mem::zeroed() }; 
  let success: BOOL;
  unsafe {
      let h_stdout: HANDLE;
      h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
      success = GetConsoleScreenBufferInfo(h_stdout, &mut console_scren_inf);
  }

  if success == TRUE {
      let cursor_position = console_scren_inf.dwCursorPosition;
      return (cursor_position.X as u16, cursor_position.Y as u16);
  }

  panic!("Error in low level access to console size");  
}

/// return (row, cols)
fn get_size() -> (u16, u16) {
  let mut console_scren_inf : CONSOLE_SCREEN_BUFFER_INFO =  unsafe { std::mem::zeroed() }; 
  let success: BOOL;
  unsafe {
      let h_stdout: HANDLE;
      h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
      success = GetConsoleScreenBufferInfo(h_stdout, &mut console_scren_inf);
  }

  if success == TRUE {
      let columns = console_scren_inf.srWindow.Right - console_scren_inf.srWindow.Left + 1;
      let rows = console_scren_inf.srWindow.Bottom - console_scren_inf.srWindow.Top + 1;
      return (rows as u16, columns as u16);
  }

  panic!("Error in low level access to console size");

}


pub fn write_char(x: u16, y: u16, c: char) {
  let success: BOOL;
  let coord_screen = COORD {X: x as i16, Y: y as i16};  // pos for the cursor
  let mut written: DWORD = 0;

  let string = c.to_string();
  let os_string = OsString::from(string);
  let wide_encoded: Vec<u16> = os_string.encode_wide().collect();

  unsafe {
    let h_stdout: HANDLE;
    h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
    success = WriteConsoleOutputCharacterW(h_stdout, wide_encoded.as_ptr(), wide_encoded.len() as u32, coord_screen, &mut written);
  }
  if success == FALSE {
    panic!("Error in low level access to console writing");
  }
}






