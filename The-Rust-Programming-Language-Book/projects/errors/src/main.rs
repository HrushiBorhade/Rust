mod recoverable_errors;
mod unresolved_errors;

fn main() {
    unresolved_errors::demonstrate_unresolved_errors();
    recoverable_errors::demonstrate_recoverable_errors();
}
