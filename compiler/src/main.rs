mod syntax_tree;
mod tokenize;

fn main() {
    let tokens = tokenize::tokenize(&std::fs::read_to_string("./example.skv").unwrap());
    let r = syntax_tree::syntax_tree(tokens.unwrap());

    match r {
        Ok(s) => {
            let j = serde_json::to_string_pretty(&s).unwrap();
            std::fs::write("test.json", j).unwrap();
        }
        Err(e) => panic!("Ln {} Col {} | {}", e.ln, e.col, e.reason),
    };
}
