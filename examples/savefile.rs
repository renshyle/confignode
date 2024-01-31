fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let savefile =
        confignode::ConfigNodeParser::parse(&std::fs::read_to_string(filename).unwrap()).unwrap();
    let game = savefile.children.get("GAME").unwrap().as_node().unwrap();

    println!(
        "Title: {}",
        game.children.get("Title").unwrap().as_text().unwrap()
    );
    println!(
        "Version: {}",
        game.children.get("version").unwrap().as_text().unwrap()
    );
    println!(
        "Version created: {}",
        game.children
            .get("versionCreated")
            .unwrap()
            .as_text()
            .unwrap()
    );
}
