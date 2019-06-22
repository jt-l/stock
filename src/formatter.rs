extern crate termion;

use termion::{color, style};

//use std:io::{Write};
use crate::api;


pub fn print(responses: Vec<api::Response>) {
    
    for response in responses {

        // build output string
        let mut output = "".to_string();
        output.push_str(&response.symbol);
        output.push_str("    ");
        output.push_str(&response.price);
        output.push_str("   (");
        output.push_str(&response.change_percent);
        output.push_str(")");

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

        // print final output
        println!("{}", output);

        // reset styling
        print!("{}", style::Reset);
    }
}
