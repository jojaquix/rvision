use winapi::shared::minwindef::{ WORD, DWORD, HMODULE, FARPROC, BOOL, TRUE, FALSE, };
use winapi::shared::wtypesbase::{ SHORT };
use winapi::shared::ntdef::{ NULL, VOID, PVOID };

use winapi::um::handleapi::{ INVALID_HANDLE_VALUE };

use winapi::um::consoleapi::{ WriteConsoleW, ReadConsoleW };

use winapi::um::wincon::{ GetConsoleScreenBufferInfo, 
  FillConsoleOutputCharacterW,
  FillConsoleOutputAttribute,
  SetConsoleCursorPosition,
  WriteConsoleOutputCharacterW,
  WriteConsoleOutputW,
  SetConsoleTextAttribute,
  CONSOLE_SCREEN_BUFFER_INFO, 
  COORD, };

use winapi::um::wincon::{ 
  FOREGROUND_RED, 
  FOREGROUND_GREEN, 
  FOREGROUND_BLUE,
  FOREGROUND_INTENSITY, 
  BACKGROUND_RED,
  BACKGROUND_GREEN,
  BACKGROUND_BLUE,
  BACKGROUND_INTENSITY };


use winapi::um::winnt::{ HANDLE, CHAR, WCHAR };
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };

use winapi::um::processenv::{ GetStdHandle };

use std::ffi::OsString;
use std::os::windows::prelude::*;

use std::mem;
//use std::sync::atomic::{AtomicU32, Ordering};



// this is based on:
// winnt::screen.h from SET tv 
//

// Cursor position
static mut currentCursorX: u16 = 0;
static mut currentCursorY: u16 = 0; 
 
// Cursor shape
static mut curStart: i16 = 0;
static mut curEnd: u16 = 0;


// Variables for this driver
// Input/output handles
// or better use AtomicU32
static mut hOut: DWORD = 0;
static mut hIn: DWORD = 0;
 //#ifdef USE_NEW_BUFFER
 //static HANDLE hStdOut;
 //#endif

pub fn say_hi() {
  println!("Hi from Windows");
}


pub fn init() {
  let  h_stdout: HANDLE = INVALID_HANDLE_VALUE;
  unsafe {
      let h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
      if h_stdout == INVALID_HANDLE_VALUE {
        panic!("Error in low level access to console init");
      }
      hOut = h_stdout as DWORD;
  }  
}


/// pre: init must be called
pub fn clear() {
  let mut csbi : CONSOLE_SCREEN_BUFFER_INFO =  unsafe { std::mem::zeroed() }; 
  let success: BOOL;
  let coordScreen: COORD = unsafe { std::mem::zeroed() };  // home for the cursor 
  let mut cCharsWritten: DWORD = 0u32 ;
  let dwConSize: DWORD ;
  
  unsafe {
      let h_stdout = hOut as HANDLE;
      success = GetConsoleScreenBufferInfo(h_stdout, &mut csbi);
      
      if success == FALSE {
        panic!("Error in low level access to console clear");
      }        

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
          let h_stdout = hOut as HANDLE;
          SetConsoleCursorPosition( h_stdout, coordScreen );
      }  
}

pub fn get_cursor_pos() -> (u16, u16) {
  let mut console_scren_inf : CONSOLE_SCREEN_BUFFER_INFO =  unsafe { std::mem::zeroed() }; 
  let success: BOOL;
  unsafe {
      let h_stdout = hOut as HANDLE;
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
    let h_stdout = hOut as HANDLE;
    success = GetConsoleScreenBufferInfo(h_stdout, &mut console_scren_inf);
  }

  if success == TRUE {
      let columns = console_scren_inf.srWindow.Right - console_scren_inf.srWindow.Left + 1;
      let rows = console_scren_inf.srWindow.Bottom - console_scren_inf.srWindow.Top + 1;
      return (rows as u16, columns as u16);
  }

  panic!("Error in low level access to console size");

}


pub fn set_color(_color: u16) {
  let success: BOOL;
  let color2 = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
  unsafe {
    let h_stdout = hOut as HANDLE;
    success = SetConsoleTextAttribute(h_stdout, color2);
  }

  if success == FALSE {
    panic!("Error in low level access to console set color");
  }
}

pub fn write_string(x: u16, y: u16, s: String) {
  let success: BOOL;  
  let mut written: DWORD = 0;

  let os_string = OsString::from(s);
  let wide_encoded: Vec<u16> = os_string.encode_wide().collect();

  unsafe {
    let h_stdout = hOut as HANDLE;
    success = WriteConsoleW(h_stdout, wide_encoded.as_ptr() as PVOID, wide_encoded.len() as DWORD, &mut written, NULL)
  }
  if success == FALSE {
    panic!("Error in low level access to console writing string");
  }  
}

pub fn write_nchar(x: u16, y: u16, c: char, count: i16) {
  
  let string = (0..count).map(|_| c).collect::<String>();
  let os_string = OsString::from(string);
  let wide_encoded: Vec<u16> = os_string.encode_wide().collect();
  let success: BOOL;
  let mut written: DWORD = 0;
  set_cursor_pos(x, y);
  
  if !wide_encoded.is_empty() {
    unsafe {
      let h_stdout = hOut as HANDLE;
      success = WriteConsoleW(h_stdout, wide_encoded.as_ptr() as PVOID, wide_encoded.len() as DWORD, &mut written, NULL)
    }
    if success == FALSE {
      panic!("Error in low level access to console writing nchar");
    } 
  } 
}

pub fn write_char(x: u16, y: u16, c: char) {
  write_nchar(x, y, c, 1);
}


