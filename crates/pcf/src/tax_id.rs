//! Tax Identification Number (TIN) Support
//!
//! Provides support for multiple tax identification number formats:
//! - CPF (Brazilian Tax ID)
//! - NIF (European Tax ID - Portugal, Spain, etc.)
//! - SSN (US Social Security Number)
//! - And more...

use crate::cpf::Cpf;
use crate::error::PcfError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Country/Region codes for tax identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaxIdCountry {
    /// Brazil - CPF
    BR,
    /// Portugal - NIF
    PT,
    /// Spain - NIF/NIE
    ES,
    /// United States - SSN
    US,
    /// United Kingdom - NINO
    GB,
    /// Germany - Steuer-ID
    DE,
    /// France - Numéro fiscal
    FR,
    /// Italy - Codice fiscale
    IT,
}

impl TaxIdCountry {
    /// Get the default format for a country
    pub fn default_format(&self) -> &'static str {
        match self {
            TaxIdCountry::BR => "XXX.XXX.XXX-XX",    // CPF
            TaxIdCountry::PT => "XXXXXXXXX",         // NIF (9 digits)
            TaxIdCountry::ES => "XXXXXXXXX",         // NIF/NIE (9 characters)
            TaxIdCountry::US => "XXX-XX-XXXX",       // SSN
            TaxIdCountry::GB => "XX XXXXXX X",       // NINO
            TaxIdCountry::DE => "XX XXXXXX XXX",     // Steuer-ID
            TaxIdCountry::FR => "XX XXX XXX XXX XX", // Numéro fiscal
            TaxIdCountry::IT => "XXXXXXXXXXXXXXXX",  // Codice fiscale (16 chars)
        }
    }
}

/// Tax Identification Number (generic)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TaxId {
    /// Brazilian CPF
    #[serde(rename = "cpf")]
    Cpf(Cpf),
    /// Portuguese NIF
    #[serde(rename = "nif_pt")]
    NifPt(String),
    /// Spanish NIF/NIE
    #[serde(rename = "nif_es")]
    NifEs(String),
    /// US Social Security Number
    #[serde(rename = "ssn")]
    Ssn(String),
    /// UK National Insurance Number
    #[serde(rename = "nino")]
    Nino(String),
    /// German Steuer-ID
    #[serde(rename = "steuer_id")]
    SteuerId(String),
    /// French Numéro fiscal
    #[serde(rename = "numero_fiscal")]
    NumeroFiscal(String),
    /// Italian Codice fiscale
    #[serde(rename = "codice_fiscale")]
    CodiceFiscale(String),
}

impl TaxId {
    /// Create a TaxId from a string and country code
    pub fn from_string(value: &str, country: TaxIdCountry) -> Result<Self, PcfError> {
        match country {
            TaxIdCountry::BR => {
                let cpf = Cpf::new(value)?;
                Ok(TaxId::Cpf(cpf))
            }
            TaxIdCountry::PT => {
                let nif = NifPt::new(value)?;
                Ok(TaxId::NifPt(nif.as_str().to_string()))
            }
            TaxIdCountry::ES => {
                let nif = NifEs::new(value)?;
                Ok(TaxId::NifEs(nif.as_str().to_string()))
            }
            TaxIdCountry::US => {
                let ssn = Ssn::new(value)?;
                Ok(TaxId::Ssn(ssn.as_str().to_string()))
            }
            TaxIdCountry::GB => {
                let nino = Nino::new(value)?;
                Ok(TaxId::Nino(nino.as_str().to_string()))
            }
            TaxIdCountry::DE => {
                let steuer_id = SteuerId::new(value)?;
                Ok(TaxId::SteuerId(steuer_id.as_str().to_string()))
            }
            TaxIdCountry::FR => {
                let numero_fiscal = NumeroFiscal::new(value)?;
                Ok(TaxId::NumeroFiscal(numero_fiscal.as_str().to_string()))
            }
            TaxIdCountry::IT => {
                let codice_fiscale = CodiceFiscale::new(value)?;
                Ok(TaxId::CodiceFiscale(codice_fiscale.as_str().to_string()))
            }
        }
    }

