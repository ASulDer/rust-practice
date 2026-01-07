pub struct ListElement {
    pub element_text: String,
    pub isCompleted: bool,
}

impl ListElement {
    pub fn new(text: String) -> ListElement {
        ListElement {
            element_text: text,
            isCompleted: false,
        }
    }

    pub fn switch_completion(&mut self) {
        self.isCompleted = !self.isCompleted;
    }
}