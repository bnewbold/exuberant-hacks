
#[macro_use]
extern crate glium; 

extern crate image;
extern crate getopts;

use std::env;
use std::u64;
use std::process::exit;
use getopts::{Options, Matches};
use glium::glutin::os::unix::WindowBuilderExt;
    
pub mod util;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn generic_main(mut opts: Options) -> (Matches, Option<u64>) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Turn, eg, "-root" into "--root"
    let args = util::convert_xscreensaver_args(args);

    // Common Args (all screensavers)
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "window", "run in a window (IGNORED)");
    opts.optflag("", "root", "run in root window (IGNORED)");
    opts.optflag("", "fps", "show frames per second (IGNORED)");
    opts.optopt("", "window-id", "X window id number", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            println!("");
            println!("{}", f.to_string());
            exit(-1);
        }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        exit(0);
    }

    // if no "--window-id", try environment variable (arg has priority though)
    let window_id_string: Option<String> =
        matches.opt_str("window-id")
               .or(env::var("XSCREENSAVER_WINDOW").ok());

    let window_id = window_id_string.map(|id| match util::dechex2u64(&id) {
        Ok(y) => y,
        Err(e) => {
            println!("Couldn't parse numerical argument: {}", e);
            exit(-1); },
    });

    if window_id.is_some() {
        println!("Drawing on existing X window: 0x{:07X}", window_id.unwrap());
    }

    (matches, window_id)
}
