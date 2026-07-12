use crate::entry::{FnEntry, StdEntry};
pub static MATH_CONSTS: StdEntry = StdEntry {
    name: "math::consts",
    description: "functions for math",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &E,
    &EULER_GAMMA,
    &FRAC_1_PI,
    &FRAC_1_SQRT_2,
    &FRAC_2_PI,
    &FRAC_2_SQRT_PI,
    &FRAC_PI_2,
    &FRAC_PI_3,
    &FRAC_PI_4,
    &FRAC_PI_6,
    &FRAC_PI_8,
    &INF,
    &IS_INF,
    &IS_NAN,
    &LN_10,
    &LN_2,
    &LOG10_2,
    &LOG10_E,
    &LOG2_E,
    &LOG2_10,
    &NAN,
    &PHI,
    &PI,
    &SQRT_2,
    &TAU,
];

static E: FnEntry = FnEntry {
    signature: "E()",
    description: "euler's number (~2.718)",
    example: "get std::math::consts::E\nE()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static EULER_GAMMA: FnEntry = FnEntry {
    signature: "EULER_GAMMA()",
    description: "euler-mascheroni constant (~0.577)",
    example: "get std::math::consts::EULER_GAMMA\nEULER_GAMMA()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_1_PI: FnEntry = FnEntry {
    signature: "FRAC_1_PI()",
    description: "1/π",
    example: "get std::math::consts::FRAC_1_PI\nFRAC_1_PI()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_1_SQRT_2: FnEntry = FnEntry {
    signature: "FRAC_1_SQRT_2()",
    description: "1/√2",
    example: "get std::math::consts::FRAC_1_SQRT_2\nFRAC_1_SQRT_2()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_2_PI: FnEntry = FnEntry {
    signature: "FRAC_2_PI()",
    description: "2/π",
    example: "get std::math::consts::FRAC_2_PI\nFRAC_2_PI()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_2_SQRT_PI: FnEntry = FnEntry {
    signature: "FRAC_2_SQRT_PI()",
    description: "2/√π",
    example: "get std::math::consts::FRAC_2_SQRT_PI\nFRAC_2_SQRT_PI()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_PI_2: FnEntry = FnEntry {
    signature: "FRAC_PI_2()",
    description: "π/2",
    example: "get std::math::consts::FRAC_PI_2\nFRAC_PI_2()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_PI_3: FnEntry = FnEntry {
    signature: "FRAC_PI_3()",
    description: "π/3",
    example: "get std::math::consts::FRAC_PI_3\nFRAC_PI_3()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_PI_4: FnEntry = FnEntry {
    signature: "FRAC_PI_4()",
    description: "π/4",
    example: "get std::math::consts::FRAC_PI_4\nFRAC_PI_4()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_PI_6: FnEntry = FnEntry {
    signature: "FRAC_PI_6()",
    description: "π/6",
    example: "get std::math::consts::FRAC_PI_6\nFRAC_PI_6()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FRAC_PI_8: FnEntry = FnEntry {
    signature: "FRAC_PI_8()",
    description: "π/8",
    example: "get std::math::consts::FRAC_PI_8\nFRAC_PI_8()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static INF: FnEntry = FnEntry {
    signature: "INF()",
    description: "positive infinity",
    example: "get std::math::consts::INF\nINF()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static IS_INF: FnEntry = FnEntry {
    signature: "is_inf(x)",
    description: "true if x is infinite",
    example: "get std::math::consts::is_inf\nis_inf(1.0)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static IS_NAN: FnEntry = FnEntry {
    signature: "is_nan(x)",
    description: "true if x is not a number",
    example: "get std::math::consts::is_nan\nis_nan(NAN())",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LN_2: FnEntry = FnEntry {
    signature: "LN_2()",
    description: "natural log of 2",
    example: "get std::math::consts::LN_2\nLN_2()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LN_10: FnEntry = FnEntry {
    signature: "LN_10()",
    description: "natural log of 10",
    example: "get std::math::consts::LN_10\nLN_10()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LOG2_10: FnEntry = FnEntry {
    signature: "LOG2_10()",
    description: "base-2 log of 10",
    example: "get std::math::consts::LOG2_10\nLOG2_10()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LOG2_E: FnEntry = FnEntry {
    signature: "LOG2_E()",
    description: "base-2 log of e",
    example: "get std::math::consts::LOG2_E\nLOG2_E()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LOG10_2: FnEntry = FnEntry {
    signature: "LOG10_2()",
    description: "base-10 log of 2",
    example: "get std::math::consts::LOG10_2\nLOG10_2()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LOG10_E: FnEntry = FnEntry {
    signature: "LOG10_E()",
    description: "base-10 log of e",
    example: "get std::math::consts::LOG10_E\nLOG10_E()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static NAN: FnEntry = FnEntry {
    signature: "NAN()",
    description: "not-a-number value",
    example: "get std::math::consts::NAN\nNAN()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PHI: FnEntry = FnEntry {
    signature: "PHI()",
    description: "golden ratio (~1.618)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
    example: "get std::math::consts::PHI\nPHI()",
};

static PI: FnEntry = FnEntry {
    signature: "PI()",
    description: "π (~3.14159)",
    example: "get std::math::consts::PI\nPI()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static SQRT_2: FnEntry = FnEntry {
    signature: "SQRT_2()",
    description: "square root of 2 (~1.414)",
    example: "get std::math::consts::SQRT_2\nSQRT_2()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TAU: FnEntry = FnEntry {
    signature: "TAU()",
    description: "τ = 2π (~6.283)",
    example: "get std::math::consts::TAU\nTAU()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
