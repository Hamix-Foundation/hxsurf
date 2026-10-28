#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub text: String,
    pub target: String,
}

impl Link {
    pub fn new(text: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            target: target.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageState {
    Empty,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub state: PageState,
    pub content: String,
    pub links: Vec<Link>,
    pub status: String,
}

impl Default for Page {
    fn default() -> Self {
        Self::new("about:blank")
    }
}

impl Page {
    pub fn new(url: impl Into<String>) -> Self {
        let u = url.into();
        let title = if u.starts_with("about:") {
            u.clone()
        } else {
            String::from("New Page")
        };
        Self {
            url: u,
            title,
            state: PageState::Empty,
            content: String::new(),
            links: Vec::new(),
            status: String::from("Ready"),
        }
    }

    pub fn set_content(&mut self, title: impl Into<String>, content: impl Into<String>, links: Vec<Link>) {
        self.title = title.into();
        self.content = content.into();
        self.links = links;
        self.state = PageState::Loaded;
        self.status = String::from("Loaded");
    }

    pub fn set_error(&mut self, err: impl Into<String>) {
        let message = err.into();
        self.title = String::from("Error Loading Page");
        self.content = message.clone();
        self.state = PageState::Error;
        self.status = message;
    }
}
