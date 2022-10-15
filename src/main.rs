use prompted::Prompt;

fn main() {
    let mut prompt = Prompt::new();
    prompt.load_config("test.json").unwrap();

    let mut buf = String::with_capacity(100);

    buf.push_str("┎╼ ");

    if prompt["logname"].include() {
        buf.push_str(format!("{} ╾╼", prompt["logname"]).as_str());
    }

    println!("{}", buf);
}
