use std::{thread, time::Duration, time::SystemTime};
use tfc::{Context, Error, traits::KeyboardContext, Key};

fn main() -> Result<(), Error> {
    let delay = Duration::from_millis(10);
    let mut ctx = Context::new()?;
    let timer = SystemTime::now();
    loop {
        thread::sleep(delay);
        ctx.key_click(Key::A)?;
        thread::sleep(delay);
        ctx.key_click(Key::B)?;
        thread::sleep(delay);
        ctx.key_click(Key::C)?;
        thread::sleep(delay);
        ctx.key_click(Key::D)?;
        thread::sleep(delay);
        ctx.key_click(Key::E)?;

        ctx.key_down(Key::Shift)?;
        thread::sleep(delay);
        ctx.key_click(Key::A)?;
        thread::sleep(delay);
        ctx.key_click(Key::B)?;
        thread::sleep(delay);
        ctx.key_click(Key::C)?;
        thread::sleep(delay);
        ctx.key_click(Key::D)?;
        thread::sleep(delay);
        ctx.key_click(Key::E)?;
        ctx.key_up(Key::Shift)?;

        ctx.key_click(Key::ReturnOrEnter)?;

        if timer.elapsed().unwrap().as_secs()>30 {
            break;
        }
    }

    Ok(())
}
