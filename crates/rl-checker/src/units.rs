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

    fn add_exponent(&mut self, symbol: &str, amount: i32) {
        let new_exponent = self.exponent(symbol) + amount;

        if new_exponent == 0 {
            self.powers.remove(symbol);
        } else {
            self.powers.insert(symbol.to_owned(), new_exponent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Unit;

    #[test]
    fn creates_symbol_unit() {
        let meters = Unit::symbol("m");

        assert_eq!(meters.exponent("m"), 1);
        assert_eq!(meters.exponent("s"), 0);
    }

    #[test]
    fn multiplies_units_and_cancels_symbols() {
        let meters = Unit::symbol("m");
        let seconds = Unit::symbol("s");

        let speed = meters.divide(&seconds);
        let distance = speed.multiply(&seconds);

        assert_eq!(distance, Unit::symbol("m"));
    }

    #[test]
    fn divides_units() {
        let meters = Unit::symbol("m");
        let seconds = Unit::symbol("s");

        let speed = meters.divide(&seconds);

        assert_eq!(speed.exponent("m"), 1);
        assert_eq!(speed.exponent("s"), -1);
    }

    #[test]
    fn combines_repeated_symbols() {
        let meters = Unit::symbol("m");

        let area = meters.multiply(&meters);

        assert_eq!(area.exponent("m"), 2);
    }

    #[test]
    fn produces_dimensionless_unit_when_symbols_cancel() {
        let meters = Unit::symbol("m");

        let result = meters.divide(&meters);

        assert!(result.is_dimensionless());
    }

    #[test]
    fn only_equal_units_are_compatible() {
        let meters = Unit::symbol("m");
        let seconds = Unit::symbol("s");

        assert!(meters.is_compatible_with(&Unit::symbol("m")));
        assert!(!meters.is_compatible_with(&seconds));
    }
}