mod print;
mod strings;
mod types;
mod vars;

fn main() {
    let name = "Julian";
    println!("{}", name);

    print::run();

    strings::run();

    types::run();

    vars::run();
}
