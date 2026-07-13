get client
get server
get println, read_int from std::io

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
