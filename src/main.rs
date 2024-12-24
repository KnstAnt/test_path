use args::get_args;

mod args;
mod error;

fn main() {
    let mut reply = "".to_owned();
    match get_args() {
        Ok(message) => {
            reply = match std::fs::write((message.path + "/" + &message.name).replace("//", "/"), "test") {
                Ok(_) => r#"{"status":"ok","message":null}"#.to_owned(),
                Err(e) => {
                    let str1 = r#"{"status":"failed","message":""#;
                    let str2 = r#""}"#;
                    format!("{str1}{}{str2}", e)
                },
            }
        },
        Err(e) => {
            let str1 = r#"{"status":"failed","message":""#;
            let str2 = r#""}"#;
            reply = format!("{str1}{}{str2}", e);
        },
    }
    println!("reply: {reply}");
    let _ = std::io::Write::write_all(&mut std::io::stdout().lock(), reply.as_bytes());
}
