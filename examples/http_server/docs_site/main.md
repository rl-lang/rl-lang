# main.rl

## `fn` path_only (line 10)

```rl
fn path_only(string url) -> string
```

Strips the query string from a URL, returning just the path portion.
Return the URL unchanged if it has no `?`.

## `fn` query_param (line 20)

```rl
fn query_param(string url, string key) -> string
```

Looks up a single query parameter's value for URL by key.
Return `""` if the URL has no query string or the key isn't present.

## `fn` main (line 45)

```rl
fn main()
```

Starts the HTTp server on 127.0.0.1:8000 and serves requests forver,
routing on the request path: `/`, `/time`, `/hits`, `/echo`, and a
catch-all 404 for everything else.

