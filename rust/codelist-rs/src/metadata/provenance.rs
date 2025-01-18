//! This file contains the provenance struct and its implementation

// External imports
use chrono::Utc;
use std::collections::HashSet;

// Internal imports
use crate::metadata::metadata_source::MetadataSource;
use crate::errors::CodeListError;

pub struct Provenance {
    pub source: MetadataSource,          
    pub created_date: chrono::DateTime<Utc>,
    pub last_modified_date: chrono::DateTime<Utc>,
    pub contributors: HashSet<String>,
}

impl Provenance {
    /// Create a new provenance
    ///
    /// # Arguments
    /// * `source` - The source of the codelist
    pub fn new(source: MetadataSource, contributors: Option<HashSet<String>>) -> Provenance {
        Provenance {
            source,
            created_date: chrono::Utc::now(),
            last_modified_date: chrono::Utc::now(),
            contributors: contributors.unwrap_or_default(),
        }
    }

    /// Update the last modified date
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    pub fn update_last_modified_date(&mut self) {
        self.last_modified_date = chrono::Utc::now();
    }

    /// Add a contributor to the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to add
    pub fn add_contributor(&mut self, contributor: String) {
        self.contributors.insert(contributor);
    }

    /// Remove a contributor from the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to remove
    pub fn remove_contributor(&mut self, contributor: String) -> Result<(), CodeListError> {
        if self.contributors.remove(&contributor) {
            Ok(())
        } else {
            Err(CodeListError::contributor_not_found(contributor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper function to get the time difference between the current time and the given date
    fn get_time_difference(date: chrono::DateTime<Utc>) -> i64 {
        let now = chrono::Utc::now();
        (date - now).num_milliseconds().abs()
    }

    fn create_test_provenance() -> Provenance {
        Provenance::new(MetadataSource::LoadedFromFile, None)
    }

    #[test]
    fn test_new_provenance_no_contributors() {
        let provenance = create_test_provenance();
        assert_eq!(provenance.source, MetadataSource::LoadedFromFile);
        let time_difference = get_time_difference(provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(provenance.contributors, HashSet::new());
    }

    #[test]
    fn test_new_provenance_with_contributors() {
        let provenance = Provenance::new(MetadataSource::LoadedFromFile, Some(HashSet::from(["Example Contributor".to_string()])));
        assert_eq!(provenance.source, MetadataSource::LoadedFromFile);
        assert_eq!(provenance.contributors, HashSet::from(["Example Contributor".to_string()]));
        let time_difference = get_time_difference(provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
    }

    #[test]
    fn test_update_last_modified_date() {
        let mut provenance = create_test_provenance();
        provenance.update_last_modified_date();
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
    }

    #[test]
    fn test_add_contributor() {
        let mut provenance = create_test_provenance();
        provenance.add_contributor("Example Contributor".to_string());
        assert_eq!(provenance.contributors, HashSet::from(["Example Contributor".to_string()]));
    }

    #[test]
    fn test_remove_contributor() -> Result<(), CodeListError> {
        let mut provenance = create_test_provenance();
        provenance.add_contributor("Example Contributor".to_string());
        provenance.remove_contributor("Example Contributor".to_string())?;
        assert_eq!(provenance.contributors, HashSet::new());
        Ok(())
    }

    #[test]
    fn test_remove_contributor_not_found() {
        let mut provenance = create_test_provenance();
        let error = provenance.remove_contributor("Example Contributor".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Contributor Example Contributor not found");
    }
}