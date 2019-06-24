extern crate termion;

use termion::{color, style};

use crate::api;


pub fn print(responses: Vec<api::Response>) {

    println!("----------------------------------");
    println!("SYMBOL    PRICE     PERCENT_CHANGE");
    println!("----------------------------------");
    
    for response in responses {

        // check if stock is negative
        let is_neg = response.change_percent.chars().next();

        // stock is down
        if let Some('-') = is_neg {
            // set color red
            print!("{}", color::Fg(color::Red));
        }

        // stock is up
        else {
            print!("{}", color::Fg(color::Green));
        }

        // print output
        print!("{:10}", &response.symbol);
        print!("{:10}", &response.price);
        println!("({})", &response.change_percent);       


        // reset styling
        print!("{}", style::Reset);

    }

    println!("----------------------------------");
}
