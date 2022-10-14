use pyo3::prelude::*;
use rspanphon::featuretable;

#[pyclass]
struct FeatureTable {
    ft: featuretable::FeatureTable,
}

#[pymethods]
impl FeatureTable {
    //! The `FeatureTable` class implements the loading, extracting, and
    //! analysis of representations based on phonological (typically
    //! articulatory) features. It constructs a FeatureTable. Currently, the
    //! Python interface only supports the internal table that is compiled into
    //! the rust libraray, though adding support for arbitrary tables is planned.
    #[new]
    fn new() -> Self {
        FeatureTable {
            ft: featuretable::FeatureTable::new(),
        }
    }

    /// Returns the sequence of phonemes present in a Unicode IPA string, using
    /// the phoneme vocabulary from the FeatureTable.
    fn phonemes(&self, s: &str) -> Vec<String> {
        self.ft.phonemes(s)
    }

    /// Converts a list of phonemes into a list of lists of feature specifications.
    fn phonemes_to_vectors(&self, ps: Vec<String>) -> Vec<Vec<i8>> {
        self.ft.phonemes_to_vectors(ps)
    }

    /// Converts a word, in Unicode IPA, into a list of lists of feature specifications.
    fn word_to_vectors(&self, s: &str) -> Vec<Vec<i8>> {
        self.ft.phonemes_to_vectors(self.ft.phonemes(s))
    }

    /// Converts a word, in Unicode IPA, into a list of lists of binary features
    /// (with 0 and -1 collapsed to 0).
    fn word_to_binary_vectors(&self, s: &str) -> Vec<Vec<i8>> {
        self.ft
            .phonemes_to_vectors(self.ft.phonemes(s))
            .iter()
            .map(|f| f.iter().map(|g| *g.max(&0)).collect::<Vec<i8>>())
            .collect::<Vec<Vec<i8>>>()
    }

    /// Performs sum pooling on the feature vectures extracted from a word and
    /// returns the resulting matrix.
    fn word_to_bag_of_features(&self, s: &str) -> Vec<i8> {
        let mut tensor = self
            .ft
            .phonemes_to_vectors(self.ft.phonemes(s))
            .iter()
            .map(|f| f.iter().map(|g| *g.max(&0)).collect::<Vec<i8>>())
            .collect::<Vec<Vec<i8>>>();
        if let Some(head) = tensor.pop() {
            tensor.iter().fold(head, |acc, v| {
                acc.iter().zip(v).map(|(f1, f2)| f1 + f2).collect()
            })
        } else {
            panic!("Empty matrix");
        }
    }

    /// Returns the unweighted feature distance between to words.
    fn feature_edit_distance(&self, s1: &str, s2: &str) -> f64 {
        self.ft.feature_edit_distance(s1, s2)
    }
}

#[pymodule]
#[pyo3(name = "panphon2")]
fn panphon2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FeatureTable>()?;
    Ok(())
}
