//! CPF (Cadastro de Pessoa Física) - Brazilian Tax Identification Number
//!
//! CPF is an 11-digit number used to identify individuals in Brazil.
//! This module provides validation and formatting utilities for CPF numbers.

use crate::error::PcfError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// CPF (Brazilian Tax Identification Number)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cpf {
    /// The CPF number as a string (stored without formatting)
    number: String,
}

impl Cpf {
    /// Create a new CPF from a string
    ///
    /// The input can be formatted (XXX.XXX.XXX-XX) or unformatted (XXXXXXXXXXX)
    ///
    /// # Example
    /// ```
    /// use bss_oss_pcf::cpf::Cpf;
    ///
    /// let cpf = Cpf::new("123.456.789-09").unwrap();
    /// assert_eq!(cpf.as_str(), "12345678909");
    /// ```
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);

        if !Self::is_valid_format(&cleaned) {
            return Err(PcfError::InvalidSubscriberData(format!(
                "Invalid CPF format: {}",
                input
            )));
        }

        if !Self::validate_checksum(&cleaned) {
            return Err(PcfError::InvalidSubscriberData(format!(
                "Invalid CPF checksum: {}",
                input
            )));
        }

        Ok(Self { number: cleaned })
    }

    /// Get CPF as string (unformatted)
    pub fn as_str(&self) -> &str {
        &self.number
    }

    /// Get CPF as formatted string (XXX.XXX.XXX-XX)
    pub fn formatted(&self) -> String {
        format!(
            "{}.{}.{}-{}",
            &self.number[0..3],
            &self.number[3..6],
            &self.number[6..9],
            &self.number[9..11]
        )
    }

    /// Clean CPF string (remove formatting characters)
    fn clean(input: &str) -> String {
        input.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    /// Check if CPF has valid format (11 digits)
    fn is_valid_format(cleaned: &str) -> bool {
        cleaned.len() == 11 && cleaned.chars().all(|c| c.is_ascii_digit())
    }

    /// Validate CPF checksum digits
    ///
    /// CPF validation algorithm:
    /// 1. Calculate first check digit using first 9 digits
    /// 2. Calculate second check digit using first 10 digits
    /// 3. Compare with provided check digits
    fn validate_checksum(cpf: &str) -> bool {
        // Reject known invalid CPFs (all same digits)
        if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
            return false;
        }

        let digits: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // Calculate first check digit
        let mut sum = 0;
        for i in 0..9 {
            sum += digits[i] * (10 - i as u32);
        }
        let first_check = (sum * 10) % 11;
        let first_check = if first_check == 10 { 0 } else { first_check };

        if first_check != digits[9] {
            return false;
        }

        // Calculate second check digit
        let mut sum = 0;
        for i in 0..10 {
            sum += digits[i] * (11 - i as u32);
        }
        let second_check = (sum * 10) % 11;
        let second_check = if second_check == 10 { 0 } else { second_check };

        second_check == digits[10]
    }

    /// Generate a random valid CPF (for testing purposes only)
    #[cfg(test)]
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        // Generate first 9 digits
        let mut digits = vec![0u32; 9];
        for i in 0..9 {
            digits[i] = rng.random_range(0..=9);
        }

        // Calculate first check digit
        let mut sum = 0;
        for i in 0..9 {
            sum += digits[i] * (10 - i as u32);
        }
        let first_check = (sum * 10) % 11;
        digits.push(if first_check == 10 { 0 } else { first_check });

        // Calculate second check digit
        let mut sum = 0;
        for i in 0..10 {
            sum += digits[i] * (11 - i as u32);
        }
        let second_check = (sum * 10) % 11;
        digits.push(if second_check == 10 { 0 } else { second_check });

        let number: String = digits.iter().map(|d| d.to_string()).collect();
        Self { number }
    }
}

impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl TryFrom<&str> for Cpf {
    type Error = PcfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Cpf::new(value)
    }
}

impl TryFrom<String> for Cpf {
    type Error = PcfError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cpf::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cpf() {
        // Valid CPF examples
        let valid_cpfs = vec![
            "123.456.789-09",
            "12345678909",
            "111.444.777-35",
            "11144477735",
        ];

        for cpf_str in valid_cpfs {
            let cpf = Cpf::new(cpf_str);
            assert!(cpf.is_ok(), "CPF {} should be valid", cpf_str);
        }
    }

    #[test]
    fn test_invalid_cpf_format() {
        let invalid_cpfs = vec![
            "1234567890",     // Too short
            "123456789012",   // Too long
            "123.456.789",    // Missing check digits
            "abc.def.ghi-jk", // Non-numeric
        ];

        for cpf_str in invalid_cpfs {
            let cpf = Cpf::new(cpf_str);
            assert!(cpf.is_err(), "CPF {} should be invalid", cpf_str);
        }
    }

    #[test]
    fn test_invalid_cpf_checksum() {
        let invalid_cpfs = vec![
            "123.456.789-00", // Wrong check digits
            "111.111.111-11", // All same digits
            "000.000.000-00", // All zeros
        ];

        for cpf_str in invalid_cpfs {
            let cpf = Cpf::new(cpf_str);
            assert!(cpf.is_err(), "CPF {} should be invalid", cpf_str);
        }
    }

    #[test]
    fn test_cpf_formatting() {
        let cpf = Cpf::new("12345678909").unwrap();
        assert_eq!(cpf.formatted(), "123.456.789-09");
        assert_eq!(cpf.as_str(), "12345678909");
    }

    #[test]
    fn test_cpf_display() {
        let cpf = Cpf::new("12345678909").unwrap();
        assert_eq!(format!("{}", cpf), "123.456.789-09");
    }

    #[test]
    fn test_random_cpf() {
        let cpf = Cpf::random();
        assert_eq!(cpf.as_str().len(), 11);

        // Verify it's valid
        let validated = Cpf::new(cpf.as_str());
        assert!(validated.is_ok());
    }
}
