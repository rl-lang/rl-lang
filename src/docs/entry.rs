// function entry that contains signature (for example `pop(arr)`)
// and description of what said function can and cannot do
// and usage example in `rl`
pub struct FnEntry {
    // what function look like
    pub signature: &'static str,
    // what function can do and use cases
    pub description: &'static str,
    // how to use it in `rl`
    pub example: &'static str,
}

// std entry that contains std name and description about it
// and has functions entry
pub struct StdEntry {
    // what is the std entry called
    pub name: &'static str,
    // what is the std entry used for
    pub description: &'static str,
    // std functions
    pub functions: &'static [&'static FnEntry],
}

// entry for explaining concepts of `rl` like variables
// each concept entry can have multiple discriptions and each
// discription can have multiple example
pub struct ConceptEntry {
    // concept name
    pub name: &'static str,
    // descriptions about the concept (description and one or multiple examples)
    pub descriptions: &'static [DescriptionEntry],
}

// entry for descriptions
// one entry can have multiple examples
// useful for diffrent approach examples
pub struct DescriptionEntry {
    pub description: &'static str,
    pub examples: &'static [&'static str],
}
