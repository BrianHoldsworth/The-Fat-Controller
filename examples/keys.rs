use std::{thread, time::Duration, time::SystemTime};
use tfc::{Context, Error, traits::KeyboardContext, Key};

fn main() -> Result<(), Error> {
    let delay = Duration::from_millis(10);
    let mut ctx = Context::new()?;
    let timer = SystemTime::now();
    loop {
        thread::sleep(delay);
        ctx.key_click(Key::A).ok();
        thread::sleep(delay);
        ctx.key_click(Key::B).ok();
        thread::sleep(delay);
        ctx.key_click(Key::C).ok();
        thread::sleep(delay);
        ctx.key_click(Key::D).ok();
        thread::sleep(delay);
        ctx.key_click(Key::E).ok();

        ctx.key_down(Key::Shift).ok();
        thread::sleep(delay);
        ctx.key_click(Key::A).ok();
        thread::sleep(delay);
        ctx.key_click(Key::B).ok();
        thread::sleep(delay);
        ctx.key_click(Key::C).ok();
        thread::sleep(delay);
        ctx.key_click(Key::D).ok();
        thread::sleep(delay);
        ctx.key_click(Key::E).ok();
        ctx.key_up(Key::Shift).ok();

        ctx.key_click(Key::ReturnOrEnter).ok();

        if timer.elapsed().unwrap().as_secs()>30 {
            break;
        }
    }

    Ok(())
}
