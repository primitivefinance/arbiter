mod behaviors;

#[arbiter_macros::main(
    name = "minter",
    about = "A simple token minter simulation",
    behaviors = behaviors::Behaviors
)]
pub async fn main() {}
