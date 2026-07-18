get split, trim, is_empty, join, concat from std::str
get arr_push, arr_map, arr_filter, arr_find, arr_reduce, len from std::array
get read_file, write_file from std::io
get path_exists from std::path
get to_int, to_string from std::types

CONST string DELIMITER = ";"
CONST string HEADER = "id;status;created_at;text"
CONST int COL_ID = 0
CONST int COL_STATUS = 1
CONST int COL_CREATED_AT = 2
CONST int COL_TEXT = 3

fn csv_parse_row(string line) -> arr[string] {
    return arr_map(split(line, DELIMITER), fn (string f) -> string {
        return trim(f)
    })?
}

fn csv_parse(string raw) -> arr[arr[string]] {
    dec arr[string] lines = split(raw, "\n")?
    dec arr[arr[string]] rows = []
    dec bool header = true

    for line in lines {
        if (is_empty(trim(line))) {
            continue
        }
        if (header) {
            header = false
            continue
        }

        rows = arr_push(rows, csv_parse_row(line))?
    }

    return rows
}

fn csv_serialize_row(arr[string] row) -> string {
    return join(row, DELIMITER)?
}

fn csv_serialize(arr[arr[string]] rows) -> string {
    dec arr[string] lines = arr_map(rows, fn (arr[string] r) -> string {
        return csv_serialize_row(r)
    })?
    return concat(HEADER, "\n", join(lines, "\n"))
}

fn csv_load(string path) -> arr[arr[string]] {
    if (!path_exists(path)) {
        return []
    }
    return csv_parse(read_file(path)?)
}

fn csv_save(string path, arr[arr[string]] rows) {
    write_file(path, csv_serialize(rows))
}

fn csv_filter_by(arr[arr[string]] rows, int col, string value) -> arr[arr[string]] {
    return arr_filter(rows, fn (arr[string] row) -> bool {
        return row[col] == value
    })?
}

fn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {
    return arr_find(rows, fn (arr[string] row) -> bool {
        return row[COL_ID] == id
    })?
}

fn csv_next_id(arr[arr[string]] rows) -> string {
    if (len(rows) == 0) {
        return "1"
    }
    dec int max_id = arr_reduce(
    rows,
    fn (int acc, arr[string] row) -> int {
        dec int id = to_int(row[COL_ID])?
        if (id > acc) {
            return id
        }
        return acc
    },
    0
    )?
    return to_string(max_id + 1)?
}

fn csv_add_row(arr[arr[string]] rows, arr[string] fields) -> arr[arr[string]] {
    return arr_push(rows, fields)?
}

fn csv_remove_by_id(arr[arr[string]] rows, string id) -> arr[arr[string]] {
    return arr_filter(rows, fn (arr[string] row) -> bool {
        return row[COL_ID] != id
    })?
}

fn csv_update_field(arr[arr[string]] rows, string id, int col, string value) -> arr[arr[string]] {
    return arr_map(rows, fn (arr[string] row) -> arr[string] {
        if (row[COL_ID] != id) {
            return row
        }
        dec arr[string] updated = row
        updated[col] = value
        return updated
    })?
}
