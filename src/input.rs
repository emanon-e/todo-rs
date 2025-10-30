use super::todo::Todo;
use colored::Colorize;
use core::fmt;
use dialoguer::{Input, Select};
use std::result::Result;

pub enum InputResult<T> {
    Result(Result<T, dialoguer::Error>),
    Quit,
}

pub enum MenuOperation {
    ListAllTodos,
    AddTodo,
}

impl fmt::Display for MenuOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            MenuOperation::ListAllTodos => "List all todos",
            MenuOperation::AddTodo => "Add todo",
        };
        write!(f, "{}", text)
    }
}

const MENU_OPERATIONS: [&'static MenuOperation; 2] =
    [&MenuOperation::ListAllTodos, &MenuOperation::AddTodo];

pub enum TodoOperation {
    ToggleIsCompleted,
    EditText,
    Delete,
}

impl fmt::Display for TodoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &'static str = match self {
            TodoOperation::ToggleIsCompleted => "Toggle is completed",
            TodoOperation::EditText => "Edit todo text",
            TodoOperation::Delete => "Delete",
        };
        write!(f, "{}", text)
    }
}

const TODO_OPERATIONS: [&'static TodoOperation; 3] = [
    &TodoOperation::ToggleIsCompleted,
    &TodoOperation::EditText,
    &TodoOperation::Delete,
];

pub fn select_menu_operation() -> InputResult<&'static MenuOperation> {
    let menu_operation = Select::new()
        .with_prompt(format!(
            "{}\n{}",
            "Press Esc or q to quit".yellow(),
            "Select an operation".cyan()
        ))
        .default(0)
        .report(false)
        .items(&MENU_OPERATIONS)
        .interact_opt();

    match menu_operation {
        Ok(Some(idx)) => InputResult::Result(Ok(MENU_OPERATIONS[idx])),
        Err(e) => InputResult::Result(Err(e)),
        Ok(None) => InputResult::Quit,
    }
}

pub fn select_todo(todos: &Vec<Todo>, default_selected_todo: usize) -> InputResult<usize> {
    let selected_todo_index = Select::new()
        .with_prompt(format!(
            "{}\n{}",
            "Press Esc or q to go to the menu".yellow(),
            "Select a todo".cyan()
        ))
        .default(default_selected_todo)
        .report(false)
        .items(todos)
        .max_length(5)
        .interact_opt();

    match selected_todo_index {
        Ok(Some(idx)) => InputResult::Result(Ok(idx)),
        Err(e) => InputResult::Result(Err(e)),
        Ok(None) => InputResult::Quit,
    }
}

pub fn select_todo_operation() -> InputResult<&'static TodoOperation> {
    let todo_operation = Select::new()
        .with_prompt(format!(
            "{}\n{}",
            "Press Esc or q to go to the menu".yellow(),
            "Select an operation".cyan()
        ))
        .default(0)
        .report(false)
        .items(&TODO_OPERATIONS)
        .interact_opt();

    match todo_operation {
        Ok(Some(idx)) => InputResult::Result(Ok(TODO_OPERATIONS[idx])),
        Err(e) => InputResult::Result(Err(e)),
        Ok(None) => InputResult::Quit,
    }
}

pub fn get_text_input(prompt: &str, initial_text: &str) -> InputResult<String> {
    let input = Input::<String>::new()
        .with_prompt(prompt)
        .report(false)
        .with_initial_text(initial_text)
        .interact_text();

    match input {
        Ok(v) => InputResult::Result(Ok(v)),
        Err(e) => InputResult::Result(Err(e)),
    }
}
