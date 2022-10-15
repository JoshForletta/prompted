use std::{
    fmt::Write,
};

use prompted::Prompt;

fn main() {
    let prompt = Prompt::new();
    prompt.load_config().unwrap();

    let mut buf = String::with_capacity(100);

    buf.write_str("┎╼ ");

    if prompt["logname"].include() {
        buf.write_fmt(format_args!("{} ╾╼", prompt["logname"]));
    }
}
