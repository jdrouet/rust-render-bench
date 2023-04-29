pub mod concat;
pub mod sailfish;
pub mod yew;

#[derive(Clone, Debug, PartialEq)]
pub struct TodoItem {
    pub content: String,
    pub done: bool,
}
