mod anagram {
    pub fn anagram_for<'a>(source: &str, inputs: &[&'a str]) -> Vec<&'a str> {
        vec![]
    }
}

fn main() {
    let inputs = ["tan", "stand", "at"];
    anagram::anagram_for("ant", &inputs);
}
