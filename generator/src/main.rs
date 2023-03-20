use clap::Parser;
use generator::get_instructions;

// important RAM areas
// 0x0000 - 0x03FF ASCII table
// 0x0400 - 0x0400 pen position
// 0x0401 - 0x0401 keycode register
// 0x0500 - 0xFEFD user code + memory
// 0xFEFE - 0xFEFF used to jump back to user code
// 0xFF00 - 0xFFFF temporary variables

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("{}", get_instructions(&args.program).unwrap())
}
