pub mod electrical;
pub mod flow;

fn main() {
    electrical::mppt::run();
    
    let f = flow::Flow::new()
        .to(|flow| flow.source())
        .to(|flow| flow.source());
}
