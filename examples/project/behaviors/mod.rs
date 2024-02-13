use arbiter_macros::Behaviors;

#[derive(Behaviors)]
pub enum Behaviors {
    MyBehavior(MyBehavior),
    MyBehavior2(MyBehavior2),
}
