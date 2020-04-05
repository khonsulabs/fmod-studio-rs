use fmod_studio::system::{StudioInitialization, System};
use std::io::{stdin, Read};
extern crate pancurses;
use pancurses::{endwin, initscr, Input};

fn main() {
    let window = initscr();
    let system = System::initialized(StudioInitialization {
        spawn_update_thread: true,
        ..Default::default()
    })
    .unwrap();
    system
        .load_bank_from_file(
            "/Applications/FMOD Studio/examples/Build/Desktop/Master.bank",
            false,
        )
        .unwrap();
    system
        .load_bank_from_file(
            "/Applications/FMOD Studio/examples/Build/Desktop/Master.strings.bank",
            false,
        )
        .unwrap();
    let music = system
        .load_bank_from_file(
            "/Applications/FMOD Studio/examples/Build/Desktop/Music.bank",
            false,
        )
        .unwrap();
    let events = music.events().unwrap();
    let first = &events[2];
    let instance = first.create_instance().unwrap();
    instance.start().unwrap();

    loop {
        match window.getch() {
            Some(Input::Character(c)) => match c {
                'q' => break,
                _ => {}
            },
            Some(Input::KeyDC) => break,
            Some(_input) => {}
            None => {}
        }
    }

    // When using the spawned update therad, you should call system.shutdown to ensure resources are released
    system.shutdown();
    endwin();
}
