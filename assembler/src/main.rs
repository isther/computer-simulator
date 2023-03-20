mod assembler;

use assembler::Assembler;
use clap::Parser;
use std::{fs::File, io::prelude::Write, path::Path};

pub const USER_CODE_START: u16 = 0x0500;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "program")]
    program_name: String,

    #[arg(short = 'o', long = "output")]
    output_file_path: String,

    #[arg(short = 'r', long, default_value_t = true)]
    render: bool,
}

fn main() {
    let args: Args = Args::parse();

    match args.render {
        false => File::create(Path::new(&args.output_file_path))
            .unwrap()
            .write_all(to_u8_slice(
                &mut Assembler::new()
                    .process(
                        USER_CODE_START,
                        generator::get_instructions(&args.program_name),
                    )
                    .unwrap(),
            ))
            .unwrap(),
        true => println!(
            "{}",
            Assembler::new()
                .string(
                    USER_CODE_START,
                    generator::get_instructions(&args.program_name),
                )
                .unwrap()
        ),
    }
}

fn to_u8_slice(slice: &mut [u16]) -> &mut [u8] {
    let byte_len = 2 * slice.len();
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast::<u8>(), byte_len) }
}
