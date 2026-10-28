#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

impl Bookmark {
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookmarkStore {
    items: Vec<Bookmark>,
}

impl Default for BookmarkStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BookmarkStore {
    pub fn new() -> Self {
        let items = vec![
            Bookmark::new("HamixOS", "about:hamix"),
            Bookmark::new("Documentation", "file:///usr/share/doc/hamix/index.html"),
            Bookmark::new("NetSurf Engine", "https://www.netsurf-browser.org/"),
            Bookmark::new("Search", "https://html.duckduckgo.com/html/"),
        ];
        Self { items }
    }

    pub fn list(&self) -> &[Bookmark] {
        &self.items
    }

    pub fn add(&mut self, title: impl Into<String>, url: impl Into<String>) {
        let b = Bookmark::new(title, url);
        if !self.items.iter().any(|item| item.url == b.url) {
            self.items.push(b);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Bookmark> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn find(&self, query: &str) -> Vec<&Bookmark> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|b| b.title.to_lowercase().contains(&q) || b.url.to_lowercase().contains(&q))
            .collect()
    }
}
