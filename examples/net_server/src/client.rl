get println from std::io
get tcp_connect, tcp_write, tcp_read, tcp_close from std::net
get result_unwrap from std::res

/// Connects to a server on 127.0.0.1:7878, sends a greeting, prints the
/// server's reply, then closes the connection.
fn connect() {
    println("connecting to 127.0.0.1:7878...")

    dec int stream = result_unwrap(tcp_connect("127.0.0.1:7878"))
    println("connected!")

    result_unwrap(tcp_write(stream, "hi server, this is the client\n"))

    dec string reply = result_unwrap(tcp_read(stream, 1024))
    println("server replied:")
    println(reply)

    tcp_close(stream)
    println("done, client closed")
}
