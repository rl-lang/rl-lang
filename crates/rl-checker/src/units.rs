use rl_ast::statements::UnitAnnotation;
use std::collections::BTreeMap;

/// Normalized representation of a unit of measure.
///
/// Each symbol is stored with its exponent:
///
/// - `m`       => `{ "m": 1 }`
/// - `m / s`   => `{ "m": 1, "s": -1 }`
/// - `m / s²`  => `{ "m": 1, "s": -2 }`
/// - `m * s²`  => `{ "m": 1, "s": 2 }`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unit {
    powers: BTreeMap<String, i32>,
}

impl Unit {
    /// Create a dimensionless unit
    ///
    /// Example: m / m.
    pub fn dimensionless() -> Self {
        Self::default()
    }

    /// Creates a unit containing one symbol with exponent `1`.
    pub fn symbol(name: impl Into<String>) -> Self {
        let mut unit = Self::dimensionless();
        unit.powers.insert(name.into(), 1);
        unit
    }

    /// Creates a unit containing one symbol with the given exponent.
    pub fn powers(name: impl Into<String>, power: i32) -> Self {
        let mut unit = Self::dimensionless();
        unit.powers.insert(name.into(), power);
        unit
    }

    /// Returns the exponent associated with a symbol.
    ///
    /// Symbols not present in the unit have exponent `0`.
    pub fn exponent(&self, symbol: &str) -> i32 {
        self.powers.get(symbol).copied().unwrap_or(0)
    }

    /// Returns `true` if the unit is dimensionless.
    pub fn is_dimensionless(&self) -> bool {
        self.powers.is_empty()
    }

    /// Addition and subtraction require compatible units.
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self == other
    }

    /// Multiplies two units by adding their exponents.
    ///
    /// `(m / s) * s = m`
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for (symbol, exponent) in &other.powers {
            result.add_exponent(symbol, *exponent);
        }

        result
    }

    /// Divides two units by subtracting their exponents.
    ///
    /// `m / s = m * s⁻¹`
    pub fn divide(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for (symbol, exponent) in &other.powers {
            result.add_exponent(symbol, -*exponent);
        }

        result
    }

    /// Constructs a new instance of `Self` based on the provided `UnitAnnotation`.
    ///
    /// This function recursively processes a `UnitAnnotation` to create a corresponding
    /// instance of `Self`. The behavior depends on the variant of the `UnitAnnotation`
    /// input:
    ///
    /// - If the annotation is `UnitAnnotation::Symbol`, the `symbol` constructor is
    ///   called with the associated `name`.
    /// - If the annotation is `UnitAnnotation::Multiply`, the function recursively
    ///   processes both the `left` and `right` annotations, then combines their results
    ///   using the `multiply` method.
    /// - If the annotation is `UnitAnnotation::Divide`, the function recursively
    ///   processes both the `left` and `right` annotations, then combines their results
    ///   using the `divide` method.
    pub fn from_annotation(annotation: &UnitAnnotation) -> Self {
        match annotation {
            UnitAnnotation::Symbol(name) => Self::symbol(name),

            UnitAnnotation::Multiply(left, right) => {
                Self::from_annotation(left).multiply(&Self::from_annotation(right))
            }

            UnitAnnotation::Divide(left, right) => {
                Self::from_annotation(left).divide(&Self::from_annotation(right))
            }
        }
    }

    fn add_exponent(&mut self, symbol: &str, amount: i32) {
        let new_exponent = self.exponent(symbol) + amount;

        if new_exponent == 0 {
            self.powers.remove(symbol);
        } else {
            self.powers.insert(symbol.to_owned(), new_exponent);
        }
    }
}