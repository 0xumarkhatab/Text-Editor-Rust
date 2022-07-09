extern crate colored;
extern crate term_cursor as cursor;
use colored::*;
#[warn(dead_code)]
use std::{thread,time};
use std::io::{stdin, self};
use std::string::FromUtf16Error;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Stylize;
/* modify */
use crossterm::{event, terminal};
/* add this line */
struct CleanUp;

fn clrscr() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_text_contents(string: &mut String, x_max: u64, y_max: u64, start_y: u64) -> u64 {
 
    let mut i = 0;
    let mut y_coord = start_y;
    let mut snaps=String::new();
    while i < string.len() {
        let mut final_x: usize = x_max.try_into().unwrap();
        if (i + final_x >= string.len()) {
            final_x = string.len() - i;
        } else {
            y_coord = y_coord + 1;
        }
        let mut snap = &string[i..(i + final_x)];
        //println!("{}", snap);
        snaps.push_str(snap.clone());
        snaps.push_str("\n");
        
        i = i + (final_x);
        if y_coord > y_max {
            return y_coord;
        }
    }

    clrscr();
    println!(
        "{}{}\n{}",
        format!("{}", format!("\t\t\tR-Edit").dark_green().bold()),
        format!("\nMenu -> \t{}{}{}", format!("CTRL+S => Save").bold(),
        format!("ALTL+Q => Exit").bold(),
        format!("Enter and Delete Key features are not enabled yet.").bold()).underline_dark_green(),
        snaps
    );

    return y_coord;
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn print_bottom(string: &str, y_max: u64, margin: u64) {
    set_cursor_position(30, y_max + margin);
    println!("{}", format!("{}", string).bold());

}

fn set_cursor_position(x: u64, y: u64) {
    cursor::set_pos(x.try_into().unwrap(), y.try_into().unwrap());
}

fn the_delay(n:u64){
    let half_second=time::Duration::from_millis(n*500);
    let now=time::Instant::now();
    thread::sleep(half_second);
    assert!(now.elapsed()>=half_second);
    
    
}

fn startup(){
    cursor::clear();
   let mut dots=String::from(".");
   let colors=["green","red","cyan","aqua","blue","orange","yellow","maroon"];
   let n=colors.len();
   for i in (0..9) {
    clrscr();
    println!("{}",format!("\t\t\tWelcome to {} \n\t\t\tThe Rust Based Text Editor",format!("R-Edit").color(colors[(i*2)%n])) );
    println!("{}{}",format!("\n\n\t\t\tLoading").color(colors[(i*3)%n]),format!("{}",dots).color(colors[(i*6)%n]).bold());
    if i%2 !=0{
        dots.push('.');

    }
    
    
    the_delay(1);
    

   }


}

fn main() -> crossterm::Result<()> {

// ________________     Variables Declaration      __________________


    let _clean_up = CleanUp;
    
    let start_y: u64 = 2;
    let mut current_y = start_y;
    let mut x_coord: u64 = 0;
    let mut y_coord: u64 = start_y;
    let x_max = 100;
    let y_max =100;
    let mut undoStack: Vec<String> = Vec::new();
    let mut redoStack: Vec<String> = Vec::new();
    let mut loopCounter = 0;
    let mut input = String::from("");


// ________________     Variables Declaration   Ended   __________________

//  _______________             Startup Menu            __________________

startup();


//      Taking control of Terminal 
    
    terminal::enable_raw_mode()?;

//  _________________________________    

//      Opening the Text Editor Interface    
set_cursor_position(x_coord, y_coord);
print_text_contents(&mut input.clone(), x_max, y_max, start_y);


    Ok(loop {
        set_cursor_position(x_coord, y_coord);
        loopCounter += 1;
        if let Event::Key(event) = event::read().expect("Reading Error from terminal") {
          //Now We are Reading from Termiinal and Match the Key Events  


          //        Matching Control Keys Events
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
                        } else if x_coord > input.len() as u64 {
                            x_coord -= 1;
                            continue;
                        } else {
                            let mut index = (y_coord - start_y) * x_max + x_coord - 1;

                            if index >= input.len() as u64 {
                                undoStack.push(input.clone());
                                index = (input.len() as u64) - 1;
                                input.remove(index.try_into().unwrap()).to_string();
                            } else {
                                undoStack.push(input.clone());
                                input.remove(index.try_into().unwrap() ).to_string();
                            }
                            print_text_contents(&mut input, x_max, y_max, start_y);

                            if x_coord >= 1 {
                                x_coord -= 1;
                            }
                            if (x_coord == 0 && current_y > start_y) {
                                current_y -= 1;
                            }
                        }
                    }
                }

                //      __________ Matching control Keys Ended _____________ 


                //      __________ Having 
                _ => {
                    let c = event.code;
                    
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

                                if index >= input.len() as u64 {
                                    input.push_str(String::from(s).as_str());
                                } else if index > 0 {
                                    input.insert(index as usize, s);
                                } else {
                                    input.insert((index - 1) as usize, s);
                                }

                                current_y = print_text_contents(&mut input, x_max, y_max, start_y);
                                if (current_y != y_coord) {
                                    y_coord = current_y;
                                    x_coord = (input.len() as u64 % x_max - 1);
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
