#![cfg_attr(target_os = "hamix", no_std)]
#![cfg_attr(target_os = "hamix", no_main)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use hxtoolkit::{
    button, column, container, divider, heading, hspace, icon_button, input, link, row, scroll,
    subheading, text, vspace, App, Context, Element, Key, KeyPress, Length, MenuItem, Padding,
    Settings, TextSize, Tone,
};
use hxsurf::bookmarks::BookmarkStore;
use hxsurf::page::PageNode;
use hxsurf::tab::TabManager;

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
    ToggleSettings,
    RemoveBookmark(usize),
    HomeChanged(String),
    SaveHome,
    FocusAddress,
    OpenBookmark(String),
    Shortcuts,
    About,
    Quit,
}

struct BrowserApp {
    tabs: TabManager,
    bookmarks: BookmarkStore,
    url_input: String,
    status: (String, Tone),
    dark: bool,
    address_focused: bool,
    settings_open: bool,
    homepage: String,
}

impl BrowserApp {
    fn new() -> Self {
        let default_home = hxport::read_to_string(&format!("{}/.config/hxsurf/homepage", hxport::home()))
            .filter(|value| !value.trim().is_empty()).unwrap_or_else(|| String::from("about:hamix"));
        let initial_url = hxport::args().get(1).cloned().unwrap_or(default_home);
        let tabs = TabManager::new(&initial_url);
        let bookmarks = BookmarkStore::load(&format!("{}/.config/hxsurf/bookmarks", hxport::home()));
        let homepage = hxport::read_to_string(&format!("{}/.config/hxsurf/homepage", hxport::home()))
            .filter(|value| !value.trim().is_empty()).unwrap_or_else(|| String::from("about:hamix"));
        let status = (tabs.active_tab().page.status.clone(), Tone::Dim);

        Self {
            tabs,
            bookmarks,
            url_input: initial_url,
            status,
            dark: false,
            address_focused: false,
            settings_open: false,
            homepage,
        }
    }
}

fn home_tile(title: &str, detail: &str, url: &str, icon_name: &str, border: u32) -> Element<Msg> {
    container(column(vec![
        button(title)
            .flat()
            .size(TextSize::Title)
            .icon(icon_name)
            .on_press(Msg::Navigate(String::from(url)))
            .into(),
        text(detail).small().dim().wrap().into(),
    ]))
    .card()
    .border(border, 1)
    .radius(10)
    .padding(14)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
}

impl App for BrowserApp {
    type Message = Msg;

    fn title(&self) -> String {
        format!("{} — Browser", self.tabs.active_tab().title())
    }

