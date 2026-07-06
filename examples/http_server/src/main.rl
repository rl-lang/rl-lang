get println from std::io
get http_server_start, http_server_recv, http_request_method, http_request_url, http_respond from std::http
get result_unwrap, is_err, result_unwrap_err from std::res
get time_now, format_time_str, format_date_str from std::time
get split, index_of, slice, format from std::str
get len, arr_range from std::array

fn path_only(string url) -> string {
    dec int q = index_of(url, "?")
    if (q == -1) {
        return url
    }
    return slice(url, 0, q)
}

fn query_param(string url, string key) -> string {
    dec int q = index_of(url, "?")
    if (q == -1) {
        return ""
    }
    dec string query = slice(url, q + 1, len(url))
    dec arr[string] pairs = split(query, "&")
    dec arr[int] range = arr_range(0, pairs.len(), 1)

    for i in range {
        dec string pair = pairs[i]
        dec int eq = index_of(pair, "=")
        if (eq == -1) {
            continue
        }
        if (slice(pair, 0, eq) == key) {
            return slice(pair, eq + 1, len(pair))
        }
    }
    return ""
}

fn main() {
    dec result[int] start_result = http_server_start("127.0.0.1:8080")
    if (is_err(start_result)) {
        println("could not start server:")
        println(result_unwrap_err(start_result))
        return
    }

    dec int server = result_unwrap(start_result)
    dec int hits = 0
    println("serving on http://127.0.0.1:8080  (Ctrl+C to stop)")

    while (true) {
        dec int req = result_unwrap(http_server_recv(server))
        dec string method = result_unwrap(http_request_method(req))
        dec string url = result_unwrap(http_request_url(req))
        dec string path = path_only(url)

        hits = hits + 1
        println(format("{} {} (hit #{})", method, path, hits))

        if (path == "/") {
            dec string body = format(
                "welcome to my rl-lang server\ndate: {}\ntime: {}\nhits so far: {}\n",
                result_unwrap(format_date_str(time_now())),
                result_unwrap(format_time_str(time_now())),
                hits
            )
            result_unwrap(http_respond(req, 200, body, "text/plain"))
        } else if (path == "/time") {
            dec string body = format("{}\n", result_unwrap(format_time_str(time_now())))
            result_unwrap(http_respond(req, 200, body, "text/plain"))
        } else if (path == "/hits") {
            dec string body = format("{}\n", hits)
            result_unwrap(http_respond(req, 200, body, "text/plain"))
        } else if (path == "/echo") {
            dec string msg = query_param(url, "msg")
            if (msg == "") {
                result_unwrap(http_respond(req, 400, "missing ?msg= query param\n", "text/plain"))
            } else {
                result_unwrap(http_respond(req, 200, format("{}\n", msg), "text/plain"))
            }
        } else {
            result_unwrap(http_respond(req, 404, "not found\n", "text/plain"))
        }
    }
}
