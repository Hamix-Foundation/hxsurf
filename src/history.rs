use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryItem {
    pub url: String,
    pub title: String,
}

impl HistoryItem {
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HistoryStore {
    back_stack: Vec<HistoryItem>,
    forward_stack: Vec<HistoryItem>,
    current: Option<HistoryItem>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self {
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
            current: None,
        }
    }

    pub fn current(&self) -> Option<&HistoryItem> {
        self.current.as_ref()
    }

    pub fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    pub fn navigate(&mut self, url: impl Into<String>, title: impl Into<String>) {
        let item = HistoryItem::new(url, title);
        if let Some(prev) = self.current.take() {
            self.back_stack.push(prev);
        }
        self.current = Some(item);
        self.forward_stack.clear();
    }

    pub fn go_back(&mut self) -> Option<&HistoryItem> {
        if let Some(prev) = self.back_stack.pop() {
            if let Some(curr) = self.current.take() {
                self.forward_stack.push(curr);
            }
            self.current = Some(prev);
            self.current.as_ref()
        } else {
            None
        }
    }

    pub fn go_forward(&mut self) -> Option<&HistoryItem> {
        if let Some(next) = self.forward_stack.pop() {
            if let Some(curr) = self.current.take() {
                self.back_stack.push(curr);
            }
            self.current = Some(next);
            self.current.as_ref()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.back_stack.clear();
        self.forward_stack.clear();
    }
}
