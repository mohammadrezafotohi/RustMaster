use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};
use std::ptr;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};
//use std::io::{self, Read};

fn main() {
    hide_console();
    let path = r"Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.bluelightreductionstate\windows.data.bluelightreduction.bluelightreductionstate";
    let p_dir = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let sub_key = p_dir.open_subkey_with_flags(
        path,
        KEY_READ);

    let (width, height) = (640, 480);
    let mut window = simple::Window::new("Dear NightLight", width, height);
    let mut image_on = window.load_image(include_bytes!("../assets/night_light_on.png")).unwrap();
    let mut image_off = window.load_image(include_bytes!("../assets/night_light_off.png")).unwrap();
    let mut clear= false;
    let mut state: i8 = 0;

    match sub_key {
        Ok(sub_key) => {
                while window.next_frame() {
                let night_light_status = sub_key.get_raw_value("Data");
                match night_light_status {
                    Ok(binary_data) => {
                        if binary_data.bytes.len() > 25 {
                        //println!("{:?} 23:{}. 24:{}", binary_data.bytes, binary_data.bytes[24], binary_data.bytes[25]);
                        if binary_data.bytes[23] == 16 && binary_data.bytes[24] == 0 {
                            print_on_window(&mut window, "NightLight is On!", (width/2) as i32, (height - 30) as i32, 70);
                            window.draw_image(&mut image_on, 0, 0);
                            if state == 1 {
                                state = 0;
                                clear = true;
                            }
                        }
                        else {
                            print_on_window(&mut window, "NightLight is Off!", (width/2) as i32, (height - 30) as i32, 70);
                            window.draw_image(&mut image_off, 0, 0);
                            if state == 0 {
                                state = 1;
                                clear = true;
                            }
                        }
                    }
                }
                    Err(_) => print_on_window(&mut window, "Failed to get regiser key!", (width - 10) as i32, (height/2) as i32, 50),
                    //println!("{}", e)
                }
                if clear {
                    window.clear();
                    clear = false;
                }
            }
        }
        Err(e) => println!("Failed to read register path! {} \n path: {}", e, path),
    }

    //let mut buffer = [0;1];
    //io::stdin().read_exact(&mut buffer).expect("!");
}

fn print_on_window(window: &mut simple::Window, massage: &str, posx: i32, posy: i32, posx_fix: i32) {
    window.print(&massage, posx - posx_fix, posy);
}

fn hide_console() {
    let window = unsafe {
        GetConsoleWindow()
    };

    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}