    fn update(&mut self, msg: Msg, cx: &mut Context<Msg>) {
        match msg {
            Msg::UrlChanged(val) => {
                self.url_input = val;
                self.address_focused = false;
            }
            Msg::Navigate(target_url) => {
                self.tabs.active_tab_mut().navigate(&target_url);
                cx.reset_scroll();
                self.url_input = self.tabs.active_tab().url().to_string();
                self.address_focused = false;
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::GoBack => {
                if self.tabs.active_tab_mut().back() {
                    cx.reset_scroll();
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
                } else {
                    self.status = (String::from("No previous page in history"), Tone::Warning);
                }
            }
            Msg::GoForward => {
                if self.tabs.active_tab_mut().forward() {
                    cx.reset_scroll();
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
                } else {
                    self.status = (String::from("No forward page in history"), Tone::Warning);
                }
            }
            Msg::Reload => {
                self.tabs.active_tab_mut().reload();
                cx.reset_scroll();
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::Home => {
                self.tabs.active_tab_mut().navigate(&self.homepage);
                cx.reset_scroll();
                self.url_input = self.homepage.clone();
                self.status = (String::from("Ready"), Tone::Dim);
            }
            Msg::NewTab => {
                let idx = self.tabs.open_tab("about:hamix");
                self.url_input = String::from("about:hamix");
                self.status = (format!("Opened tab #{}", idx + 1), Tone::Dim);
            }
            Msg::SelectTab(idx) => {
                self.tabs.select_tab(idx);
                cx.reset_scroll();
                self.url_input = self.tabs.active_tab().url().to_string();
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::CloseTab(idx) => {
                if self.tabs.close_tab(idx) {
                    cx.reset_scroll();
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (String::from("Closed tab"), Tone::Dim);
                }
            }
            Msg::AddBookmark => {
                let url = self.tabs.active_tab().url().to_string();
                let title = self.tabs.active_tab().title().to_string();
                self.bookmarks.add(title, url);
                self.bookmarks.save(&format!("{}/.config/hxsurf/bookmarks", hxport::home()));
                self.status = (String::from("Page bookmarked"), Tone::Success);
            }
            Msg::ToggleSettings => self.settings_open = !self.settings_open,
            Msg::RemoveBookmark(index) => {
                self.bookmarks.remove(index);
                self.bookmarks.save(&format!("{}/.config/hxsurf/bookmarks", hxport::home()));
            }
            Msg::HomeChanged(value) => self.homepage = value,
            Msg::SaveHome => {
                let path = format!("{}/.config/hxsurf/homepage", hxport::home());
                if let Some(parent) = path.rsplit_once('/') {
                    if let Some(grandparent) = parent.0.rsplit_once('/') { let _ = hxport::mkdir(grandparent.0); }
                    let _ = hxport::mkdir(parent.0);
                }
                let _ = hxport::write(&path, self.homepage.as_bytes());
                self.settings_open = false;
            }
            Msg::FocusAddress => {
                self.address_focused = true;
            }
            Msg::OpenBookmark(url) => {
                self.tabs.active_tab_mut().navigate(&url);
                cx.reset_scroll();
                self.url_input = url;
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::Shortcuts => {
                cx.message(
                    "Keyboard Shortcuts",
                    "Ctrl+T: New Tab · Ctrl+W: Close Tab · Ctrl+L: Address Bar · Ctrl+R / F5: Reload · Alt+Left: Back · Alt+Right: Forward · Ctrl+D: Bookmark · Ctrl+Q: Quit",
                );
            }
            Msg::About => {
                cx.message(
                    "About Browser",
                    "Browser 0.1.0\nWeb browser for HamixOS.",
                );
            }
            Msg::Quit => {
                cx.quit();
            }
        }
    }

    fn appearance(&mut self, dark: bool) {
        self.dark = dark;
    }

    fn view(&self) -> Element<Msg> {
        let (chrome_bg, chrome_border, tab_active, tab_idle, chrome_text, chrome_muted) = if self.dark {
            (0x1c212b, 0x394150, 0x303846, 0x242a35, 0xe5e9f0, 0xa7b0bf)
        } else {
            (0xf1f3f6, 0xd4d9e1, 0xffffff, 0xe5e9ef, 0x273244, 0x697586)
        };
        let active_tab_index = self.tabs.active_index();
        let mut tab_buttons: Vec<Element<Msg>> = Vec::new();
        for (i, tab) in self.tabs.tabs().iter().enumerate() {
            let is_active = i == active_tab_index;
            let display_title = if tab.title().len() > 18 {
                let mut s: String = tab.title().chars().take(14).collect();
                s.push('…');
                s
            } else {
                tab.title().to_string()
            };

            if is_active {
                let active_item = container(row(vec![
                    text(display_title).strong().color(chrome_text).into(),
                    hspace(6).into(),
                    icon_button("ui/close", "Close tab").on_press(Msg::CloseTab(i)).into(),
                ]))
                .bg(tab_active)
                .border(chrome_border, 1)
                .radius(6)
                .padding(Padding::symmetric(3, 10))
                .width(Length::Shrink)
                .height(Length::Shrink);
                tab_buttons.push(active_item.into());
            } else {
                let mut inactive_row = vec![
                    button(display_title).flat().size(TextSize::Small).on_press(Msg::SelectTab(i)).into(),
                ];
                if self.tabs.tabs().len() > 1 {
                    inactive_row.push(hspace(2).into());
                    inactive_row.push(icon_button("ui/close", "Close tab").on_press(Msg::CloseTab(i)).into());
                }
                let inactive_item = container(row(inactive_row))
                .bg(tab_idle)
                .radius(6)
                .padding(Padding::symmetric(3, 8))
                .width(Length::Shrink)
                .height(Length::Shrink);
                tab_buttons.push(inactive_item.into());
            }
            tab_buttons.push(hspace(4).into());
        }
        tab_buttons.push(button("+").flat().size(TextSize::Small).on_press(Msg::NewTab).into());
        let tab_bar = row(tab_buttons);

        let active_tab = self.tabs.active_tab();
        let document_dark = self.dark && active_tab.url().starts_with("about:");
        let (document_bg, document_text, document_heading, document_muted, code_bg, code_border) = if document_dark {
            (0x1b1e25, 0xe8eaf0, 0xf2f4f8, 0xa0a7b4, 0x23272f, 0x363c48)
        } else {
            (0xffffff, 0x1e293b, 0x0f172a, 0x64748b, 0xf8fafc, 0xe2e8f0)
        };
        let mut address_field = input(&self.url_input, "Search or enter a website address", Msg::UrlChanged)
            .on_submit(Msg::Navigate(self.url_input.clone()))
            .width(Length::Fill);
        if self.address_focused {
            address_field = address_field.focused();
        }
        let nav_bar = row(vec![
            icon_button("ui/back", "Back")
                .on_press_maybe(if active_tab.history.can_go_back() { Some(Msg::GoBack) } else { None })
                .into(),
            hspace(1).into(),
            icon_button("ui/forward", "Forward")
                .on_press_maybe(if active_tab.history.can_go_forward() { Some(Msg::GoForward) } else { None })
                .into(),
            hspace(1).into(),
            icon_button("ui/refresh", "Reload page").on_press(Msg::Reload).into(),
            hspace(1).into(),
            icon_button("ui/home", "Home").on_press(Msg::Home).into(),
            hspace(4).into(),
            address_field.into(),
            hspace(6).into(),
            button("Go").icon("ui/forward").on_press(Msg::Navigate(self.url_input.clone())).into(),
            hspace(4).into(),
            icon_button("ui/save", "Bookmark this page").on_press(Msg::AddBookmark).into(),
        ]);

        let mut bm_buttons: Vec<Element<Msg>> = Vec::new();
        bm_buttons.push(text("Bookmarks").small().color(chrome_muted).into());
        bm_buttons.push(hspace(6).into());
        for bm in self.bookmarks.list().iter().take(8) {
            bm_buttons.push(
                button(bm.title.clone())
                    .flat()
                    .size(TextSize::Small)
                    .on_press(Msg::OpenBookmark(bm.url.clone()))
                    .into(),
            );
            bm_buttons.push(hspace(4).into());
        }
        let bookmark_bar = row(bm_buttons);

        let mut page_items: Vec<Element<Msg>> = Vec::new();
        for node in &active_tab.page.nodes {
            match node {
                PageNode::Heading(1, h) => {
                    page_items.push(heading(h.clone()).color(document_heading).font("display").selectable().into());
                    page_items.push(vspace(10).into());
                }
                PageNode::Heading(2, h) => {
                    page_items.push(subheading(h.clone()).color(document_heading).font("title").selectable().into());
                    page_items.push(vspace(8).into());
                }
                PageNode::Heading(3, h) => {
                    page_items.push(text(h.clone()).strong().color(document_text).font("strong").selectable().into());
                    page_items.push(vspace(6).into());
                }
                PageNode::Heading(_, h) => {
                    page_items.push(text(h.clone()).strong().color(document_text).selectable().into());
                    page_items.push(vspace(4).into());
                }
                PageNode::Paragraph(p) => {
                    page_items.push(text(p.clone()).wrap().color(document_text).fill().selectable().into());
                    page_items.push(vspace(6).into());
                }
                PageNode::Link { text: l_text, url: l_url } => {
                    let target = l_url.clone();
                    page_items.push(link(l_text.clone()).on_press(Msg::Navigate(target)).into());
                    page_items.push(vspace(6).into());
                }
                PageNode::Bullet(b) => {
                    let bullet_row = row(vec![
                        text("• ").color(document_muted).into(),
                        text(b.clone()).wrap().color(document_text).fill().selectable().into(),
                    ]);
                    page_items.push(bullet_row.into());
                    page_items.push(vspace(4).into());
                }
                PageNode::Numbered(n, b) => {
                    let num_row = row(vec![
                        text(format!("{}. ", n)).color(document_muted).into(),
                        text(b.clone()).wrap().color(document_text).fill().selectable().into(),
                    ]);
                    page_items.push(num_row.into());
                    page_items.push(vspace(4).into());
                }
                PageNode::CodeBlock(c) => {
                    let code_box = container(text(c.clone()).mono().font("mono").color(document_text).selectable())
                        .bg(code_bg)
                        .border(code_border, 1)
                        .radius(6)
                        .padding(12)
                        .height(Length::Shrink);
                    page_items.push(code_box.into());
                    page_items.push(vspace(10).into());
                }
                PageNode::Quote(q) => {
                    let quote_box = container(text(q.clone()).wrap().color(document_text).fill().selectable())
                        .bg(code_bg)
                        .border(code_border, 2)
                        .radius(4)
                        .padding(12)
                        .height(Length::Shrink);
                    page_items.push(quote_box.into());
                    page_items.push(vspace(10).into());
                }
                PageNode::Notice { title, text: n_text, is_error } => {
                    let (bg_col, border_col, title_col, text_col) = if *is_error && document_dark {
                        (0x321f25, 0x71313a, 0xff9b9b, 0xffc5c5)
                    } else if *is_error {
                        (0xfef2f2, 0xfecaca, 0x991b1b, 0x7f1d1d)
                    } else if document_dark {
                        (0x202a3a, 0x354968, 0x9fc0ff, 0xd4e2ff)
                    } else {
                        (0xeff6ff, 0xbfdbfe, 0x1e40af, 0x1e3a8a)
                    };
                    let notice_box = container(column(vec![
                        subheading(title.clone()).color(title_col).into(),
                        vspace(4).into(),
                        text(n_text.clone()).wrap().color(text_col).fill().into(),
                    ]))
                    .bg(bg_col)
                    .border(border_col, 1)
                    .radius(8)
                    .padding(14)
                    .height(Length::Shrink);
                    page_items.push(notice_box.into());
                    page_items.push(vspace(12).into());
                }
                PageNode::Divider => {
                    page_items.push(divider());
                    page_items.push(vspace(10).into());
                }
            }
        }

        if page_items.is_empty() {
            page_items.push(text("Empty page").dim().into());
        }

        let page_card = column(page_items)
            .bg(document_bg)
            .padding(Padding::symmetric(16, 28))
            .width(Length::Fill)
            .height(Length::Shrink);

        let page_container = column(vec![page_card.into()])
        .bg(document_bg)
        .width(Length::Fill)
        .height(Length::Shrink);

        let content_view = scroll(page_container).height(Length::Fill);

        let is_home = active_tab.url() == "about:hamix" || active_tab.url() == "about:home";
        let content_view = if self.settings_open {
            let mut items = vec![
                heading("Browser settings").into(),
                vspace(12).into(),
                text("Home page").strong().into(),
                input(&self.homepage, "about:hamix or https://…", Msg::HomeChanged).on_submit(Msg::SaveHome).into(),
                vspace(8).into(),
                row(vec![button("Save settings").on_press(Msg::SaveHome).into(), hspace(8).into(), button("Done").on_press(Msg::ToggleSettings).into()]).into(),
                divider(),
                subheading("Bookmarks").into(),
            ];
            if self.bookmarks.list().is_empty() {
                items.push(text("No bookmarks saved").dim().into());
            }
            for (index, bookmark) in self.bookmarks.list().iter().enumerate() {
                items.push(container(row(vec![
                    column(vec![text(bookmark.title.clone()).strong().into(), text(bookmark.url.clone()).small().dim().into()]).width(Length::Fill).into(),
                    button("Open").on_press(Msg::OpenBookmark(bookmark.url.clone())).into(),
                    hspace(6).into(),
                    button("Remove").on_press(Msg::RemoveBookmark(index)).into(),
                ])).padding(10).into());
                items.push(vspace(4).into());
            }
            scroll(container(column(items).padding(24).width(Length::Fill).height(Length::Shrink)).height(Length::Fill)).height(Length::Fill)
        } else if is_home {
            let hero = container(column(vec![
                heading("The web, at your pace").font("display").color(document_heading).into(),
                vspace(6).into(),
                text("A quiet starting point for HamixOS. Search the web, open your bookmarks, or enter an address above.")
                    .wrap().color(document_muted).into(),
            ]))
            .bg(chrome_bg)
            .radius(12)
            .padding(Padding::symmetric(26, 30))
            .width(Length::Fill)
            .height(Length::Shrink);
            let tiles = column(vec![
                row(vec![
                    home_tile("HamixOS documentation", "System guides and help", "about:help", "ui/file", chrome_border),
                    hspace(14).into(),
                    home_tile("Search the web", "Find pages with DuckDuckGo", "https://html.duckduckgo.com/html/", "ui/search", chrome_border),
                ]).into(),
                vspace(12).into(),
                row(vec![
                    home_tile("NetSurf project", "The browser engine behind this project", "https://www.netsurf-browser.org/", "ui/network", chrome_border),
                    hspace(14).into(),
                    home_tile("HamixOS", "Visit the project portal", "https://hamixos.org", "ui/home", chrome_border),
                ]).into(),
            ]);
            scroll(container(column(vec![
                hero.into(),
                vspace(24).into(),
                subheading("Quick access").color(document_heading).into(),
                vspace(10).into(),
                tiles.into(),
            ]).padding(Padding::symmetric(34, 48)).width(Length::Fill).height(Length::Shrink))
                .height(Length::Shrink))
                .height(Length::Fill)
        } else { content_view };

        column(vec![
            container(tab_bar).bg(chrome_bg).padding(Padding::symmetric(3, 10)).height(Length::Shrink).into(),
            container(nav_bar).bg(chrome_bg).padding(Padding::symmetric(3, 10)).height(Length::Shrink).into(),
            container(bookmark_bar).bg(chrome_bg).padding(Padding::symmetric(2, 12)).height(Length::Shrink).into(),
            container(divider()).bg(chrome_bg).height(Length::Shrink).into(),
            content_view.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn key(&self, press: &KeyPress) -> Option<Msg> {
        if press.mods.ctrl {
            match press.key {
                Key::Char('t') | Key::Char('T') => return Some(Msg::NewTab),
                Key::Char('w') | Key::Char('W') => return Some(Msg::CloseTab(self.tabs.active_index())),
                Key::Char('r') | Key::Char('R') => return Some(Msg::Reload),
                Key::Char('d') | Key::Char('D') => return Some(Msg::AddBookmark),
                Key::Char('q') | Key::Char('Q') => return Some(Msg::Quit),
                Key::Char('l') | Key::Char('L') => return Some(Msg::FocusAddress),
                _ => {}
            }
        }
        if press.mods.alt {
            match press.key {
                Key::Left => return Some(Msg::GoBack),
                Key::Right => return Some(Msg::GoForward),
                _ => {}
            }
        }
        match press.key {
            Key::F(5) => Some(Msg::Reload),
            _ => None,
        }
    }

    fn menu(&self) -> Vec<MenuItem<Msg>> {
        vec![
            MenuItem::submenu(
                "File",
                vec![
                    MenuItem::new("New Tab", Msg::NewTab).shortcut("Ctrl+T"),
                    MenuItem::new("Close Tab", Msg::CloseTab(self.tabs.active_index())).shortcut("Ctrl+W"),
                    MenuItem::new("Settings and Bookmarks", Msg::ToggleSettings),
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
            MenuItem::submenu("Bookmarks", vec![MenuItem::new("Manage Bookmarks and Settings", Msg::ToggleSettings)]),
            MenuItem::submenu(
                "Help",
                vec![
                    MenuItem::new("Keyboard Shortcuts", Msg::Shortcuts),
                    MenuItem::new("About Browser", Msg::About),
                ],
            ),
        ]
    }
}

fn run() -> i32 {
    let args = hxport::args();
    if args.get(1).map(String::as_str) == Some("--diagnose") {
        let Some(url) = args.get(2) else {
            hxport::stdout("FAIL Browser diagnostic needs a URL\n");
            return 2;
        };
        return match hxsurf::page::fetch_network_content(url) {
            Ok(content) if !content.trim().is_empty() => {
                hxport::stdout(&format!("PASS Browser fetched {}\n", url));
                0
            }
            Ok(_) => {
                hxport::stdout("FAIL Browser received an empty response\n");
                1
            }
            Err(error) => {
                hxport::stdout(&format!("FAIL Browser fetch: {}\n", error));
                1
            }
        };
    }
    hxtoolkit::run(
        Settings::new("browser").icon("web-browser")
            .size(1024, 680)
            .min_size(540, 380),
        BrowserApp::new(),
    )
}

#[cfg(target_os = "hamix")]
hxstd::entry!(run);

#[cfg(not(target_os = "hamix"))]
hxtoolkit::main!(run);
