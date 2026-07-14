get println from std::io
get tcp_listen, tcp_accept, tcp_read, tcp_write, tcp_close from std::net
get is_err, result_unwrap, result_unwrap_err from std::res

/// Listens on 127.0.0.1:7878, accepts a single client connection, reads
/// one message, sends a canned reply, then closes the connection.
fn serve() {
    println("starting server on 127.0.0.1:7878...")

    dec result[int] listen_result = tcp_listen("127.0.0.1:7878")
    if (is_err(listen_result)) {
        println("could not start server:")
        println(result_unwrap_err(listen_result))
        return
    }

    dec int listener = result_unwrap(listen_result)
    println("listening! waiting for a connection...")

    dec int stream = result_unwrap(tcp_accept(listener))
    println("client connected!")

    dec string message = result_unwrap(tcp_read(stream, 1024))
    println("received:")
    println(message)

    result_unwrap(tcp_write(stream, "hello from the server!\n"))

    tcp_close(stream)
    tcp_close(listener)
    println("done, server closed")
}
