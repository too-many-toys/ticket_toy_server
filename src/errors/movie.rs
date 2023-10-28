pub enum MovieAPIError<'a> {
    Input(&'a str, Option<&'a str>),
    API(String),
}
