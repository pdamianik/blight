use blight::{
    Change::{Regular,Sweep},
    Direction::{Dec,Inc},
};
use colored::Colorize;
use std::env::{self,Args};

fn main() {
    if blight::is_running() {
        return;
    }
    argument_parser(env::args());
}

fn argument_parser(mut args: Args) {
    if let Some(arg) = args.next().and_then(|_| args.next()) {
        match &arg[..] {
            "help" => blight::print_help(),
            "status" => blight::print_status(args.next()),
            "list" => blight::print_devices(),
            "save" => blight::save(args.next()),
            "restore" => blight::restore(),
            "set" => {
                if let Some(v) = args.next() { blight::set_bl(&v, args.next()) }
            },
            "inc" => {
                if let Some(v) = args.next() {
                    blight::change_bl(&v, Regular, Inc, args.next())
                } else {
                    blight::change_bl("2", Regular, Inc, args.next())
                }
            },
            "dec" => {
                if let Some(v) = args.next() {
                    blight::change_bl(&v, Regular, Dec, args.next())
                } else {
                    blight::change_bl("2", Regular, Dec, args.next())
                }
            },
            "sweep-up" => {
                if let Some(v) = args.next() {
                    blight::change_bl(&v, Sweep, Inc, args.next())
                } else {
                    blight::change_bl("10", Sweep, Inc, args.next())
                }
            },
            "sweep-down" => {
                if let Some(v) = args.next() {
                    blight::change_bl(&v, Sweep, Dec, args.next())
                } else {
                    blight::change_bl("10", Sweep, Dec, args.next())
                }
            },
            _ => println!("{}\n{}",
                          "Oops... You appear to have entered an unrecognised command ┐(´•_•`)┌".bold(),
                          "Tip: Try `blight help` to see all supported commands".yellow(),
            )
        }
    } else {
        blight::print_welcome();
    }
}
