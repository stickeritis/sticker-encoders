use conllu::graph::{Node, Sentence as ConlluSentence};
use conllu::token::Token;

pub trait Sentence {
    type Token;

    fn len(&self) -> usize;

    fn token(&self, idx: usize) -> Option<&Self::Token>;

    fn token_mut(&mut self, idx: usize) -> Option<&mut Self::Token>;
}

impl Sentence for ConlluSentence {
    type Token = Token;

    fn len(&self) -> usize {
        ConlluSentence::len(self)
    }

    fn token(&self, idx: usize) -> Option<&Self::Token> {
        match &self[idx] {
            Node::Root => None,
            Node::Token(token) => Some(token),
        }
    }

    fn token_mut(&mut self, idx: usize) -> Option<&mut Self::Token> {
        match &mut self[idx] {
            Node::Root => None,
            Node::Token(token) => Some(token),
        }
    }
}

pub trait Form {
    fn form(&self) -> &str;
}

impl Form for Token {
    fn form(&self) -> &str {
        self.form()
    }
}

pub trait Lemma {
    fn lemma(&self) -> Option<&str>;

    fn set_lemma(&mut self, lemma: impl Into<String>);
}

impl Lemma for Token {
    fn lemma(&self) -> Option<&str> {
        self.lemma()
    }

    fn set_lemma(&mut self, lemma: impl Into<String>) {
        Token::set_lemma(self, Some(lemma));
    }
}
