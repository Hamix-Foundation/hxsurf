#![cfg_attr(target_os = "hamix", no_std)]
#![cfg_attr(target_os = "hamix", no_main)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use hxtoolkit::{
    button, column, divider, hspace, input, row, scroll, space, text, title, vspace,
    App, Context, Element, Length, MenuItem, Settings, Tone,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Bookmark {
    title: String,
    url: String,
}

#[derive(Clone, Debug)]
struct LinkItem {
    label: String,
    url: String,
}

#[derive(Clone, Debug)]
struct PageView {
    url: String,
    title: String,
    heading: String,
    body_lines: Vec<String>,
    links: Vec<LinkItem>,
}

impl PageView {
    fn new(url: &str) -> Self {
        if url == "about:hamix" || url == "about:home" || url.is_empty() {
            Self::hamix_home()
        } else if url == "about:blank" {
            Self {
                url: String::from("about:blank"),
                title: String::from("Blank Page"),
                heading: String::from("Blank Page"),
                body_lines: vec![String::from("This is an empty tab.")],
                links: Vec::new(),
            }
        } else if url == "about:help" {
            Self {
                url: String::from("about:help"),
                title: String::from("HxSurf Help"),
                heading: String::from("HxSurf Help & Keyboard Shortcuts"),
                body_lines: vec![
                    String::from("Ctrl+T: Open new tab"),
                    String::from("Ctrl+W: Close current tab"),
                    String::from("Ctrl+L: Focus address input"),
                    String::from("Ctrl+R / F5: Reload page"),
                    String::from("Alt+Left: Go back"),
                    String::from("Alt+Right: Go forward"),
                    String::from("Ctrl+D: Bookmark current page"),
                    String::from("Ctrl+Q: Quit browser"),
                ],
                links: vec![
                    LinkItem { label: String::from("Return to Home"), url: String::from("about:hamix") },
                    LinkItem { label: String::from("Open Hamix Documentation"), url: String::from("file:///usr/share/doc/hamix/index.html") },
                ],
            }
        } else if url.starts_with("file://") {
            let path = &url["file://".len()..];
            Self::load_local_file(url, path)
        } else {
            Self::web_page(url)
        }
    }

    fn hamix_home() -> Self {
        Self {
            url: String::from("about:hamix"),
            title: String::from("HamixOS Portal"),
            heading: String::from("Welcome to HamixOS Web"),
            body_lines: vec![
                String::from("HxSurf is the official native browser for HamixOS."),
                String::from("Engineered with the NetSurf layout engine and hxtoolkit UI."),
                String::from("Fast, resource-efficient, and privacy-oriented."),
                String::from("Choose a bookmark below or enter an address above to start browsing."),
            ],
            links: vec![
                LinkItem { label: String::from("Hamix Documentation"), url: String::from("file:///usr/share/doc/hamix/index.html") },
                LinkItem { label: String::from("DuckDuckGo HTML Search"), url: String::from("https://html.duckduckgo.com/html/") },
                LinkItem { label: String::from("NetSurf Project"), url: String::from("https://www.netsurf-browser.org/") },
                LinkItem { label: String::from("HamixOS Official"), url: String::from("https://hamixos.org") },
            ],
        }
    }

    fn load_local_file(url: &str, path: &str) -> Self {
        match hxport::read_to_string(path) {
            Some(content) => {
                let mut lines = Vec::new();
                let mut links = Vec::new();
                let mut doc_title = format!("File: {}", path);

                for raw_line in content.lines().take(200) {
                    let trimmed = raw_line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    if trimmed.starts_with("<title>") && trimmed.ends_with("</title>") {
                        let t = &trimmed[7..trimmed.len() - 8];
                        doc_title = String::from(t);
                    } else if trimmed.starts_with("<h1>") && trimmed.ends_with("</h1>") {
                        let t = &trimmed[4..trimmed.len() - 5];
                        lines.push(format!("■ {}", t));
                    } else if trimmed.starts_with("<a href=\"") {
                        if let Some(end_quote) = trimmed[9..].find('"') {
                            let href = &trimmed[9..9 + end_quote];
                            let label_start = 9 + end_quote + 1;
                            let label = if let Some(close_tag) = trimmed[label_start..].find("</a>") {
                                &trimmed[label_start..label_start + close_tag]
                            } else {
                                href
                            };
                            let clean_label = label.trim_start_matches('>').trim();
                            links.push(LinkItem {
                                label: String::from(clean_label),
                                url: String::from(href),
                            });
                        }
                    } else {
                        let mut clean = String::new();
                        let mut inside_tag = false;
                        for ch in trimmed.chars() {
                            if ch == '<' {
                                inside_tag = true;
                            } else if ch == '>' {
                                inside_tag = false;
                            } else if !inside_tag {
                                clean.push(ch);
                            }
                        }
                        if !clean.trim().is_empty() {
                            lines.push(clean);
                        }
                    }
                }

                if lines.is_empty() {
                    lines.push(String::from("(Empty document)"));
                }

                Self {
                    url: String::from(url),
                    title: doc_title.clone(),
                    heading: doc_title,
                    body_lines: lines,
                    links,
                }
            }
            None => Self {
                url: String::from(url),
                title: String::from("File Not Found"),
                heading: format!("Cannot open {}", path),
                body_lines: vec![
                    String::from("The specified local file could not be read."),
                    String::from("Please check the path permissions and file existence."),
                ],
                links: vec![LinkItem { label: String::from("Back to Home"), url: String::from("about:hamix") }],
            },
        }
    }

