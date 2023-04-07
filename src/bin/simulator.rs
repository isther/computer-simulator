use clap::Parser;
use computer_simulator::{
    get_instructions, glfw_run, Assembler, Computer, Keyboard, PrintStateConfig, USER_CODE_START,
};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Notify},
    time::{interval, Duration},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "program")]
    program_name: String,

    #[arg(short = 's', long, default_value_t = true)]
    print_state: bool,

    #[arg(long, default_value_t = 7)]
    print_state_every: u16,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let (key_press_sender, key_press_receiver) = mpsc::channel(1);
    let (screen_sender, screen_receiver) = mpsc::channel(1);
    let quit = Arc::new(Notify::new());
    let mut computer = Computer::new(screen_sender, quit.clone());
    let mut key_board = Keyboard::new(key_press_receiver, quit.clone());

    computer.connect_keyboard(&mut key_board);

    let bin = Assembler::new()
        .process(
            USER_CODE_START,
            get_instructions(&args.program_name.to_string()),
        )
        .unwrap();

    // Load bin
    computer.load_to_ram(0x0500, bin);

    tokio::spawn(async move {
        key_board.run().await;
    });

    tokio::spawn(async move {
        computer
            .run(
                computer.screen_control.clone(),
                interval(Duration::from_nanos(1000)),
                PrintStateConfig {
                    print_state: args.print_state,
                    print_state_every: args.print_state_every,
                },
            )
            .await;
    });

    glfw_run(screen_receiver, key_press_sender, quit.clone());
    //BUG: Can not run
}
