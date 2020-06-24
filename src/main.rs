use clap::{Arg, App};
use std::fs;

mod bf;
mod ir;
use bf::ExecutionContext;

fn main() -> std::io::Result<()> {
    let matches = App::new("Brainfuck Interpeter")
                    .arg(Arg::with_name("FILE")
                        .help("Brainfuck source file.")
                        .required(true)
                        .index(1))
                .get_matches();

    let src_fname = matches.value_of("FILE").unwrap();
    let src_file_contents = fs::read(src_fname)?;

    let mut ctx = ExecutionContext::new(src_file_contents);

    // TODO: add configs for stuff like debugging and single stepping?
    ctx.execute_all();

    Ok(())
}