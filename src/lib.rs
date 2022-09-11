pub mod analyze;
pub mod args;
pub mod table_of_contents;
pub mod transform;
pub mod write;

#[derive(Debug)]
pub struct AppState<'a> {
    pub start_replace_token: &'a str,
    pub end_replace_token: &'a str,
    pub start_tag_index: i32,
    pub end_tag_index: i32,
}
