use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

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

    pub fn load(path: &str) -> Self {
        let Some(contents) = hxport::read_to_string(path) else {
            return Self::new();
        };
        let items = contents.lines().filter_map(|line| {
            let (title, url) = line.split_once('\t')?;
            if url.is_empty() { None } else { Some(Bookmark::new(title, url)) }
        }).collect();
        Self { items }
    }

    pub fn save(&self, path: &str) {
        let mut contents = String::new();
        for item in &self.items {
            let title = item.title.replace('\t', " ").replace('\n', " ").replace('\r', " ");
            let url = item.url.replace('\t', "").replace('\n', "").replace('\r', "");
            contents.push_str(&title);
            contents.push('\t');
            contents.push_str(&url);
            contents.push('\n');
        }
        if let Some(parent) = path.rsplit_once('/') {
            if let Some(grandparent) = parent.0.rsplit_once('/') {
                let _ = hxport::mkdir(grandparent.0);
            }
            let _ = hxport::mkdir(parent.0);
        }
        let _ = hxport::write(path, contents.as_bytes());
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
