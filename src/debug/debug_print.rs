#[cfg(debug_assertions)]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($( $args:expr ),*) => {}
}