pub mod cpritf;
pub mod debcpr;

/// Package License object, containing main license ID and possibly additional.
pub struct PkgLicence {
    id: String,
    other: Vec<String>,
}

impl PkgLicence {
    /// Create new instance of PkgLicence object
    pub fn new(id: String) -> Self {
        PkgLicence { id, other: Vec::default() }
    }

    /// Returns the ID of the main license
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Returns true if there are other licences then just the main ID
    pub fn has_other(&self) -> bool {
        return !self.get_other().is_empty();
    }

    /// Returns other licenses, if any
    pub fn get_other(&self) -> &Vec<String> {
        self.other.as_ref()
    }

    pub fn add(&mut self, other: String) {
        self.other.push(other);
    }
}
