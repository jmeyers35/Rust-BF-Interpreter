use clap::{Arg, App};
use std::fs;

mod defs;

fn main() -> std::io::Result<()> {
    let matches = App::new("Brainfuck Interpeter")
                    .arg(Arg::with_name("INPUT")
                        .help("Brainfuck source file.")
                        .required(true)
                        .index(1))
                .get_matches();

    let src_fname = matches.value_of("INPUT").unwrap();
    let src_file_contents = fs::read(src_fname)?;

    let mut ctx = defs::ExecutionContext::new(src_file_contents);

    // TODO: add configs for stuff like debugging and single stepping?

    ctx.execute_all();

    Ok(())
}
