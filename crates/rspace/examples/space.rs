/*
    Appellation: space <example>
    Created At: 2025.12.26:18:23:32
    Contrib: @FL03
*/
use rspace::{Container, RawSpace};

fn main() -> rspace::Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(false)
        .with_line_number(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    tracing::info! { "Welcome to {p}!", p = env!("CARGO_PKG_NAME") }

    let container = Something([1, 2, 3, 4, 5]);
    tracing::info! { ?container }

    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Something<T: ?Sized>(pub T);

impl<T> RawSpace for Something<T> {
    type Elem = T;
}

impl<T> Container<T> for Something<[T]>
where
    [T]: RawSpace<Elem = T>,
{
    type Cont<V> = [V];
}
