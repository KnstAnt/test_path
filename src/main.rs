use args::get_args;

mod error;
mod args;

fn main() {
    if let Ok(message) = get_args() {
        std::fs::write(message.path + &message.name,  "test").expect("Unable to write file");
    };
}
