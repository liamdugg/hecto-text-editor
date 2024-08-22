use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {

    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        
        match b {
            
            // if no error occured with b
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03}\r",b);
                }

                else {
                    // {0:08b} --> toma arg cero(0:) y representa siempre con 8 bits(8) en binario(b) y rellena con ceros (0)
                    // {0:0#3} --> toma arg cero(0:) y representa con 3 digitos, rellena con ceros.
                    // {1:#?}  --> toma arg uno(1:) y muestra su "debug representation"
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}",b,c);
        
                }

                if c == 'q' {
                    break;
                }
            }

            // if b has an error (cant be unwrapped)
            Err(err) => println!("Error {}", err),
        }
    }

    disable_raw_mode().unwrap();

}
