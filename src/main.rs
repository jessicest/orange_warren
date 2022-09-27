use druid::PlatformError;

mod fragment;
mod squares;
mod window;
mod world;
mod item;

type Zone = squares::Zone<squares::Square<i64>>;

fn main() -> Result<(), PlatformError> {
    println!("Hello, world!");
    window::do_a_window()
}
