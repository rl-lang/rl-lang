get client
get server
get println, read_int from std::io

/// Entry point for the demo: asks the user to pick a mode and dispatches
/// to `server()` (mode 1) or `connect()` (mode 2)
fn main() {
    println("choose mode:")
    println("\t1| server")
    println("\t2| client")
    match read_int("your choice: ") {
        1 => { serve() }
        2 => { connect() }
        _ => { println("invalid choice") }
    }
}
