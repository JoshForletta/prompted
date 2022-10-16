use std::fmt::Write;

use prompted::Prompt;

fn main() {
    let mut prompt = Prompt::new();
    prompt.load_config("test.json");

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
