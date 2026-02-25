#[derive(Clone, Copy, PartialEq, Default)]
pub enum SortBy {
    #[default]
    Name,
    Size,
    Type,
    Date,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ViewMode {
    #[default]
    List,
    Grid,
    Compact,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}
