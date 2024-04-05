/// Package License object, containing main license ID and possibly additional.
pub struct PkgLicence {
    id: String,
    other: Option<Vec<String>>,
}

impl PkgLicence {
    /// Create new instance of PkgLicence object
    pub fn new(id: String) -> Self {
        PkgLicence { id, other: None }
    }

    /// Returns the ID of the main license
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Returns true if there are other licences then just the main ID
    /// Returns other licenses, if any
    pub fn get_other(&self) -> Option<&Vec<String>> {
        self.other.as_ref()
    }

    pub fn add(&mut self, other: String) {
        if self.other.is_none() {
            self.other = Some(Vec::default());
        }

        self.other = self
            .other
            .as_mut()
            .map(|mut v| {
                v.push(other);
                v
            })
            .cloned();
    }
}
