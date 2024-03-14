use legion::Schedule;
use pas_cman::{main_loop, render_map_system, BResult, BTermBuilder, State};

fn main() -> BResult<()> {
    let mut state = State::new();
    let (w, h) = state.load_file("resources/map.txt").expect("could not load map");
    
    let context = BTermBuilder::new()
        .with_title("pas cman")
        .with_dimensions(w, h)
        .with_fps_cap(30.0)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
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