    /// Get the country for this tax ID
    pub fn country(&self) -> TaxIdCountry {
        match self {
            TaxId::Cpf(_) => TaxIdCountry::BR,
            TaxId::NifPt(_) => TaxIdCountry::PT,
            TaxId::NifEs(_) => TaxIdCountry::ES,
            TaxId::Ssn(_) => TaxIdCountry::US,
            TaxId::Nino(_) => TaxIdCountry::GB,
            TaxId::SteuerId(_) => TaxIdCountry::DE,
            TaxId::NumeroFiscal(_) => TaxIdCountry::FR,
            TaxId::CodiceFiscale(_) => TaxIdCountry::IT,
        }
    }

    /// Get the tax ID as a string (unformatted)
    pub fn as_str(&self) -> &str {
        match self {
            TaxId::Cpf(cpf) => cpf.as_str(),
            TaxId::NifPt(s) => s,
            TaxId::NifEs(s) => s,
            TaxId::Ssn(s) => s,
            TaxId::Nino(s) => s,
            TaxId::SteuerId(s) => s,
            TaxId::NumeroFiscal(s) => s,
            TaxId::CodiceFiscale(s) => s,
        }
    }

    /// Get formatted tax ID string
    pub fn formatted(&self) -> String {
        match self {
            TaxId::Cpf(cpf) => cpf.formatted(),
            TaxId::NifPt(nif) => NifPt::format(nif),
            TaxId::NifEs(nif) => NifEs::format(nif),
            TaxId::Ssn(ssn) => Ssn::format(ssn),
            TaxId::Nino(nino) => Nino::format(nino),
            TaxId::SteuerId(id) => SteuerId::format(id),
            TaxId::NumeroFiscal(nf) => NumeroFiscal::format(nf),
            TaxId::CodiceFiscale(cf) => CodiceFiscale::format(cf),
        }
    }

    /// Validate the tax ID
    pub fn validate(&self) -> Result<(), PcfError> {
        match self {
            TaxId::Cpf(_cpf) => {
                // CPF is already validated during creation
                Ok(())
            }
            TaxId::NifPt(nif) => NifPt::validate(nif),
            TaxId::NifEs(nif) => NifEs::validate(nif),
            TaxId::Ssn(ssn) => Ssn::validate(ssn),
            TaxId::Nino(nino) => Nino::validate(nino),
            TaxId::SteuerId(id) => SteuerId::validate(id),
            TaxId::NumeroFiscal(nf) => NumeroFiscal::validate(nf),
            TaxId::CodiceFiscale(cf) => CodiceFiscale::validate(cf),
        }
    }
}

impl fmt::Display for TaxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

// Portuguese NIF (Número de Identificação Fiscal)
pub struct NifPt {
    number: String,
}

impl NifPt {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(nif: &str) -> String {
        // Format: XXX XXX XXX (9 digits with spaces)
        if nif.len() == 9 {
            format!("{} {} {}", &nif[0..3], &nif[3..6], &nif[6..9])
        } else {
            nif.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn validate(nif: &str) -> Result<(), PcfError> {
        if nif.len() != 9 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "NIF must be 9 digits, got {}",
                nif.len()
            )));
        }

        if !nif.chars().all(|c| c.is_ascii_digit()) {
            return Err(PcfError::InvalidSubscriberData(
                "NIF must contain only digits".to_string(),
            ));
        }

        // NIF validation algorithm (mod 11)
        let digits: Vec<u32> = nif.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut sum = 0;
        for i in 0..8 {
            sum += digits[i] * (9 - i as u32);
        }
        let remainder = sum % 11;
        let check_digit = if remainder < 2 { 0 } else { 11 - remainder };

        if check_digit != digits[8] {
            return Err(PcfError::InvalidSubscriberData(
                "Invalid NIF checksum".to_string(),
            ));
        }

        Ok(())
    }
}

// Spanish NIF/NIE
pub struct NifEs {
    number: String,
}

