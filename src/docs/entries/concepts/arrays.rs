use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static ARRAYS: ConceptEntry = ConceptEntry {
    name: "arrays",
    descriptions: &[
        DescriptionEntry {
            description: "declare a mutable array with `dec arr[<type>] <name> = [<items>]`",
            examples: &[
                "dec arr[int]   nums  = [1, 2, 3]",
                "dec arr[string]   words = [\"hello\", \"world\"]",
                "dec arr[float] vals  = [1.0, 2.0, 3.0]",
                "dec arr[char]  chars = ['.', 'r', 'l']",
                "dec arr[bool] bools = [true, false, true]",
            ],
        },
        DescriptionEntry {
            description: "access an element by index with `arr[i]` zero-based",
            examples: &[
                "dec arr[int] nums = [10, 20, 30]\nprintln(nums[0])  // 10\nprintln(nums[2])  // 30",
            ],
        },
        DescriptionEntry {
            description: "assign to an index with `arr[i] = value`",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\nnums[1] = 99\nprintln(nums)  // [1, 99, 3]",
            ],
        },
        DescriptionEntry {
            description: "arrays are typed and all elements must be the same type",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\n// dec arr[int] bad = [1, \"two\"]  // error: type mismatch",
            ],
        },
        DescriptionEntry {
            description: "nested arrays are supported",
            examples: &["dec arr[arr[int]] matrix = [[1, 2], [3, 4]]\nprintln(matrix[0][1])  // 2"],
        },
    ],
};
