
extern crate glium; 
extern crate image;
extern crate getopts;
extern crate time;

use std::env;
use std::process::exit;
use getopts::{Options, Matches};
use glium::glutin::os::unix::WindowBuilderExt;
use glium::{DisplayBuild, Display};
    
pub mod util;

pub trait ExuberantHack {
    fn draw_frame(&mut self, t: f64) -> Result<(), String>;
    fn get_display(&self) -> &Display;
}

/// Prints usage/help info for an individual hack
pub fn print_usage(opts: &Options) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Executes a hack
pub fn run(hack: &mut ExuberantHack, conf: &Matches) {

    loop {
        hack.draw_frame(time::precise_time_s()).ok();
        for ev in hack.get_display().poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        // XXX: sleep here for 10ms
    }
}

/// Adds and parses most command-line options.
/// This function handles all the generic stuff, like --window-id and --help.
pub fn main_helper(mut opts: Options) -> Matches {
    let args: Vec<String> = env::args().collect();

    // Turn, eg, "-root" into "--root"
    let args = util::convert_xscreensaver_args(args);

    // Common Args (all screensavers)
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "window", "run in a window (IGNORED)");
    opts.optflag("", "root", "run in root window (IGNORED)");
    opts.optopt("", "window-id", "X window id number", "NUM");
    opts.optflag("", "fps", "show frames per second (IGNORED)");
    opts.optopt("", "delay", "inter-frame delay (0 to 100000) (IGNORED)", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&opts);
            println!("");
            println!("{}", f.to_string());
            exit(-1);
        }
    };

    if matches.opt_present("help") {
        print_usage(&opts);
        exit(0);
    }

    matches
}

pub fn make_display(conf: &Matches) -> glium::Display {

    // if no "--window-id", try environment variable (arg has priority though)
    let window_id_string: Option<String> =
        conf.opt_str("window-id")
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

    let win_builder: glium::glutin::WindowBuilder = match window_id {
        Some(id) =>
            glium::glutin::WindowBuilder::new()
                                         .from_existing_window(id),
        None => glium::glutin::WindowBuilder::new()
                                             .with_title(format!("Exuberant Hack!"))
                                             .with_depth_buffer(24),
    };
    let display = win_builder.build_glium().unwrap();
    display
}
