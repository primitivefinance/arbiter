use behaviors::Behaviors;

#[arbiter_macros::main(
    name = "ExampleArbiterProject",
    about = "Our example to get you started."
    behaviors = Behaviors
)]
pub async fn main() {}
