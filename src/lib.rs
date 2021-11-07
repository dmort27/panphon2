use pyo3::prelude::*;
use rspanphon::featuretable;

#[pyclass]
struct FeatureTable {
    ft: featuretable::FeatureTable,
}

#[pymethods]
impl FeatureTable {

    #[new]
    fn new() -> Self {
        FeatureTable {
            ft: featuretable::FeatureTable::new()
        }
    }

    fn phonemes(&self, s: &str) -> Vec<String> {
        self.ft.phonemes(s)
    }

    fn phonemes_to_vectors(&self, ps: Vec<String>) -> Vec<Vec<i8>> {
        self.ft.phonemes_to_vectors(&ps)
    }

    fn word_to_vectors(&self, s: &str) -> Vec<Vec<i8>> {
        self.ft.phonemes_to_vectors(&self.ft.phonemes(s))
    }

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
