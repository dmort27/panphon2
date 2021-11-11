# panphon2
PanPhon2 is a more performant version of Python PanPhon being written in Rust (and depending on `rspanphon`. It does not have all of the functionality of [PanPhon](https://github.com/dmort27/panphon), but it should be much faster.

The main class in PanPhon2 is `FeatureTable`. To instantiate a `FeatureTable`, the following is sufficient:

```python
>>> import panphon2
>>> ft = panphon2.FeatureTable()
```

`FeatureTable` currenly has these methods:

`phonemes(str) -> [str]` Takes a string in IPA and returns a list of phonemes.

`phonemes_to_vectors([str]) -> [[int]]` Takes a list of phonemes and returns a list of list of numeric feature values.

`word_to_vectors(str) -> [[int]]` Takes an IPA string and returns a list of lists of numberic feature values.

`feature_edit_distance(str, str) -> float` Takes two IPA strings and returns the feature-weighted distance between them.

`word_to_bag_of_features` Takes an IPA string and returns a vector of the sums of each dimension in the feature vectors corresponding to the phonemes.