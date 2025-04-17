use std::env;
use std::str::FromStr;
use std::{io::{stdin, Read}, thread};

use legion::Schedule;
use pas_cman_ipl::{main_loop, render_map_system, BResult, BTermBuilder, State};

fn main() -> BResult<()> {
    let w = 30;
    let h = 20;

    let resources = env::var("PAS_RESOURCES").unwrap_or(String::from_str("resources/").unwrap());
    let (sx, rx) = std::sync::mpsc::channel();
    let mut state = State::new(rx);
    
    thread::spawn(move || {
        let mut buffer = [0_u8; std::mem::size_of::<pas_cman_ipl::pascman_protocol::Message>()];
        while stdin().read_exact(&mut buffer).is_ok() {
            unsafe {
                let message = buffer.as_mut_ptr() as *mut pas_cman_ipl::pascman_protocol::Message;
                sx.send(*message).expect("error sending message on the channel");
            }
        }
    });

    let context = BTermBuilder::new()
        .with_title("pas cman")
        .with_dimensions(w, h)
        //.with_fps_cap(30.0)
        .with_tile_dimensions(32, 32)
        .with_resource_path(resources)
        .with_font("pas-cman-font-32.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(w, h,"pas-cman-font-32.png")
        .with_simple_console_no_bg(w, h,"pas-cman-font-32.png")
        .with_simple_console_no_bg(w, h,"pas-cman-font-32.png")
        .with_simple_console(w*2, h*2, "terminal8x8.png")
        .build()?;

    // initialization systems
    Schedule::builder()
        .add_system(render_map_system())
        .build()
        .execute(&mut state.ecs, &mut state.resources)
        ;

    main_loop(context, state)?;

    Ok(())
}
