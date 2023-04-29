use std::rc::Rc;
use yew::prelude::*;

use crate::TodoItem;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    item: Rc<crate::TodoItem>,
}

#[function_component(TodoItemComponent)]
fn todo_item_component(props: &TodoItemProps) -> Html {
    html! {
        <article>
            <input type="checkbox" checked={props.item.done} />
            <label>{props.item.content.as_str()}</label>
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    list: Vec<Rc<crate::TodoItem>>,
}

#[function_component(TodoListComponent)]
fn todo_list_component(props: &TodoListProps) -> Html {
    html! {
        <main>
            <h1>{ "My TODO list" }</h1>
            {
                props.list.iter().map(|item| html! {
                    <TodoItemComponent item={item} />
                }).collect::<Html>()
            }
        </main>
    }
}

pub async fn render_multi(todo_list: Vec<TodoItem>) -> Result<String, String> {
    let result = yew::ServerRenderer::<TodoListComponent>::with_props(move || {
        let list = todo_list.into_iter().map(Rc::new).collect();
        TodoListProps { list }
    })
    .render()
    .await;
    Ok(format!(
        r#"<!DOCTYPE html><html><body>{}</body></html>"#,
        result
    ))
}

pub async fn render_local(todo_list: Vec<TodoItem>) -> Result<String, String> {
    let result = yew::LocalServerRenderer::<TodoListComponent>::with_props({
        let list = todo_list.into_iter().map(Rc::new).collect();
        TodoListProps { list }
    })
    .render()
    .await;
    Ok(format!(
        r#"<!DOCTYPE html><html><body>{}</body></html>"#,
        result
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_render() {
        let payload = vec![
            crate::TodoItem {
                done: false,
                content: "foo".to_string(),
            },
            crate::TodoItem {
                done: false,
                content: "bar".to_string(),
            },
            crate::TodoItem {
                done: true,
                content: "baz".to_string(),
            },
        ];
        let _ = render_multi(payload).await.unwrap();
    }
}
