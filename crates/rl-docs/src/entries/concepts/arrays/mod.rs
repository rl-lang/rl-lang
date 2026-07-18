use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ARRAYS: ConceptEntry = ConceptEntry {
    name: "arrays",
    summary: "fixed-type, growable arrays declared with `dec arr[<type>] <name> = [<items>]`, indexed with `arr[i]`, and manipulated through the `array` stdlib module - copied by value on assignment, not shared by reference",
    category: ConceptCategory::Syntax,
    prerequisites: &["types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring an array"),
            description: "declare a mutable array with `dec arr[<type>] <name> = [<items>]`",
            examples: &[
                "dec arr[int]   nums  = [1, 2, 3]",
                "dec arr[string]   words = [\"hello\", \"world\"]",
                "dec arr[float] vals  = [1.0, 2.0, 3.0]",
                "dec arr[char]  chars = ['.', 'r', 'l']",
                "dec arr[bool] bools = [true, false, true]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("indexing"),
            description: "access an element by index with `arr[i]`, zero-based",
            examples: &[
                "dec arr[int] nums = [10, 20, 30]\nprintln(nums[0])  // 10",
                "dec arr[int] nums = [10, 20, 30]\nprintln(nums[2])  // 30",
            ],
            expected_output: &["10", "30"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("assigning to an index"),
            description: "assign to an index with `arr[i] = value`",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\nnums[1] = 99\nprintln(nums)  // [1, 99, 3]",
            ],
            expected_output: &["[1, 99, 3]"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("arrays are homogeneous"),
            description: "arrays are typed - every element must be the same type",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\n// dec arr[int] bad = [1, \"two\"]  // error: type mismatch",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("nested arrays"),
            description: "nested arrays are supported",
            examples: &["dec arr[arr[int]] matrix = [[1, 2], [3, 4]]\nprintln(matrix[0][1])  // 2"],
            expected_output: &["2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("arrays are value types, not references"),
            description: "arrays are plain values, not references - assigning one array variable to another, or passing an array into a function, copies the whole array; mutating the copy never affects the original",
            examples: &[
                "dec arr[int] a = [1, 2, 3]\ndec arr[int] b = a\nb[0] = 99\nprintln(a[0])  // 1, unaffected by b's change",
            ],
            expected_output: &["1"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("array stdlib functions return new arrays"),
            description: "stdlib array functions like `arr_push`/`arr_pop`/`arr_sort` don't mutate their argument in place - they return a new array, which is why the result gets reassigned back (`nums = nums.arr_push(3)`)",
            examples: &[
                "get std::array::arr_push\n\ndec arr[int] nums = [1, 2]\nnums = nums.arr_push(3)\nprintln(nums)  // [1, 2, 3]",
            ],
            expected_output: &["[1, 2, 3]"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("out-of-bounds indexing errors"),
            description: "reading or writing an index outside the array's bounds raises a runtime error rather than returning `null` or silently growing the array - use `arr_push` to add elements instead of assigning past the end",
            examples: &[
                "// dec arr[int] nums = [1, 2, 3]\n// println(nums[5])  // runtime error: index 5 out of bounds (len 3)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no negative indexing"),
            description: "unlike Python-style negative indexing, `arr[-1]` isn't a way to get the last element - a literal negative index is a runtime error; use `arr_last`/`arr_first` from `std::array` instead",
            examples: &[
                "// dec arr[int] nums = [1, 2, 3]\n// println(nums[-1])  // runtime error: index cannot be negative: -1",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("empty array literals"),
            description: "an empty array literal `[]` type-checks against any declared array element type, since there are no elements to conflict with it",
            examples: &["dec arr[int] nums = []\nprintln(nums)  // []"],
            expected_output: &["[]"],
        },
    ],
    pitfalls: &[
        "arrays are value types, not references - assigning one array to another variable, or passing one into a function, copies the whole array; mutating the copy never affects the original",
        "stdlib array functions (`arr_push`, `arr_pop`, `arr_sort`, ...) return a new array instead of mutating in place - reassign the result to keep the change",
        "reading or writing an out-of-range index is a runtime error, not `null` or automatic growth - use `arr_push` to add elements instead of assigning past the end",
        "there's no negative indexing (`arr[-1]` for the last element) - a literal negative index raises a runtime error; use `arr_last`/`arr_first` from `std::array` instead",
    ],
    related: &["types", "tuples", "sets", "maps"],
    related_stdlib: &["array"],
    since: Some("v0.1.5"),
};