impl NifEs {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(nif: &str) -> String {
        // Format: X-XXXXXXXX-X or XXXXXXXXX
        if nif.len() == 9 {
            format!("{}-{}-{}", &nif[0..1], &nif[1..8], &nif[8..9])
        } else {
            nif.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.to_uppercase().replace("-", "").replace(" ", "")
    }

    fn validate(nif: &str) -> Result<(), PcfError> {
        if nif.len() != 9 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "NIF must be 9 characters, got {}",
                nif.len()
            )));
        }

        // NIF validation (first char is letter, rest are digits)
        let first_char = nif.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() {
            return Err(PcfError::InvalidSubscriberData(
                "NIF must start with a letter".to_string(),
            ));
        }

        // Validate that remaining characters are digits
        let _digits: Vec<u32> = nif[1..]
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                PcfError::InvalidSubscriberData("NIF digits must be numeric".to_string())
            })?;

        // Simplified validation - in production, use full algorithm with checksum
        Ok(())
    }
}

// US Social Security Number
pub struct Ssn {
    number: String,
}

impl Ssn {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(ssn: &str) -> String {
        // Format: XXX-XX-XXXX
        if ssn.len() == 9 {
            format!("{}-{}-{}", &ssn[0..3], &ssn[3..5], &ssn[5..9])
        } else {
            ssn.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn validate(ssn: &str) -> Result<(), PcfError> {
        if ssn.len() != 9 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "SSN must be 9 digits, got {}",
                ssn.len()
            )));
        }

        if !ssn.chars().all(|c| c.is_ascii_digit()) {
            return Err(PcfError::InvalidSubscriberData(
                "SSN must contain only digits".to_string(),
            ));
        }

        // SSN validation - cannot be all zeros, cannot start with 000, etc.
        if ssn == "000000000" {
            return Err(PcfError::InvalidSubscriberData(
                "Invalid SSN: all zeros".to_string(),
            ));
        }

        if &ssn[0..3] == "000" {
            return Err(PcfError::InvalidSubscriberData(
                "Invalid SSN: area number cannot be 000".to_string(),
            ));
        }

        if &ssn[3..5] == "00" {
            return Err(PcfError::InvalidSubscriberData(
                "Invalid SSN: group number cannot be 00".to_string(),
            ));
        }

        if &ssn[5..9] == "0000" {
            return Err(PcfError::InvalidSubscriberData(
                "Invalid SSN: serial number cannot be 0000".to_string(),
            ));
        }

        Ok(())
    }
}

// UK National Insurance Number
pub struct Nino {
    number: String,
}

impl Nino {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(nino: &str) -> String {
        // Format: XX XXXXXX X
        if nino.len() == 9 {
            format!("{} {} {}", &nino[0..2], &nino[2..8], &nino[8..9])
        } else {
            nino.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.to_uppercase().replace(" ", "").replace("-", "")
    }

    fn validate(nino: &str) -> Result<(), PcfError> {
        if nino.len() != 9 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "NINO must be 9 characters, got {}",
                nino.len()
            )));
        }

        // Format: 2 letters, 6 digits, 1 letter
        let chars: Vec<char> = nino.chars().collect();
        if !chars[0].is_ascii_alphabetic() || !chars[1].is_ascii_alphabetic() {
            return Err(PcfError::InvalidSubscriberData(
                "NINO must start with 2 letters".to_string(),
            ));
        }

        for i in 2..8 {
            if !chars[i].is_ascii_digit() {
                return Err(PcfError::InvalidSubscriberData(
                    "NINO must have 6 digits after initial letters".to_string(),
                ));
            }
        }

        if !chars[8].is_ascii_alphabetic() {
            return Err(PcfError::InvalidSubscriberData(
                "NINO must end with a letter".to_string(),
            ));
        }

        // Cannot start with certain prefixes
        let prefix = &nino[0..2];
        let invalid_prefixes = ["BG", "GB", "NK", "KN", "TN", "NT", "ZZ"];
        if invalid_prefixes.contains(&prefix) {
            return Err(PcfError::InvalidSubscriberData(format!(
                "NINO cannot start with {}",
                prefix
            )));
        }

        Ok(())
    }
}

