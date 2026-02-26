#[derive(Clone, PartialEq, Default)]
pub enum SortBy {
    #[default]
    Name,
    Size,
    Type,
    Date,
}

#[derive(Clone, PartialEq, Default)]
pub enum ViewMode {
    #[default]
    List,
    Grid,
    Compact,
}

#[derive(Clone, PartialEq, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}
