use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

mod sand;
use sand::{World, Species};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CELL_SIZE: u32 = 8;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("falling sand", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WINDOW_WIDTH, WINDOW_HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut world = World::new(WINDOW_WIDTH/CELL_SIZE, WINDOW_HEIGHT/CELL_SIZE, CELL_SIZE);
    let mut drawing: (bool, u32, u32, Species) = (false, 0, 0, Species::Sand);

    let mut event = sdl_context.event_pump()?;
    'running: loop {
        for event in event.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if key == Keycode::Num1 {
                        drawing.3 = Species::Sand;
                    } else if key == Keycode::Num2 {
                        drawing.3 = Species::Wall;
                    }
                },
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, .. } => 
                    {drawing.0 = true;},
                Event::MouseButtonUp {mouse_btn: MouseButton::Left, .. } => 
                    {drawing.0 = false;},
                Event::MouseMotion {x, y, ..} => {
                    drawing.1 = x as u32;
                    drawing.2 = y as u32;
                },
                _ => {}
            }
        }
        if drawing.0
            { world.paint(drawing.1/CELL_SIZE,drawing.2/CELL_SIZE, drawing.3); }
        // tick the world
        world.tick();
        // render the world and then display it
        texture.with_lock(None, |b: &mut [u8], p: usize| world.render(b, p))?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        // shitty sleep to get 60fps
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}