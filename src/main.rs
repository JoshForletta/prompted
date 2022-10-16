use std::fmt::Write;

use log::{error};

use prompted::Prompt;


fn main() {
    simple_logger::init();

    let mut prompt = Prompt::new();
    if let Err(e) = prompt.load_config("test_config.json") {
        error!("Failed loading config: {}", e);
    };

    let mut buf = String::with_capacity(100);

    buf.push_str("┎╼ ");

    if prompt["logname"].include() {
        write!(buf, "{}", prompt["logname"]);

        if prompt["hostname"].include() {
            buf.write_str("@");
        } else {
            buf.write_str(" ╾╼ ");
        }
    }

    if prompt["hostname"].include() {
        write!(buf, "{}", prompt["hostname"]);

        if prompt["path"].include() {
            buf.write_str(" ╾╼ ");
        }
    }

    if prompt["path"].include() {
        write!(buf, "{}", prompt["path"]);
    }

    if prompt["git"].include() {
        write!(buf, " ╾╼ {}", prompt["git"]);
    }

    buf.write_str("\n┖─╼ $ ");

    println!("{}", buf);
}