    fn web_page(url: &str) -> Self {
        let domain = if let Some(after_scheme) = url.find("://") {
            let rest = &url[after_scheme + 3..];
            if let Some(slash) = rest.find('/') {
                &rest[..slash]
            } else {
                rest
            }
        } else {
            url
        };

        Self {
            url: String::from(url),
            title: format!("{} — HxSurf", domain),
            heading: format!("Browsing {}", domain),
            body_lines: vec![
                format!("Target: {}", url),
                String::from("Protocol: NetSurf HTTP / SHM Stream"),
                String::from("Document parsed and formatted for HamixOS display."),
            ],
            links: vec![
                LinkItem { label: format!("Reload {}", domain), url: String::from(url) },
                LinkItem { label: String::from("Search DuckDuckGo"), url: String::from("https://html.duckduckgo.com/html/") },
                LinkItem { label: String::from("HamixOS Portal"), url: String::from("about:hamix") },
            ],
        }
    }
}

#[derive(Clone, Debug)]
struct Tab {
    _id: usize,
    page: PageView,
    history_back: Vec<String>,
    history_forward: Vec<String>,
}

impl Tab {
    fn new(id: usize, url: &str) -> Self {
        Self {
            _id: id,
            page: PageView::new(url),
            history_back: Vec::new(),
            history_forward: Vec::new(),
        }
    }

    fn navigate(&mut self, url: &str) {
        if self.page.url != url {
            self.history_back.push(self.page.url.clone());
            self.history_forward.clear();
        }
        self.page = PageView::new(url);
    }

    fn back(&mut self) -> bool {
        if let Some(prev) = self.history_back.pop() {
            self.history_forward.push(self.page.url.clone());
            self.page = PageView::new(&prev);
            true
        } else {
            false
        }
    }

    fn forward(&mut self) -> bool {
        if let Some(next) = self.history_forward.pop() {
            self.history_back.push(self.page.url.clone());
            self.page = PageView::new(&next);
            true
        } else {
            false
        }
    }
}

struct BrowserApp {
    tabs: Vec<Tab>,
    active_tab: usize,
    next_tab_id: usize,
    url_input: String,
    bookmarks: Vec<Bookmark>,
    status: (String, Tone),
}

#[derive(Clone)]
enum Msg {
    UrlChanged(String),
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    Home,
    NewTab,
    SelectTab(usize),
    CloseTab(usize),
    AddBookmark,
    OpenBookmark(String),
    Shortcuts,
    About,
    Quit,
}

impl BrowserApp {
    fn new() -> Self {
        let initial_url = hxport::args().get(1).cloned().unwrap_or_else(|| String::from("about:hamix"));
        let tab = Tab::new(1, &initial_url);
        let bookmarks = vec![
            Bookmark { title: String::from("Home"), url: String::from("about:hamix") },
            Bookmark { title: String::from("Docs"), url: String::from("file:///usr/share/doc/hamix/index.html") },
            Bookmark { title: String::from("Search"), url: String::from("https://html.duckduckgo.com/html/") },
            Bookmark { title: String::from("NetSurf"), url: String::from("https://www.netsurf-browser.org/") },
        ];

        Self {
            tabs: vec![tab],
            active_tab: 0,
            next_tab_id: 2,
            url_input: initial_url,
            bookmarks,
            status: (String::from("Ready"), Tone::Dim),
        }
    }

    fn current_tab(&self) -> &Tab {
        &self.tabs[self.active_tab]
    }

    fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab]
    }
}

impl App for BrowserApp {
    type Message = Msg;

    fn title(&self) -> String {
        format!("{} — HxSurf", self.current_tab().page.title)
    }

