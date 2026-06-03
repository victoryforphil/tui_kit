use super::filter::filter_items;

/// A single selectable item in the picker.
#[derive(Debug, Clone)]
pub struct PickerItem {
    /// Display label shown in the list and used for filtering.
    pub label: String,
    /// Value returned on selection. Usually a short key or identifier.
    pub key: String,
}

impl PickerItem {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
        }
    }
}

/// Result of a picker interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickerOutcome {
    Selected(String),
    Cancelled,
}

impl PickerOutcome {
    pub fn into_string_or(self, default: &str) -> String {
        match self {
            Self::Selected(k) => k,
            Self::Cancelled => default.to_owned(),
        }
    }
}

pub struct PickerState {
    pub title: String,
    pub items: Vec<PickerItem>,
    labels: Vec<String>,
    pub query: String,
    pub filtered_indices: Vec<usize>,
    pub cursor: usize,
}

impl PickerState {
    pub fn new(title: impl Into<String>, items: Vec<PickerItem>) -> Self {
        let labels: Vec<String> = items.iter().map(|i| i.label.clone()).collect();
        let filtered_indices = filter_items(&labels, "");
        Self {
            title: title.into(),
            items,
            labels,
            query: String::new(),
            filtered_indices,
            cursor: 0,
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
        self.refilter();
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
        self.refilter();
    }

    pub fn move_up(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        self.cursor = if self.cursor == 0 {
            self.filtered_indices.len() - 1
        } else {
            self.cursor - 1
        };
    }

    pub fn move_down(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        self.cursor = (self.cursor + 1) % self.filtered_indices.len();
    }

    pub fn selected_key(&self) -> Option<&str> {
        let &idx = self.filtered_indices.get(self.cursor)?;
        Some(&self.items[idx].key)
    }

    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }

    fn refilter(&mut self) {
        self.filtered_indices = filter_items(&self.labels, &self.query);
        if self.cursor >= self.filtered_indices.len() {
            self.cursor = self.filtered_indices.len().saturating_sub(1);
        }
    }
}
