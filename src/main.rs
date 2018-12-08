extern crate crossterm;
extern crate clap;
use clap::{Arg, App};
// use std::io;

use crossterm::style::{Color, style};

fn main() {
    let matches = App::new("My Super Program")
                        .version("1.0")
                        .author("Kevin K. <kbknapp@gmail.com>")
                        .about("Does awesome things")
                        .arg(Arg::with_name("COLUMNS")
                            .required(true)
                            .index(1))
                        .get_matches();

    let columns = matches
        .value_of("COLUMNS")
        .unwrap()
        .parse::<i32>()
        .unwrap_or(0);

    let two_line = "
   ###     ######   ######  ########  ######   ######           
  ## ##   ##    ## ##    ## ##       ##    ## ##    ##          
 ##   ##  ##       ##       ##       ##       ##                
##     ## ##       ##       ######    ######   ######           
######### ##       ##       ##             ##       ##          
##     ## ##    ## ##    ## ##       ##    ## ##    ##          
##     ##  ######   ######  ########  ######   ######           

 ######   ########     ###    ##    ## ######## ######## ######## 
##    ##  ##     ##   ## ##   ###   ##    ##    ##       ##     ## 
##        ##     ##  ##   ##  ####  ##    ##    ##       ##     ##
##   #### ########  ##     ## ## ## ##    ##    ######   ##     ##
##    ##  ##   ##   ######### ##  ####    ##    ##       ##     ##
##    ##  ##    ##  ##     ## ##   ###    ##    ##       ##     ##
 ######   ##     ## ##     ## ##    ##    ##    ######## ########
 ";

    let one_line = "
   ###     ######   ######  ########  ######   ######            ######   ########     ###    ##    ## ######## ######## ######## 
  ## ##   ##    ## ##    ## ##       ##    ## ##    ##          ##    ##  ##     ##   ## ##   ###   ##    ##    ##       ##     ##
 ##   ##  ##       ##       ##       ##       ##                ##        ##     ##  ##   ##  ####  ##    ##    ##       ##     ##
##     ## ##       ##       ######    ######   ######           ##   #### ########  ##     ## ## ## ##    ##    ######   ##     ##
######### ##       ##       ##             ##       ##          ##    ##  ##   ##   ######### ##  ####    ##    ##       ##     ##
##     ## ##    ## ##    ## ##       ##    ## ##    ##          ##    ##  ##    ##  ##     ## ##   ###    ##    ##       ##     ##
##     ##  ######   ######  ########  ######   ######            ######   ##     ## ##     ## ##    ##    ##    ######## ########
";

    let will_print;
    if columns < 66 {
        will_print = ""
    } else if columns > 130 {
        will_print = one_line;
    } else {
        will_print = two_line;
    }


    let styled = style(will_print)
        .with(Color::Green)
        .slow_blink()
        .bold();


    print!("{}", styled);



}