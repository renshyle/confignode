use std::collections::HashMap;

use confignode::{ConfigNode, ConfigNodeParser, ConfigNodeValue};

#[test]
fn comments() {
    assert_eq!(
        ConfigNodeParser::parse(
            "
            // test
            ABC = DEF"
        ),
        Ok(ConfigNode {
            children: HashMap::from([("ABC".to_owned(), ConfigNodeValue::Text("DEF".to_owned()))])
        })
    );

    assert_eq!(
        ConfigNodeParser::parse(
            "
            ABC = DE//F"
        ),
        Ok(ConfigNode {
            children: HashMap::from([("ABC".to_owned(), ConfigNodeValue::Text("DE".to_owned()))])
        })
    );

    assert_eq!(
        ConfigNodeParser::parse("A/B/C =/ DE/F//G"),
        Ok(ConfigNode {
            children: HashMap::from([(
                "A/B/C".to_owned(),
                ConfigNodeValue::Text("/ DE/F".to_owned())
            )])
        })
    );

    assert_eq!(
        ConfigNodeParser::parse(
            "GAME//test
            {
                Title = Career (CAREER)
            }"
        ),
        Ok(ConfigNode {
            children: HashMap::from([(
                "GAME".to_owned(),
                ConfigNodeValue::Node(ConfigNode {
                    children: HashMap::from([(
                        "Title".to_owned(),
                        ConfigNodeValue::Text("Career (CAREER)".to_owned())
                    )])
                })
            )])
        })
    );
}

#[test]
fn empty() {
    assert_eq!(
        ConfigNodeParser::parse(""),
        Ok(ConfigNode {
            children: HashMap::new()
        })
    );
}

#[test]
fn special_characters() {
    assert_eq!(
        ConfigNodeParser::parse(
            "!!@(#@/
            {
                +-\\() = \\=/-$(!)
            }"
        ),
        Ok(ConfigNode {
            children: HashMap::from([(
                "!!@(#@/".to_owned(),
                ConfigNodeValue::Node(ConfigNode {
                    children: HashMap::from([(
                        "+-\\()".to_owned(),
                        ConfigNodeValue::Text("\\=/-$(!)".to_owned())
                    )])
                })
            )])
        })
    );
}
