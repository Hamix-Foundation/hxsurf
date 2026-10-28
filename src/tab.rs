use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use crate::history::HistoryStore;
use crate::page::Page;

#[derive(Clone, Debug)]
pub struct Tab {
    pub id: usize,
    pub page: Page,
    pub history: HistoryStore,
}

impl Tab {
    pub fn new(id: usize, initial_url: impl Into<String>) -> Self {
        let url = initial_url.into();
        let page = Page::new(&url);
        let mut history = HistoryStore::new();
        history.navigate(&url, &page.title);
        Self { id, page, history }
    }

    pub fn url(&self) -> &str {
        &self.page.url
    }

    pub fn title(&self) -> &str {
        &self.page.title
    }

    pub fn navigate(&mut self, url: impl Into<String>) {
        let u = url.into();
        self.page = Page::new(&u);
        self.history.navigate(&u, &self.page.title);
    }

    pub fn back(&mut self) -> bool {
        if let Some(item) = self.history.go_back().cloned() {
            self.page = Page::new(&item.url);
            true
        } else {
            false
        }
    }

    pub fn forward(&mut self) -> bool {
        if let Some(item) = self.history.go_forward().cloned() {
            self.page = Page::new(&item.url);
            true
        } else {
            false
        }
    }

    pub fn reload(&mut self) {
        let current_url = self.page.url.clone();
        self.page = Page::new(&current_url);
    }
}

#[derive(Clone, Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_index: usize,
    next_id: usize,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new("about:hamix")
    }
}

impl TabManager {
    pub fn new(initial_url: impl Into<String>) -> Self {
        let tab = Tab::new(1, initial_url);
        Self {
            tabs: vec![tab],
            active_index: 0,
            next_id: 2,
        }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.active_index]
    }

    pub fn active_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_index]
    }

    pub fn open_tab(&mut self, url: impl Into<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let tab = Tab::new(id, url);
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        self.active_index
    }

    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 {
            return false;
        }
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_index >= self.tabs.len() {
                self.active_index = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_index = index;
        }
    }
}
