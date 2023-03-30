use computer_simulator::{glfw_run, Computer, Keyboard, PrintStateConfig};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Notify},
    time::{interval, Duration},
};

#[tokio::main]
async fn main() {
    let (key_press_sender, key_press_receiver) = mpsc::channel(1);
    let (screen_sender, screen_receiver) = mpsc::channel(1);
    let quit = Arc::new(Notify::new());
    let mut computer = Computer::new(screen_sender, quit.clone());
    let mut key_board = Keyboard::new(key_press_receiver, quit.clone());

    computer.connect_keyboard(&mut key_board);

    //TODO: read bin
    // computer.load_to_ram(0x0500, bin);

    tokio::spawn(async move {
        key_board.run().await;
    });

    tokio::spawn(async move {
        computer
            .run(
                computer.screen_control.clone(),
                interval(Duration::from_nanos(1)),
                //TODO: make this configurable
                PrintStateConfig {
                    print_state: false,
                    print_state_every: 0,
                },
            )
            .await;
    });

    glfw_run(screen_receiver, key_press_sender, quit.clone());
}
