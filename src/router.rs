extern crate trie_rs;
use trie_rs::{TrieBuilder, Trie};

// idea: you don't need a trie! just use a hash
// maybe?

pub struct RouterBuilder {
  trie_builder: TrieBuilder<&'static str>,
}

impl RouterBuilder {
  pub fn new() -> RouterBuilder {
    RouterBuilder {
      trie_builder: TrieBuilder::new()
    }
  }

  pub fn build(&self) -> Router {
    Router {
      trie: self.trie_builder.build()
    }
  }
}

pub struct Router {
  trie: Trie<&'static str>,
}

impl Router {

}
