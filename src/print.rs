use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Print Function with coloring and emojis
pub fn print(msg: &str) {
    // TODO : Parse coloring + emoji
    println!("{}", msg);
}

/// Colorize Struct Components
pub fn formatStruct(s: &str) -> String {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut ret = String::new();
    let parts: Vec<&str> = s.split_whitespace().collect();
    let mut indent = 0;
    println!("ALL PARTS : {:?}", parts);
    for i in 0..parts.len() {
        stdout.set_color(&ColorSpec::new());
        let s = parts[i];
        if s.starts_with("{") {
            indent += 1;
            write!(&mut stdout, " ");
        }
        if s.starts_with("[") {
            let p: Vec<&str> = s.split("[").collect();
            write!(&mut stdout, "[\n");
            indent += 1;
            for i in 0..indent {
                write!(&mut stdout, "\t");
            }
            write!(&mut stdout, "{}", p.get(1).unwrap());
            continue;
        }
        if s.starts_with("}") {
            indent -= 1;
            write!(&mut stdout, "\n");
            for i in 0..indent {
                write!(&mut stdout, "\t");
            }
            let c: Vec<&str> = s.split("}").collect();
            if c.get(1).is_some() {
                let ch = *c.get(1).unwrap();
                if ch == "," {
                    write!(&mut stdout, "}},\n");
                }
                if ch == "]" {
                    write!(&mut stdout, "}}\n");
                    indent -= 1;
                    for i in 0..indent {
                        write!(&mut stdout, "\t");
                    }
                    write!(&mut stdout, "]\n");
                }
                if ch == "" {
                    write!(&mut stdout, "}}\n");
                }
            } else {
                write!(&mut stdout, "}}\n");
            }
            for i in 0..indent {
                write!(&mut stdout, "\t");
            }
            continue;
        }
        let mut is_on_string = false;
        for c in s.chars() {
            stdout.set_color(&ColorSpec::new());
            if c == '"' {
                stdout.set_color(&ColorSpec::new().set_fg(Some(Color::Green)));
                if !is_on_string {
                    is_on_string = true;
                } else {
                    is_on_string = false;
                }
            }
            if is_on_string {
                stdout.set_color(&ColorSpec::new().set_fg(Some(Color::Green)));
            }
            let cint = c as isize;
            if cint <= 57 && cint >= 48 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
                write!(&mut stdout, "{}", &String::from(c));
                continue;
            }
            write!(&mut stdout, "{}", &String::from(c));
        }
        if s.starts_with("{") {
            write!(&mut stdout, "\n");
            for i in 0..indent {
                write!(&mut stdout, "\t");
            }
        }
        if s.ends_with(",") {
            write!(&mut stdout, "\n");
            for i in 0..indent {
                write!(&mut stdout, "\t");
            }
        }
        if s.ends_with(":") {
            write!(&mut stdout, " ");
        }
    }
    write!(&mut stdout, "\n");
    return ret;
}

/// Print Structs with highlighting
pub fn print_struct<T>(obj: &T)
where
    T: std::fmt::Debug,
{
    formatStruct(&format!("{:?}", obj));
}

// TODO : Logging
// TODO : Tables
// TODO : Progress Bars
// TODO : Status
// TODO : Tree?
// TODO : Tracebacks, Errors
