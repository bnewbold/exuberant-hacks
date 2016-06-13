
use std::str::FromStr;
use std::num::ParseIntError;

pub fn view_matrix(position: &[f32; 3],
               direction: &[f32; 3],
               up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };  
    
    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };
    
    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

/// This little hacky function converts X-style args like "-root" to standard
/// UNIX long-style args ("--root") that can be parsed by getopts.
///
/// These variant long-form args should probably be hidden from the user, eg
/// not show up in usage or manpage output.
pub fn convert_xscreensaver_args(raw: Vec<String>) -> Vec<String> {

    let known_args = vec!["-root",
                          "-window",
                          "-mono",
                          "-install",
                          "-noinstall",
                          "-visual",
                          "-window-id",
                          "-fps",
                          "-no-fps",
                          "-pair",
                          "-record-animation"];

    let ret: Vec<String> = raw.into_iter().map(|arg| {
        if known_args.contains(&(arg.as_str())) {
            String::from_str("-").unwrap() + &arg
        } else {
            arg
        }
    }).collect();
    ret
}

#[test]
fn test_xargconv() {
    assert_eq!(vec!["--root"], convert_xscreensaver_args(vec!["-root".to_string()]));
    assert_eq!(vec!["--root"], convert_xscreensaver_args(vec!["--root".to_string()]));
    assert_eq!(vec!["root"], convert_xscreensaver_args(vec!["root".to_string()]));
    assert_eq!(vec!["-asdf"], convert_xscreensaver_args(vec!["-asdf".to_string()]));
    assert_eq!(vec!["-h"], convert_xscreensaver_args(vec!["-h".to_string()]));
}

/// Converts a string which is decimal ("1234") or hex-with-prefix ("0x12BC") to u64.
pub fn dechex2u64(raw: &str) -> Result<u64, ParseIntError> {

    if (raw).starts_with("0x") {
        u64::from_str_radix(raw.trim_left_matches('0').trim_left_matches('x'), 16)
    } else {
        raw.parse::<u64>()
    }
}

#[test]
fn test_dechex2u64() {
    assert_eq!(Ok(123), dechex2u64("123"));
    assert_eq!(Ok(0), dechex2u64("000"));
    assert_eq!(Ok(65535), dechex2u64("0xFFFF"));
    assert_eq!(Ok(291), dechex2u64("0x123"));
    assert!(dechex2u64("0x").is_err());
    assert!(dechex2u64("").is_err());
    assert!(dechex2u64("FF123").is_err());
    assert!(dechex2u64("0FF123").is_err());
    assert!(dechex2u64("asdf").is_err());
}