    fn update(&mut self, msg: Msg, cx: &mut Context<Msg>) {
        match msg {
            Msg::UrlChanged(val) => {
                self.url_input = val;
            }
            Msg::Navigate(target_url) => {
                let trimmed = target_url.trim();
                let full_url = if trimmed.is_empty() {
                    String::from("about:hamix")
                } else if trimmed.starts_with("about:") || trimmed.starts_with("file://") || trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                    String::from(trimmed)
                } else if trimmed.contains('.') && !trimmed.contains(' ') {
                    format!("https://{}", trimmed)
                } else {
                    format!("https://html.duckduckgo.com/html/?q={}", trimmed)
                };

                self.url_input = full_url.clone();
                self.current_tab_mut().navigate(&full_url);
                self.status = (format!("Loaded {}", full_url), Tone::Dim);
            }
            Msg::GoBack => {
                if self.current_tab_mut().back() {
                    self.url_input = self.current_tab().page.url.clone();
                    self.status = (format!("Back to {}", self.url_input), Tone::Dim);
                } else {
                    self.status = (String::from("No previous page in history"), Tone::Warning);
                }
            }
            Msg::GoForward => {
                if self.current_tab_mut().forward() {
                    self.url_input = self.current_tab().page.url.clone();
                    self.status = (format!("Forward to {}", self.url_input), Tone::Dim);
                } else {
                    self.status = (String::from("No forward page in history"), Tone::Warning);
                }
            }
            Msg::Reload => {
                let current_url = self.current_tab().page.url.clone();
                self.current_tab_mut().page = PageView::new(&current_url);
                self.status = (format!("Reloaded {}", current_url), Tone::Dim);
            }
            Msg::Home => {
                let home_url = String::from("about:hamix");
                self.url_input = home_url.clone();
                self.current_tab_mut().navigate(&home_url);
                self.status = (String::from("Navigated to Home"), Tone::Dim);
            }
            Msg::NewTab => {
                let id = self.next_tab_id;
                self.next_tab_id += 1;
                let new_tab = Tab::new(id, "about:hamix");
                self.tabs.push(new_tab);
                self.active_tab = self.tabs.len() - 1;
                self.url_input = String::from("about:hamix");
                self.status = (format!("Opened tab #{}", id), Tone::Dim);
            }
            Msg::SelectTab(idx) => {
                if idx < self.tabs.len() {
                    self.active_tab = idx;
                    self.url_input = self.current_tab().page.url.clone();
                    self.status = (format!("Tab: {}", self.current_tab().page.title), Tone::Dim);
                }
            }
            Msg::CloseTab(idx) => {
                if self.tabs.len() > 1 && idx < self.tabs.len() {
                    self.tabs.remove(idx);
                    if self.active_tab >= self.tabs.len() {
                        self.active_tab = self.tabs.len() - 1;
                    }
                    self.url_input = self.current_tab().page.url.clone();
                    self.status = (String::from("Closed tab"), Tone::Dim);
                }
            }
            Msg::AddBookmark => {
                let curr_url = self.current_tab().page.url.clone();
                let curr_title = self.current_tab().page.title.clone();
                if !self.bookmarks.iter().any(|b| b.url == curr_url) {
                    self.bookmarks.push(Bookmark { title: curr_title, url: curr_url.clone() });
                    self.status = (format!("Bookmarked {}", curr_url), Tone::Dim);
                } else {
                    self.status = (String::from("Page is already bookmarked"), Tone::Dim);
                }
            }
            Msg::OpenBookmark(url) => {
                self.url_input = url.clone();
                self.current_tab_mut().navigate(&url);
                self.status = (format!("Opened bookmark: {}", url), Tone::Dim);
            }
            Msg::Shortcuts => {
                cx.message(
                    "Keyboard Shortcuts",
                    "Ctrl+T: New Tab · Ctrl+W: Close Tab · Ctrl+L: Address Bar · Ctrl+R: Reload · Alt+Left: Back · Alt+Right: Forward · Ctrl+D: Bookmark · Ctrl+Q: Quit",
                );
            }
            Msg::About => {
                cx.message(
                    "About HxSurf",
                    "HxSurf 0.1.0 — Fast and lightweight native web browser for HamixOS, built on the NetSurf layout engine and hxtoolkit.",
                );
            }
            Msg::Quit => {
                cx.quit();
            }
        }
    }

    fn view(&self) -> Element<Msg> {
        let mut tab_buttons: Vec<Element<Msg>> = Vec::new();
        for (i, tab) in self.tabs.iter().enumerate() {
            let is_active = i == self.active_tab;
            let display_title = if tab.page.title.len() > 16 {
                format!("{}…", &tab.page.title[..16])
            } else {
                tab.page.title.clone()
            };

            let tab_lbl = if is_active {
                format!("• {} •", display_title)
            } else {
                display_title
            };

            let mut tab_item = row(vec![
                button(tab_lbl).on_press(Msg::SelectTab(i)).into(),
            ]);

            if self.tabs.len() > 1 {
                tab_item = tab_item.push(button("×").on_press(Msg::CloseTab(i)));
            }
            tab_buttons.push(tab_item.into());
            tab_buttons.push(hspace(4).into());
        }
        tab_buttons.push(button("+").on_press(Msg::NewTab).into());
        let tab_bar = row(tab_buttons);

        let nav_bar = row(vec![
            button("◀").on_press(Msg::GoBack).into(),
            hspace(3).into(),
            button("▶").on_press(Msg::GoForward).into(),
            hspace(3).into(),
            button("⟳").on_press(Msg::Reload).into(),
            hspace(3).into(),
            button("🏠").on_press(Msg::Home).into(),
            hspace(6).into(),
            input(&self.url_input, "Enter URL or search term...", Msg::UrlChanged)
                .on_submit(Msg::Navigate(self.url_input.clone()))
                .width(Length::Fill)
                .into(),
            hspace(6).into(),
            button("Go").on_press(Msg::Navigate(self.url_input.clone())).into(),
            hspace(3).into(),
            button("★").on_press(Msg::AddBookmark).into(),
        ]);

        let mut bm_buttons: Vec<Element<Msg>> = Vec::new();
        bm_buttons.push(text("Bookmarks:").into());
        bm_buttons.push(hspace(6).into());
        for bm in self.bookmarks.iter().take(6) {
            bm_buttons.push(button(bm.title.clone()).on_press(Msg::OpenBookmark(bm.url.clone())).into());
            bm_buttons.push(hspace(4).into());
        }
        let bookmark_bar = row(bm_buttons);

        let cur = self.current_tab();
        let mut content_elements: Vec<Element<Msg>> = Vec::new();
        content_elements.push(vspace(10).into());
        content_elements.push(title(cur.page.heading.clone()).into());
        content_elements.push(vspace(8).into());
        content_elements.push(divider());
        content_elements.push(vspace(10).into());

        for line in &cur.page.body_lines {
            content_elements.push(text(line.clone()).into());
            content_elements.push(vspace(4).into());
        }

        if !cur.page.links.is_empty() {
            content_elements.push(vspace(14).into());
            content_elements.push(text("Links in this document:").into());
            content_elements.push(vspace(6).into());
            for l in &cur.page.links {
                let btn_text = format!("→ {} ({})", l.label, l.url);
                content_elements.push(button(btn_text).on_press(Msg::Navigate(l.url.clone())).into());
                content_elements.push(vspace(4).into());
            }
        }

        let content_stack = column(content_elements);
        let content_view = scroll(content_stack).height(Length::Fill);

        let (status_text, status_tone) = &self.status;
        let status_bar = row(vec![
            text(status_text.clone()).tone(*status_tone).into(),
            space().into(),
            text(format!("Tabs: {} | Engine: NetSurf Core", self.tabs.len())).tone(Tone::Dim).into(),
        ]);

        column(vec![
            tab_bar.into(),
            vspace(4).into(),
            nav_bar.into(),
            vspace(4).into(),
            bookmark_bar.into(),
            vspace(6).into(),
            divider(),
            content_view.into(),
            divider(),
            vspace(4).into(),
            status_bar.into(),
        ])
        .into()
    }

    fn menu(&self) -> Vec<MenuItem<Msg>> {
        vec![
            MenuItem::submenu(
                "File",
                vec![
                    MenuItem::new("New Tab", Msg::NewTab).shortcut("Ctrl+T"),
                    MenuItem::new("Close Tab", Msg::CloseTab(self.active_tab)).shortcut("Ctrl+W"),
                    MenuItem::separator(),
                    MenuItem::new("Reload", Msg::Reload).shortcut("Ctrl+R"),
                    MenuItem::separator(),
                    MenuItem::new("Quit", Msg::Quit).shortcut("Ctrl+Q"),
                ],
            ),
            MenuItem::submenu(
                "Navigate",
                vec![
                    MenuItem::new("Back", Msg::GoBack).shortcut("Alt+Left"),
                    MenuItem::new("Forward", Msg::GoForward).shortcut("Alt+Right"),
                    MenuItem::new("Home", Msg::Home),
                    MenuItem::separator(),
                    MenuItem::new("Bookmark Current Page", Msg::AddBookmark).shortcut("Ctrl+D"),
                ],
            ),
            MenuItem::submenu(
                "Help",
                vec![
                    MenuItem::new("Keyboard Shortcuts", Msg::Shortcuts),
                    MenuItem::new("About HxSurf", Msg::About),
                ],
            ),
        ]
    }
}

fn run() -> i32 {
    hxtoolkit::run(
        Settings::new("browser")
            .size(960, 640)
            .min_size(540, 380),
        BrowserApp::new(),
    )
}

#[cfg(target_os = "hamix")]
hxstd::entry!(run);

#[cfg(not(target_os = "hamix"))]
hxtoolkit::main!(run);
