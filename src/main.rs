// Issues:
// Summary breaks when PE is not applicable (e.g. indexes)
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = broth::Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = broth::run(command) {
        eprintln!("Broth error: {e}");
        std::process::exit(1);
    };
}
