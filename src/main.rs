
use druid::PlatformError;

mod squares;
mod window;

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    window::do_a_window()
}
