pub mod electrical;
pub mod flow;

fn main() {
    electrical::mppt::run();
    flow::example();
}
