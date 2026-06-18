use std::{thread, time::Duration, time::SystemTime};
use tfc::{Context, Error, traits::KeyboardContext, Key};

fn main() -> Result<(), Error> {
    let delay = Duration::from_millis(10);
    let mut ctx = Context::new()?;
    let timer = SystemTime::now();
    loop {
        thread::sleep(delay);
        ctx.key_click(Key::A, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::B, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::C, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::D, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::E, None)?;

        ctx.key_down(Key::Shift)?;
        thread::sleep(delay);
        ctx.key_click(Key::A, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::B, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::C, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::D, None)?;
        thread::sleep(delay);
        ctx.key_click(Key::E, None)?;
        ctx.key_up(Key::Shift)?;

        ctx.key_click(Key::ReturnOrEnter, None)?;

        if timer.elapsed().unwrap().as_secs()>30 {
            break;
        }
    }

    Ok(())
}
