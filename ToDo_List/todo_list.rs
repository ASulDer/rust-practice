pub mod list_element;
use list_element::ListElement;

pub struct ToDoList {
    taskList: Vec<ListElement>,
}

impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList {
            taskList: Vec::new(),
        }
    }

    pub fn switch_taskCompletion(&mut self, index: usize) {
        if index < self.taskList.len() {
            self.taskList[index].switch_completion();
        }
    }

    pub fn add_task(&mut self, text: String) {
        let task = ListElement::new(text);
        self.taskList.push(task);
    }

    pub fn swap_tasks(&mut self, index1: usize, index2: usize) {
        if index1 < self.taskList.len() && index2 < self.taskList.len() {
            &let helpElement = &self.taskList[index1];
            &self.taskList[index1] = &self.taskList[index2];
            &self.taskList[index2] = &helpElement;
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.taskList.len() {
            self.taskList.remove(index);
        }
    }

    pub fn get_len(&self) -> usize {
        self.taskList.len()
    }

    pub fn print(&self) {
        for (i, task) in self.taskList.iter().enumerate() {
            let status = if task.isCompleted { "+" } else { "x" };
            println!("{}: {} [{}]", i + 1, task.element_text, status);
        }
    }

    // should put the text in console editor and let user edit it
    // pub fn edit_task(index: u64) {};
}