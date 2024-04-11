mod behaviors;

#[arbiter_macros::main(
    name = "minter",
    about = "A simple token minter simulation",
    behaviors = behaviors::Behaviors<_>
)]
pub async fn main() {}
