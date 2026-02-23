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

impl ViewMode {
    fn default() -> Self {
        ViewMode::List
    }
}

impl SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
    pub fn toggle(&mut self) {
        *self = match self {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        }
    }
}
