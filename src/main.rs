mod five;
mod four;
mod one;
mod two;

mod intcomp;

fn main() {
    one::run_a();
    one::run_b();
    two::run_a();
    two::run_b();

    four::run_a();
    four::run_b();
    five::run_a();
    five::run_b();
}
