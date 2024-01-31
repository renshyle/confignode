use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::error::{ParseError, ParseErrorKind};

/// Represents the value of a ConfigNode key
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigNodeValue {
    /// Any value that is not a new node. Numbers, lists, and other types of values are not parsed and are represented as their original textual form as this type.
    Text(String),
    /// A new node
    Node(ConfigNode),
}

/// Represents a node in the ConfigNode tree
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ConfigNode {
    /// All the direct children of this node
    pub children: HashMap<String, ConfigNodeValue>,
}

/// Parses a string into a [`ConfigNode`] struct
pub struct ConfigNodeParser<'a> {
    it: Peekable<Chars<'a>>,
}

impl ConfigNodeParser<'_> {
    /// Parses a ConfigNode string into a [`ConfigNode`] struct.
    pub fn parse(text: &str) -> Result<ConfigNode, ParseError> {
        let mut parser = ConfigNodeParser {
            it: text.chars().peekable(),
        };

        parser.parse_confignode()
    }

    // parses a node *without* the surrounding curly brackets
    fn parse_confignode(&mut self) -> Result<ConfigNode, ParseError> {
        let mut children = HashMap::new();

        self.skip_whitespace();

        loop {
            let identifier = match self.it.peek() {
                Some('}') | None => {
                    break;
                }
                Some('/') => {
                    self.it.next();

                    if self.it.peek() == Some(&'/') {
                        self.skip_line();
                        self.skip_whitespace();
                        continue;
                    } else {
                        let mut identifier = self.parse_identifier()?;
                        identifier.insert(0, '/');
                        identifier
                    }
                }
                Some(_) => self.parse_identifier()?,
            };

            self.skip_whitespace();

            match self.it.peek() {
                Some('{') => {
                    self.it.next();
                    children.insert(identifier, ConfigNodeValue::Node(self.parse_confignode()?));
                    self.skip_whitespace();

                    match self.it.next() {
                        Some('}') => {}
                        Some(_) => {
                            // i think this should be unreachable
                            return Err(ParseError {
                                kind: ParseErrorKind::InvalidCharacter,
                            });
                        }
                        None => {
                            return Err(ParseError {
                                kind: ParseErrorKind::UnexpectedEof,
                            })
                        }
                    }
                }
                Some('=') => {
                    self.it.next();
                    children.insert(identifier, ConfigNodeValue::Text(self.parse_string()?));
                }
                Some(_) => {
                    // i think this should be unreachable
                    return Err(ParseError {
                        kind: ParseErrorKind::InvalidCharacter,
                    });
                }
                None => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedEof,
                    })
                }
            }

            self.skip_whitespace();
        }

        Ok(ConfigNode { children })
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let mut identifier = String::new();

        loop {
            match self.it.peek() {
                Some('{') | Some('=') | Some('\n') | Some('\r') => {
                    break;
                }
                Some('/') => {
                    self.it.next();

                    if self.it.peek() == Some(&'/') {
                        self.skip_line();
                        break;
                    } else {
                        identifier.push('/');
                    }
                }
                Some('}') => {
                    return Err(ParseError {
                        kind: ParseErrorKind::InvalidCharacter,
                    })
                }
                Some(c) => {
                    identifier.push(*c);
                    self.it.next();
                }
                None => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedEof,
                    })
                }
            }
        }

        Ok(identifier.trim().to_owned())
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut string = String::new();

        while let Some(c) = self.it.peek() {
            if *c == '\n' || *c == '\r' {
                break;
            } else if *c == '/' {
                self.it.next();

                if self.it.peek() == Some(&'/') {
                    self.skip_line();
                    break;
                } else {
                    string.push('/');
                }
            } else {
                string.push(*c);
                self.it.next();
            }
        }

        Ok(string.trim().to_owned())
    }

    fn skip_line(&mut self) {
        while self.it.peek().map_or(false, |&x| x != '\n') {
            self.it.next();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.it.peek().map_or(false, |x| x.is_ascii_whitespace()) {
            self.it.next();
        }
    }
}

impl ConfigNodeValue {
    /// Returns a reference to the inner tring of the `ConfigNodeValue` enum if `self` is of type [`ConfigNodeValue::Text`],
    /// otherwise `None`.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ConfigNodeValue::Text(x) => Some(x),
            ConfigNodeValue::Node(_) => None,
        }
    }

    /// Returns a reference to the inner `ConfigNode` of the `ConfigNodeValue` enum if `self` is of type [`ConfigNodeValue::Node`],
    /// otherwise `None`.
    pub fn as_node(&self) -> Option<&ConfigNode> {
        match self {
            ConfigNodeValue::Text(_) => None,
            ConfigNodeValue::Node(x) => Some(x),
        }
    }
}
