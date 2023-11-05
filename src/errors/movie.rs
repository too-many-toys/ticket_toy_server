pub enum MovieApiError {
    Input(String, Option<String>),
    API(String),
}
