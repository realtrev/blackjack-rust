use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

pub struct Choice<T> {
    pub label: String,
    pub value: T,
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn wait_for_keypress() {
    wait_for_no_input();

    while !event::poll(Duration::from_millis(10)).unwrap() {
        // do nothing
    }
}

pub fn wait_for_no_input() {
    std::thread::sleep(Duration::from_millis(30));
    while event::poll(Duration::from_millis(10)).unwrap() {
        event::read().unwrap();
    }
}

pub fn select<T: Copy>(opts: Vec<Choice<T>>) -> T {
    let mut selected: usize = 0;

    fn print<T>(opts: &Vec<Choice<T>>, selected: usize) {
        for (i, opt) in opts.iter().enumerate() {
            if i == selected {
                print!("> ");
            } else {
                print!("  ");
            }
            println!("{}", opt.label);
        }
    }
    
    wait_for_no_input();
    print(&opts, selected);
    std::thread::sleep(std::time::Duration::from_millis(200));

    loop {
        wait_for_no_input();
        stdout().flush().unwrap();

        // loop through keydown events and handle them
        match event::read().unwrap() {
            Event::Key(KeyEvent { code, modifiers: _, .. }) => {
                match code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Up => {
                        if selected != 0 {
                            selected -= 1;
                        }
                        clear_lines(opts.len() as u16);
                        print(&opts, selected);
                    }
                    KeyCode::Down => {
                        if selected < opts.len() - 1 {
                            selected += 1;
                        }
                        clear_lines(opts.len() as u16);
                        print(&opts, selected);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Char(' ') => {
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        wait_for_no_input();
        std::thread::sleep(Duration::from_millis(50));
    }

    opts[selected].value
}

pub fn clear_lines(n: u16) {
    print!("\x1B[{}A\x1B[2K", n);
}

pub fn read_money() -> i32 {
    let mut input = String::new();
    loop {
        input.clear();
        println!("                                                                                                    ");
        clear_lines(1);
        print!("> $");
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                clear_lines(2);
                println!("Enter your bet: Invalid input");
            },
        }
    }
}