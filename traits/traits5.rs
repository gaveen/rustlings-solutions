// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// Alternative 1 - trait parameters:
// fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
// Alternative 2 - generic parameters with trait bounds:
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
// Alternative 3 - also generic parameters with trait bounds:
fn some_func<T>(item: T) -> bool
    where T: SomeTrait + OtherTrait
{
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
