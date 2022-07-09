extern crate colored;
extern crate term_cursor as cursor;
use colored::*;
#[warn(dead_code)]
use std::io::{stdin, self};

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Stylize;
/* modify */
use crossterm::{event, terminal};
/* add this line */
struct CleanUp;

fn get_current_line(string: String, n: u8, x_max: u8, y_max: u8) {
    let mut i = 0;
    let mut numChunks = 0;
    while i < string.len() && numChunks < n {
        let mut final_x: usize = x_max.into();
        if (i + final_x >= string.len()) {
            final_x = string.len() - i;
        }
        let snap = &string[i..(i + final_x)];
        println!("{}", snap);
        i = i + (final_x);
    }
}

fn clrscr() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_text_contents(string: &mut String, x_max: u8, y_max: u8, start_y: u8) -> u8 {
    clrscr();
    println!(
        "{}",
        format!("{}", format!("\t\t\tText Editor in Rust").green().bold()).underline_blue()
    );

    let mut i = 0;
    let mut y_coord = start_y;
    while i < string.len() {
        let mut final_x: usize = x_max.into();
        if (i + final_x >= string.len()) {
            final_x = string.len() - i;
        } else {
            y_coord = y_coord + 1;
        }
        let snap = &string[i..(i + final_x)];
        println!("{}", snap);
        i = i + (final_x);
        if y_coord > y_max {
            return y_coord;
        }
    }

    return y_coord;
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn print_bottom(string: &str, y_max: u8, margin: u8) {
    set_cursor_position(30, y_max + margin);

    println!("{}", format!("{}", string).bold());
}
fn set_cursor_position(x: u8, y: u8) {
    cursor::set_pos(x.into(), y.into());
}
fn get_num_lines(string:String,x_max:u8)->i32{

    let mut i = 0;
    let mut counter=0;
    while i < string.len() {
        i = i +x_max as usize ;
        counter+=1;
        
    }
    return counter;    
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    /* add the following */
    let start_y: u8 = 1;
    let mut current_y = start_y;
    let mut x_coord: u8 = 0;
    let mut y_coord: u8 = start_y;
    let x_max = 20;
    let y_max = 8;
    let mut undoStack: Vec<String> = Vec::new();
    let mut redoStack: Vec<String> = Vec::new();

    let mut input = String::from("");
    print_text_contents(&mut input.clone(), x_max, y_max, start_y);

    let mut loopCounter = 0;

    Ok(loop {
        set_cursor_position(x_coord, y_coord);
        //        println!("x_coord={} y_coord={} counter={}",x_coord,y_coord,loopCounter);
        loopCounter += 1;

        if let Event::Key(event) = event::read().expect("Reading Error from terminal") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::ALT,
                } => {
                    //     set_cursor_position(30,(y_max+4) );

                    println!("Quitting !");
                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    println!("Interrupted !");
                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    
                    print_bottom("\nDo You want to Save your Work ? ", y_max, 4);
                    {
                       let _clean_up_ = CleanUp;
                    }
                    let mut valid_statement=false;
                    while valid_statement==false{
                        println!("Y -> Yes\nN ->No");
                        let mut  user_save_option=String::new();
                        let _bytes_read= stdin().read_line(&mut user_save_option);
                        let mut user_save_option =(user_save_option.trim()) ;
    
                        match user_save_option{
                            "Y" | "y"=>{
                                println!("Saving the file....");
                                valid_statement=true;
                            }
                            "N" | "n"=>{
                                println!("File is not intended to be Saved");
                                valid_statement=true;
                            }
                            
                            _=>{
                                println!("\n\n\t\tWhoopsInvalid Choice ....\nEnter Again to Save the Work\n")
                                
                            }
                        }
                        
                    }
                    
                    break;
                    
                }

                KeyEvent {
                    code: KeyCode::Char('z'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    redoStack.push(input.clone());
                    input = match undoStack.pop() {
                        Some(s_) => s_,
                        None => input,
                    };
                    print_text_contents(&mut input, x_max, y_max, start_y);
                }
                KeyEvent {
                    code: KeyCode::Char('y'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    undoStack.push(input.clone());
                    input = match redoStack.pop() {
                        Some(s_) => s_,
                        None => input,
                    };
                    print_text_contents(&mut input, x_max, y_max, start_y);
                }

                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    if (x_coord >= 1) {
                        x_coord -= 1;
                    }
                    if (x_coord == 0 && current_y > start_y && y_coord - 1 >= start_y) {
                        y_coord -= 1;
                    }

                }
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    if (x_coord + 1 <= x_max) {
                        x_coord += 1;
                    } else {
                        if y_coord + 1 <= y_max && y_coord + 1 < current_y {
                            y_coord += 1;
                        }
                    }

                }
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    if (y_coord - 1 >= start_y) {
                        y_coord -= 1;
                    }

                }

                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    
                    if (y_coord + 1 <= current_y ) {
                        if (y_coord + 1 < y_max) {
                            y_coord += 1;
                        } else {
                            y_coord = start_y;
                        }
                    } else {
                        y_coord = start_y;
                    }

                    //       set_cursor_position(x_coord,y_coord);
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: event::KeyModifiers::NONE,
                } => {

                    if input.len() > 0 {
                        if x_coord == 0 && y_coord > start_y {
                            y_coord -= 1;
                            x_coord = x_max;
                            set_cursor_position(x_coord, y_coord);
                        } else if x_coord == 0 && y_coord == start_y {
                            //backspace
                        } else if x_coord > input.len() as u8 {
                            x_coord -= 1;
                            continue;
                        } else {
                            let mut index = (y_coord - start_y) * x_max + x_coord - 1;

                            if index >= input.len() as u8 {
                                undoStack.push(input.clone());
                                index = (input.len() as u8) - 1;
                                input.remove(index.into()).to_string();
                            } else {
                                undoStack.push(input.clone());
                                input.remove((index).into()).to_string();
                            }
                            print_text_contents(&mut input, x_max, y_max, start_y);

                            if x_coord >= 1 {
                                x_coord -= 1;
                            }
                            if (x_coord == 0 && current_y > start_y) {
                                current_y -= 1;
                            }
                            //               set_cursor_position(x_coord,y_coord);
                        }
                    }
                }

                _ => {
                    let c = event.code;
                    //             set_cursor_position(x_coord, y_coord);
                    match c {
                        KeyCode::Enter => {
                            if y_coord + 1 <= y_max {
                                input.push_str(String::from(".").as_str());
                                x_coord += 1;
                                //                           set_cursor_position(x_coord, y_coord);
                                print_text_contents(&mut input, x_max, y_max, start_y);
                                undoStack.push(input.clone());
                            } else {
                                print_bottom("The Text has reached to maximum Capacity", y_max, 4);
                            }
                        }
                        KeyCode::Left => todo!(),
                        KeyCode::Right => todo!(),
                        KeyCode::Up => todo!(),
                        KeyCode::Down => todo!(),
                        KeyCode::Home => todo!(),
                        KeyCode::End => todo!(),
                        KeyCode::PageUp => todo!(),
                        KeyCode::PageDown => todo!(),
                        KeyCode::Tab => todo!(),
                        KeyCode::BackTab => todo!(),
                        KeyCode::Delete => todo!(),
                        KeyCode::Insert => todo!(),
                        KeyCode::F(_) => todo!(),
                        KeyCode::Null => todo!(),
                        KeyCode::Esc => todo!(),
                        KeyCode::Char(s) => {
                            if (current_y <= y_max) {
                                let mut index = (y_coord - start_y) * x_max + x_coord;

                                if index >= input.len() as u8 {
                                    input.push_str(String::from(s).as_str());
                                } else if index > 0 {
                                    input.insert(index as usize, s);
                                } else {
                                    input.insert((index - 1) as usize, s);
                                }

                                current_y = print_text_contents(&mut input, x_max, y_max, start_y);
                                if (current_y != y_coord) {
                                    y_coord = current_y;
                                    x_coord = (input.len() as u8 % x_max - 1);
                                }

                                if (x_coord + 1 <= x_max) {
                                    x_coord = x_coord + 1;
                                } else {
                                }
                            } else {
                                print_bottom("The Text has reached to maximum Capacity", y_max, 4);
                            }
                        }
                        _ => println!("Invalid "),
                    }
                }
            }
        }
    })
}
