use crate::TodoItem;

fn render_item(res: &mut String, todo_item: TodoItem) {
    res.push_str("<article>");
    res.push_str("<input type=\"checkbox\" checked=");
    res.push_str(if todo_item.done {
        "\"checked\""
    } else {
        "\"\""
    });
    res.push_str(" />");
    res.push_str("<label>");
    res.push_str(&todo_item.content);
    res.push_str("</label>");
    res.push_str("</article>");
}

fn render_list(res: &mut String, todo_list: Vec<TodoItem>) {
    res.push_str("<main>");
    res.push_str("<h1>");
    res.push_str("My TODO list");
    res.push_str("</h1>");
    for item in todo_list {
        render_item(res, item);
    }
    res.push_str("</main>");
}

pub fn render(todo_list: Vec<TodoItem>) -> Result<String, String> {
    let mut result = String::from("<!DOCTYPE html><html><body>");
    render_list(&mut result, todo_list);
    result.push_str("</body>");
    result.push_str("</html>");
    Ok(result)
}

pub async fn render_async(todo_list: Vec<TodoItem>) -> Result<String, String> {
    render(todo_list)
}
