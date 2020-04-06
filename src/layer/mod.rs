//! CoNLL-X layer encoder.

use failure::Error;
use serde_derive::{Deserialize, Serialize};

use super::{EncodingProb, SentenceDecoder, SentenceEncoder};

mod error;
use self::error::*;

/// Tagging layer.
#[serde(rename_all = "lowercase")]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Layer {
    UPos,
    XPos,

    /// A specific morphologic feature.
    Feature {
        feature: String,

        // Default value if the feature is absent.
        default: Option<String>,
    },

    /// All morphological features represented as a string.
    #[serde(rename = "feature_string")]
    FeatureString,

    Misc {
        feature: String,

        // Default value if the feature is absent.
        default: Option<String>,
    },
}

impl Layer {
    /// Construct a feature layer.
    pub fn feature(feature: String, default: Option<String>) -> Self {
        Layer::Feature { feature, default }
    }

    /// Construct a miscellaneous feature layer.
    pub fn misc(feature: String, default: Option<String>) -> Self {
        Layer::Misc { feature, default }
    }
}

/// Layer values.
#[allow(clippy::len_without_is_empty)]
pub trait LayerValue {
    /// Get the form.
    fn form(&self, idx: usize) -> &str;

    /// Maximum node index.
    fn len(&self) -> usize;

    /// Set a layer value.
    fn set_value(&mut self, idx: usize, layer: &Layer, value: impl Into<String>);

    /// Get a layer value.
    fn value(&self, idx: usize, layer: &Layer) -> Option<String>;
}

/// Encode sentences using a CoNLL-X layer.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LayerEncoder {
    layer: Layer,
}

impl LayerEncoder {
    /// Construct a new layer encoder of the given layer.
    pub fn new(layer: Layer) -> Self {
        LayerEncoder { layer }
    }
}

impl<L> SentenceDecoder<L> for LayerEncoder
where
    L: LayerValue,
{
    type Encoding = String;

    fn decode<S>(&self, labels: &[S], sentence: &mut L) -> Result<(), Error>
    where
        S: AsRef<[EncodingProb<Self::Encoding>]>,
    {
        assert_eq!(
            labels.len(),
            sentence.len() - 1,
            "Labels and sentence length mismatch"
        );

        for (idx, label) in labels.iter().enumerate() {
            if let Some(label) = label.as_ref().get(0) {
                sentence.set_value(idx + 1, &self.layer, label.encoding().as_str());
            }
        }

        Ok(())
    }
}

impl<L> SentenceEncoder<L> for LayerEncoder
where
    L: LayerValue,
{
    type Encoding = String;

    fn encode(&self, sentence: &L) -> Result<Vec<Self::Encoding>, Error> {
        let mut encoding = Vec::with_capacity(sentence.len() - 1);

        for idx in 1..sentence.len() {
            let label =
                sentence
                    .value(idx, &self.layer)
                    .ok_or_else(|| EncodeError::MissingLabel {
                        form: sentence.form(idx).to_owned(),
                    })?;
            encoding.push(label.to_owned());
        }

        Ok(encoding)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::iter::FromIterator;

    use conllu::graph::Sentence;
    use conllu::token::{Features, Misc, Token, TokenBuilder};

    use crate::layer::{Layer, LayerValue};

    #[test]
    fn layer() {
        let token: Token = TokenBuilder::new("test")
            .upos("CP")
            .xpos("P")
            .features(Features::try_from("c=d|a=b").unwrap())
            .misc(Misc::from("u=v|x=y"))
            .into();

        let sent = Sentence::from_iter(vec![token]);

        assert_eq!(sent.value(1, &Layer::UPos), Some("CP".to_string()));
        assert_eq!(sent.value(1, &Layer::XPos), Some("P".to_string()));
        assert_eq!(
            sent.value(1, &Layer::feature("a".to_owned(), None)),
            Some("b".to_string())
        );
        assert_eq!(
            sent.value(1, &Layer::feature("c".to_owned(), None)),
            Some("d".to_string())
        );
        assert_eq!(sent.value(1, &Layer::feature("e".to_owned(), None)), None);
        assert_eq!(
            sent.value(
                1,
                &Layer::feature("e".to_owned(), Some("some_default".to_string()))
            ),
            Some("some_default".to_string())
        );
        assert_eq!(
            sent.value(1, &Layer::FeatureString),
            Some("a=b|c=d".to_string())
        );

        assert_eq!(
            sent.value(1, &Layer::misc("u".to_owned(), None)),
            Some("v".to_string())
        );
        assert_eq!(
            sent.value(1, &Layer::misc("x".to_owned(), None)),
            Some("y".to_string())
        );
        assert_eq!(sent.value(1, &Layer::misc("z".to_owned(), None)), None);
        assert_eq!(
            sent.value(
                1,
                &Layer::misc("z".to_owned(), Some("some_default".to_string()))
            ),
            Some("some_default".to_string())
        );
    }

    #[test]
    fn set_layer() {
        let token: Token = TokenBuilder::new("test").into();
        let mut sent = Sentence::from_iter(vec![token]);

        assert_eq!(sent.value(1, &Layer::FeatureString), Some("_".to_string()));

        sent.set_value(1, &Layer::UPos, "CP");
        sent.set_value(1, &Layer::XPos, "P");
        sent.set_value(1, &Layer::feature("a".to_owned(), None), "b");
        sent.set_value(1, &Layer::misc("u".to_owned(), None), "v");

        assert_eq!(sent.value(1, &Layer::UPos), Some("CP".to_string()));
        assert_eq!(sent.value(1, &Layer::XPos), Some("P".to_string()));
        assert_eq!(
            sent.value(1, &Layer::feature("a".to_owned(), None)),
            Some("b".to_string())
        );
        assert_eq!(sent.value(1, &Layer::feature("c".to_owned(), None)), None);
        assert_eq!(
            sent.value(1, &Layer::FeatureString),
            Some("a=b".to_string())
        );

        assert_eq!(
            sent.value(1, &Layer::misc("u".to_owned(), None)),
            Some("v".to_string())
        );
        assert_eq!(sent.value(1, &Layer::misc("x".to_owned(), None)), None);
    }
}
