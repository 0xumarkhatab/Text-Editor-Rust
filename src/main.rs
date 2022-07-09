extern crate colored;
extern crate term_cursor as cursor;
use colored::*;
#[warn(dead_code)]
use std::{thread,time};
use std::io::{stdin, self,Write, Read};
use std::string::FromUtf16Error;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Stylize;
use crossterm::{event, terminal};

use crate::reader::read_console_line;


mod reader;



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
        
            format!("{}", format!("\t\t\tR-Edit - Write Efficiently ").dark_green().bold()),
            format!("\n{} -> \t{} {} {} {}", 
                 format!("Menu").green() ,    
                 format!("{} => Create New File ",format!("Ctrl + N").cyan()).bold() ,
                 format!("{} => Save to File ",format!("Ctrl + S").cyan()).bold() ,
                 format!("{} => Load from File ",format!("Ctrl + L").cyan()).bold()  ,
                 format!("{} => Exit ",format!("Alt + Q").cyan()).bold() ),
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
    println!("{}\n", format!("{}", string).bold());

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
   
   let mut dots=String::from(".");
   let colors=["green","red","cyan","aqua","blue","orange","yellow","maroon"];
   let n=colors.len();
   for i in (0..9) {
    clrscr();
    println!("{}\n{}\r",format!("\t\t\tWelcome to {} \n\t\t\tThe Rust Based Text Editor",format!("R-Edit").color(colors[(i*2)%n])) ,
     format!("{}{}",format!("\t\t\t  Loadig Modules").color(colors[(i*3)%n]),format!("{}",dots).color(colors[(i*6)%n]).bold()));
    if i%2 !=0{
        dots.push('.');

    }
    
    
    the_delay(1);
    

   }


}

fn load_from_file(fname:&mut String)->String{
    let mut file  = std::fs::File::open(String::from("./")+fname).expect("file opening failed");
    let mut buffer=String::from("");
    let _res= file.read_to_string(&mut buffer);
    return buffer;

}

fn save_to_file(string:String,fname:&mut String)->bool{
    let mut file  = std::fs::File::create(String::from("./")+fname).expect("file creation failed");
    file.write_all(string.as_bytes()).expect("File Writing Failed");
    return true;

}


fn toggle_terminal_raw_mode(type_:u8){
    if(type_==1){
        terminal::enable_raw_mode();
    }
    else{
    terminal::disable_raw_mode();     
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
    
    toggle_terminal_raw_mode(1);


//  _________________________________    //

//      Opening the Text Editor Interface  ///  

let opening_note=String::from("Start from here ");
print_text_contents(&mut String::from("-> Press any Key to start Writing "), x_max, y_max, start_y);
event::read();
print_text_contents(&mut input, x_max, y_max, start_y);

//______________________________________ //

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
                    print_bottom("Exiting - Thank You for using R-Edit", current_y+4, 4);
                    break;
                }

                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    print_bottom("Interrupted - Thank You for using R-Edit", current_y+4, 4);

                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    
                    print_bottom("\n\n\t\t -- File Save Wizard-- \n\nDo You want to Save your Work ? ", y_max, 4);
                    toggle_terminal_raw_mode(0);

                    let mut valid_statement=false;
                    while valid_statement==false{
                        println!("Y -> Yes\nN ->No");
                        let mut user_save_option =reader::read_console_line() ;
                        let mut user_save_option=user_save_option.as_str();

    
                        match user_save_option.clone(){
                            "Y" | "y"=>{
                                println!("Saving the file....");
                                println!("\nEnter File name( eg data.txt ):= ");
                                let file_name = read_console_line();
                                save_to_file(input.clone(), &mut file_name.clone());

                                valid_statement=true;
                                println!("File saved Successfullly !");           

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

                    println!("Going back to edit mode !");           
                    the_delay(4);
                    toggle_terminal_raw_mode(1);
                    print_text_contents(&mut input, x_max, y_max, start_y);
                }
                // Loading From File
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    toggle_terminal_raw_mode(0);
                    print_bottom("\n\n\t\t --File Loading Wizard -- \n\nEnter File name ro Load from (e.g data.txt) ", y_max, 4);
                    let mut file_name = read_console_line();
                    let data = load_from_file(&mut file_name);
                    input=data;
                    println!("\nData Loaded Successfully !");

                    println!("\nGoing back to edit mode with File Data !");           
                    the_delay(4);
                    toggle_terminal_raw_mode(1);
                    print_text_contents(&mut input, x_max, y_max, start_y);
                    
                

                }
                KeyEvent {
                    code: KeyCode::Char('n'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    toggle_terminal_raw_mode(0);
                    print_bottom("\n\n\t\t -- File Creation Wizard -- \n\nWe would suggest  saving the work with CTRL+S before ?\nSelect Options ", y_max, 4);
                    toggle_terminal_raw_mode(0);

                    let mut valid_statement=false;
                        println!("Y -> Create New File \nN -> Cancel File Creation ");
                        let mut user_save_option =reader::read_console_line() ;
                        let mut user_save_option=user_save_option.as_str();
    
                        match user_save_option.clone(){
                            "Y" | "y"=>{
                                println!("\nEntering into new Workspace ...");
                                input=String::from("");
                                x_coord=0;
                                y_coord=start_y;
                                current_y=0;
                            }
                            "N" | "n"=>{
                                println!("File is not intended to be Created");
                                valid_statement=true;
                            }
                            
                            _=>{
                                println!("\n\n\t\tWhoopsInvalid Choice ....\nEnter Again to Save the Work\n")
                                
                            }
                        }
                            
                    println!("\nGoing back to Edit mode !");           
                    the_delay(4);
                    toggle_terminal_raw_mode(1);
                    print_text_contents(&mut input, x_max, y_max, start_y);
                    
                

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