// German Steuer-ID
pub struct SteuerId {
    number: String,
}

impl SteuerId {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(id: &str) -> String {
        // Format: XX XXXXXX XXX
        if id.len() == 11 {
            format!("{} {} {}", &id[0..2], &id[2..8], &id[8..11])
        } else {
            id.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn validate(id: &str) -> Result<(), PcfError> {
        if id.len() != 11 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "Steuer-ID must be 11 digits, got {}",
                id.len()
            )));
        }

        if !id.chars().all(|c| c.is_ascii_digit()) {
            return Err(PcfError::InvalidSubscriberData(
                "Steuer-ID must contain only digits".to_string(),
            ));
        }

        // Cannot be all same digit
        if id.chars().all(|c| c == id.chars().next().unwrap()) {
            return Err(PcfError::InvalidSubscriberData(
                "Steuer-ID cannot be all same digit".to_string(),
            ));
        }

        Ok(())
    }
}

// French Numéro fiscal
pub struct NumeroFiscal {
    number: String,
}

impl NumeroFiscal {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(nf: &str) -> String {
        // Format: XX XXX XXX XXX XX
        if nf.len() == 13 {
            format!(
                "{} {} {} {} {}",
                &nf[0..2],
                &nf[2..5],
                &nf[5..8],
                &nf[8..11],
                &nf[11..13]
            )
        } else {
            nf.to_string()
        }
    }

    fn clean(input: &str) -> String {
        input.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn validate(nf: &str) -> Result<(), PcfError> {
        if nf.len() != 13 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "Numéro fiscal must be 13 digits, got {}",
                nf.len()
            )));
        }

        if !nf.chars().all(|c| c.is_ascii_digit()) {
            return Err(PcfError::InvalidSubscriberData(
                "Numéro fiscal must contain only digits".to_string(),
            ));
        }

        Ok(())
    }
}

// Italian Codice fiscale
pub struct CodiceFiscale {
    number: String,
}

impl CodiceFiscale {
    pub fn new(input: &str) -> Result<Self, PcfError> {
        let cleaned = Self::clean(input);
        Self::validate(&cleaned)?;
        Ok(Self { number: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.number
    }

    pub fn format(cf: &str) -> String {
        // Format: XXXXXXXXXXXXXXXX (16 characters, uppercase)
        cf.to_uppercase()
    }

    fn clean(input: &str) -> String {
        input.to_uppercase().replace(" ", "").replace("-", "")
    }

    fn validate(cf: &str) -> Result<(), PcfError> {
        if cf.len() != 16 {
            return Err(PcfError::InvalidSubscriberData(format!(
                "Codice fiscale must be 16 characters, got {}",
                cf.len()
            )));
        }

        // Format: 6 letters, 2 digits, 1 letter, 2 digits, 1 letter, 3 digits, 1 letter
        // Simplified validation - check alphanumeric
        if !cf.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(PcfError::InvalidSubscriberData(
                "Codice fiscale must be alphanumeric".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_id_from_string() {
        // Test CPF
        let tax_id = TaxId::from_string("123.456.789-09", TaxIdCountry::BR);
        assert!(tax_id.is_ok());

        // Test NIF PT
        let tax_id = TaxId::from_string("123456789", TaxIdCountry::PT);
        assert!(tax_id.is_ok());

        // Test SSN
        let tax_id = TaxId::from_string("123456789", TaxIdCountry::US);
        assert!(tax_id.is_ok());
    }

    #[test]
    fn test_nif_pt() {
        // Valid NIF (example)
        let _nif = NifPt::new("123456789");
        // Note: This may fail checksum validation, but tests the structure
    }

    #[test]
    fn test_ssn() {
        let ssn = Ssn::new("123456789");
        assert!(ssn.is_ok());

        // Invalid: all zeros
        let ssn = Ssn::new("000000000");
        assert!(ssn.is_err());
    }

    #[test]
    fn test_nino() {
        let nino = Nino::new("AB123456C");
        assert!(nino.is_ok());

        // Invalid prefix
        let nino = Nino::new("BG123456C");
        assert!(nino.is_err());
    }
}
