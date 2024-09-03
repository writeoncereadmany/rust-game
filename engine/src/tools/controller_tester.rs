use sdl2::{controller::GameController, event::Event, keyboard::Keycode, EventPump};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let game_controller_subsystem = sdl_context.game_controller()?;

    let _window = video_subsystem.window("input-tester", 100, 100)
        .build()
        .expect("could not initialize video subsystem");

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    let mut _controller: Option<GameController> = Option::None;

    'outer: loop {
        'inner: for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion { .. } => break 'inner,
                Event::Window { .. } => break 'inner,
                Event::Quit {..} => break 'outer,
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'outer,
                Event::ControllerDeviceAdded{ which, .. } => { 
                    _controller = game_controller_subsystem.open(which).ok();
                },
                _ => {}
            }
            println!("Event fired: {:?}", event);
        }
    }

    Ok(())
}
