use crate::entry::{FnEntry, StdEntry};
pub static MATH_CONSTS: StdEntry = StdEntry {
    name: "math::consts",
    description: "functions for math",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
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
    example: "get std::math::consts::E\n\nE()",
    expected_output: Some("2.718281828459045"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static EULER_GAMMA: FnEntry = FnEntry {
    signature: "EULER_GAMMA()",
    description: "euler-mascheroni constant (~0.577)",
    example: "get std::math::consts::EULER_GAMMA\n\nEULER_GAMMA()",
    expected_output: Some("0.5772156649015329"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_1_PI: FnEntry = FnEntry {
    signature: "FRAC_1_PI()",
    description: "1/π",
    example: "get std::math::consts::FRAC_1_PI\n\nFRAC_1_PI()",
    expected_output: Some("0.3183098861837907"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_1_SQRT_2: FnEntry = FnEntry {
    signature: "FRAC_1_SQRT_2()",
    description: "1/√2",
    example: "get std::math::consts::FRAC_1_SQRT_2\n\nFRAC_1_SQRT_2()",
    expected_output: Some("0.7071067811865476"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_2_PI: FnEntry = FnEntry {
    signature: "FRAC_2_PI()",
    description: "2/π",
    example: "get std::math::consts::FRAC_2_PI\n\nFRAC_2_PI()",
    expected_output: Some("0.6366197723675814"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_2_SQRT_PI: FnEntry = FnEntry {
    signature: "FRAC_2_SQRT_PI()",
    description: "2/√π",
    example: "get std::math::consts::FRAC_2_SQRT_PI\n\nFRAC_2_SQRT_PI()",
    expected_output: Some("1.1283791670955126"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_PI_2: FnEntry = FnEntry {
    signature: "FRAC_PI_2()",
    description: "π/2",
    example: "get std::math::consts::FRAC_PI_2\n\nFRAC_PI_2()",
    expected_output: Some("1.5707963267948966"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_PI_3: FnEntry = FnEntry {
    signature: "FRAC_PI_3()",
    description: "π/3",
    example: "get std::math::consts::FRAC_PI_3\n\nFRAC_PI_3()",
    expected_output: Some("1.0471975511965976"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_PI_4: FnEntry = FnEntry {
    signature: "FRAC_PI_4()",
    description: "π/4",
    example: "get std::math::consts::FRAC_PI_4\n\nFRAC_PI_4()",
    expected_output: Some("0.7853981633974483"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_PI_6: FnEntry = FnEntry {
    signature: "FRAC_PI_6()",
    description: "π/6",
    example: "get std::math::consts::FRAC_PI_6\n\nFRAC_PI_6()",
    expected_output: Some("0.5235987755982988"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static FRAC_PI_8: FnEntry = FnEntry {
    signature: "FRAC_PI_8()",
    description: "π/8",
    example: "get std::math::consts::FRAC_PI_8\n\nFRAC_PI_8()",
    expected_output: Some("0.39269908169872414"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static INF: FnEntry = FnEntry {
    signature: "INF()",
    description: "positive infinity",
    example: "get std::math::consts::INF\n\nINF()",
    expected_output: Some("inf"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static IS_INF: FnEntry = FnEntry {
    signature: "is_inf(x)",
    description: "true if x is infinite",
    example: "get std::math::consts::is_inf\n\nis_inf(1.0)",
    expected_output: Some("false"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static IS_NAN: FnEntry = FnEntry {
    signature: "is_nan(x)",
    description: "true if x is not a number",
    example: "get std::math::consts::NAN\nget std::math::consts::is_nan\n\nis_nan(NAN())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LN_2: FnEntry = FnEntry {
    signature: "LN_2()",
    description: "natural log of 2",
    example: "get std::math::consts::LN_2\n\nLN_2()",
    expected_output: Some("0.6931471805599453"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LN_10: FnEntry = FnEntry {
    signature: "LN_10()",
    description: "natural log of 10",
    example: "get std::math::consts::LN_10\n\nLN_10()",
    expected_output: Some("2.302585092994046"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LOG2_10: FnEntry = FnEntry {
    signature: "LOG2_10()",
    description: "base-2 log of 10",
    example: "get std::math::consts::LOG2_10\n\nLOG2_10()",
    expected_output: Some("3.321928094887362"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LOG2_E: FnEntry = FnEntry {
    signature: "LOG2_E()",
    description: "base-2 log of e",
    example: "get std::math::consts::LOG2_E\n\nLOG2_E()",
    expected_output: Some("1.4426950408889634"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LOG10_2: FnEntry = FnEntry {
    signature: "LOG10_2()",
    description: "base-10 log of 2",
    example: "get std::math::consts::LOG10_2\n\nLOG10_2()",
    expected_output: Some("0.3010299956639812"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static LOG10_E: FnEntry = FnEntry {
    signature: "LOG10_E()",
    description: "base-10 log of e",
    example: "get std::math::consts::LOG10_E\n\nLOG10_E()",
    expected_output: Some("0.4342944819032518"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static NAN: FnEntry = FnEntry {
    signature: "NAN()",
    description: "not-a-number value",
    example: "get std::math::consts::NAN\n\nNAN()",
    expected_output: Some("NaN"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static PHI: FnEntry = FnEntry {
    signature: "PHI()",
    description: "golden ratio (~1.618)",
    expected_output: Some("1.618033988749895"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
    example: "get std::math::consts::PHI\n\nPHI()",
};

static PI: FnEntry = FnEntry {
    signature: "PI()",
    description: "π (~3.14159)",
    example: "get std::math::consts::PI\n\nPI()",
    expected_output: Some("3.141592653589793"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static SQRT_2: FnEntry = FnEntry {
    signature: "SQRT_2()",
    description: "square root of 2 (~1.414)",
    example: "get std::math::consts::SQRT_2\n\nSQRT_2()",
    expected_output: Some("1.4142135623730951"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static TAU: FnEntry = FnEntry {
    signature: "TAU()",
    description: "τ = 2π (~6.283)",
    example: "get std::math::consts::TAU\n\nTAU()",
    expected_output: Some("6.283185307179586"),
    returns: "float",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
