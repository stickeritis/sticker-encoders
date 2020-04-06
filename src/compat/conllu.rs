use std::convert::TryFrom;

use conllu::graph::Sentence;
use conllu::token::Features;

use crate::layer::{Layer, LayerValue};
use crate::lemma::Lemmas;

impl LayerValue for Sentence {
    fn form(&self, idx: usize) -> &str {
        let node = &self[idx];
        assert!(node.is_token(), "Attempted to set value on root node");
        let token = self[idx].token().unwrap();
        token.form()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn set_value(&mut self, idx: usize, layer: &Layer, value: impl Into<String>) {
        let node = &mut self[idx];
        assert!(node.is_token(), "Attempted to set value on root node");
        let token = self[idx].token_mut().unwrap();

        let value = value.into();

        match layer {
            Layer::UPos => {
                token.set_upos(Some(value));
            }
            Layer::XPos => {
                token.set_xpos(Some(value));
            }
            Layer::Feature { feature, .. } => {
                token.features_mut().insert(feature.clone(), value);
            }
            Layer::FeatureString => {
                token.set_features(
                    Features::try_from(value.as_str()).expect("Invalid feature representation"),
                );
            }
            Layer::Misc { feature, .. } => {
                token.misc_mut().insert(feature.clone(), Some(value));
            }
        };
    }

    fn value(&self, idx: usize, layer: &Layer) -> Option<String> {
        let node = &self[idx];
        assert!(node.is_token(), "Attempted to get value from root node");
        let token = self[idx].token().unwrap();

        match layer {
            Layer::UPos => token.upos().map(ToOwned::to_owned),
            Layer::XPos => token.xpos().map(ToOwned::to_owned),
            Layer::FeatureString => Some(token.features().into()),
            Layer::Feature { feature, default } => token
                .features()
                .get(feature)
                .cloned()
                .or_else(|| default.clone()),
            Layer::Misc { feature, default } => match token.misc().get(feature) {
                // Feature with an associated value.
                Some(Some(ref val)) => Some(val.clone()),

                // Feature without an associated value, should not be used.
                Some(None) => None,

                // The feature is absent.
                None => default.clone(),
            },
        }
    }
}

impl Lemmas for Sentence {
    fn form(&self, idx: usize) -> &str {
        let node = &self[idx];
        assert!(node.is_token(), "Attempted to get form of root node");
        let token = self[idx].token().unwrap();
        token.form()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn lemma(&self, idx: usize) -> Option<&str> {
        let node = &self[idx];
        assert!(node.is_token(), "Attempted to get lemma from root node");
        let token = self[idx].token().unwrap();
        token.lemma()
    }

    fn set_lemma(&mut self, idx: usize, lemma: impl Into<String>) {
        let node = &mut self[idx];
        assert!(node.is_token(), "Attempted to set lemma on root node");
        let token = self[idx].token_mut().unwrap();
        token.set_lemma(Some(lemma.into()));
    }
}
