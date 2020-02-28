use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

mod sand;
use sand::{World, Species};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("falling sand", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 800, 600)
        .map_err(|e| e.to_string())?;

    let mut world = World::new(100, 75);
    let mut paint_color = Species::Sand;

    let mut event = sdl_context.event_pump()?;
    'running: loop {
        for event in event.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if key == Keycode::Num1 {
                        paint_color = Species::Sand;
                    } else if key == Keycode::Num2 {
                        paint_color = Species::Wall;
                    }
                },
                Event::MouseMotion {x, y, mousestate, ..} => {
                    if mousestate.is_mouse_button_pressed(MouseButton::Left) {
                        world.paint((x / 8) as u32, (y / 8) as u32, paint_color);
                    }
                },
                _ => {}
            }
        }
        world.tick();
        texture.with_lock(None, |b: &mut [u8], p: usize| world.render(b, p))?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}