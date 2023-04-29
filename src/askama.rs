use crate::TodoItem;
use askama::Template;

#[derive(Template)]
#[template(path = "page.html")]
struct Page {
    list: Vec<TodoItem>,
}

pub fn render(todo_list: Vec<TodoItem>) -> Result<String, String> {
    Page { list: todo_list }.render().map_err(|err| err.to_string())
}

pub async fn render_async(todo_list: Vec<TodoItem>) -> Result<String, String> {
    render(todo_list)
}
