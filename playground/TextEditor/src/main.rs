pub struct TextEditor {
    text: String,
}

impl TextEditor {
    pub fn new() -> TextEditor {
        TextEditor { text: String::new() }
    }

    pub fn add_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn reset(&mut self) {
        self.text = String::new();
    }
}

fn main() {
    let mut editor = TextEditor::new();
    editor.add_char('a');
    editor.add_char('b');
    editor.add_char('c');
    {
        let my_txt = editor.get_text();
        println!("{:?}", my_txt);
    }
    editor.reset();
}
