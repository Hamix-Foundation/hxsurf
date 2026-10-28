#![cfg_attr(target_os = "hamix", no_std)]
#![cfg_attr(target_os = "hamix", no_main)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use hxtoolkit::{
    button, column, container, divider, heading, hspace, input, link, row, scroll, space,
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
}

impl BrowserApp {
    fn new() -> Self {
        let initial_url = hxport::args().get(1).cloned().unwrap_or_else(|| String::from("about:hamix"));
        let tabs = TabManager::new(&initial_url);
        let bookmarks = BookmarkStore::new();
        let status = (tabs.active_tab().page.status.clone(), Tone::Dim);

        Self {
            tabs,
            bookmarks,
            url_input: initial_url,
            status,
        }
    }
}

impl App for BrowserApp {
    type Message = Msg;

    fn title(&self) -> String {
        format!("{} — hxsurf", self.tabs.active_tab().title())
    }

    fn update(&mut self, msg: Msg, cx: &mut Context<Msg>) {
        match msg {
            Msg::UrlChanged(val) => {
                self.url_input = val;
            }
            Msg::Navigate(target_url) => {
                self.tabs.active_tab_mut().navigate(&target_url);
                self.url_input = self.tabs.active_tab().url().to_string();
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::GoBack => {
                if self.tabs.active_tab_mut().back() {
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
                } else {
                    self.status = (String::from("No previous page in history"), Tone::Warning);
                }
            }
            Msg::GoForward => {
                if self.tabs.active_tab_mut().forward() {
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
                } else {
                    self.status = (String::from("No forward page in history"), Tone::Warning);
                }
            }
            Msg::Reload => {
                self.tabs.active_tab_mut().reload();
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::Home => {
                self.tabs.active_tab_mut().navigate("about:hamix");
                self.url_input = String::from("about:hamix");
                self.status = (String::from("Ready"), Tone::Dim);
            }
            Msg::NewTab => {
                let idx = self.tabs.open_tab("about:hamix");
                self.url_input = String::from("about:hamix");
                self.status = (format!("Opened tab #{}", idx + 1), Tone::Dim);
            }
            Msg::SelectTab(idx) => {
                self.tabs.select_tab(idx);
                self.url_input = self.tabs.active_tab().url().to_string();
                self.status = (self.tabs.active_tab().page.status.clone(), Tone::Dim);
            }
            Msg::CloseTab(idx) => {
                if self.tabs.close_tab(idx) {
                    self.url_input = self.tabs.active_tab().url().to_string();
                    self.status = (String::from("Closed tab"), Tone::Dim);
                }
            }
            Msg::AddBookmark => {
                let url = self.tabs.active_tab().url().to_string();
                let title = self.tabs.active_tab().title().to_string();
                self.bookmarks.add(title, url);
                self.status = (String::from("Page bookmarked"), Tone::Success);
            }
            Msg::OpenBookmark(url) => {
                self.tabs.active_tab_mut().navigate(&url);
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
                    "About hxsurf",
                    "hxsurf 0.1.0\nFast and lightweight native web browser for HamixOS, built on the NetSurf layout engine and hxtoolkit.",
                );
            }
            Msg::Quit => {
                cx.quit();
            }
        }
    }

    fn view(&self) -> Element<Msg> {
        let active_tab_index = self.tabs.active_index();
        let mut tab_buttons: Vec<Element<Msg>> = Vec::new();
        for (i, tab) in self.tabs.tabs().iter().enumerate() {
            let is_active = i == active_tab_index;
            let display_title = if tab.title().len() > 18 {
                let mut s = String::from(&tab.title()[..18]);
                s.push('…');
                s
            } else {
                tab.title().to_string()
            };

            if is_active {
                let active_item = container(row(vec![
                    text(display_title).strong().color(0x0f172a).into(),
                    hspace(6).into(),
                    button("×").flat().size(TextSize::Small).on_press(Msg::CloseTab(i)).into(),
                ]))
                .bg(0xffffff)
                .border(0xcfd7e3, 1)
                .radius(6)
                .padding(Padding::symmetric(6, 12));
                tab_buttons.push(active_item.into());
            } else {
                let mut inactive_row = vec![
                    button(display_title).flat().size(TextSize::Small).on_press(Msg::SelectTab(i)).into(),
                ];
                if self.tabs.tabs().len() > 1 {
                    inactive_row.push(hspace(2).into());
                    inactive_row.push(button("×").flat().size(TextSize::Small).on_press(Msg::CloseTab(i)).into());
                }
                let inactive_item = container(row(inactive_row))
                .bg(0xe2e8f0)
                .radius(6)
                .padding(Padding::symmetric(4, 8));
                tab_buttons.push(inactive_item.into());
            }
            tab_buttons.push(hspace(4).into());
        }
        tab_buttons.push(button("+").flat().size(TextSize::Small).on_press(Msg::NewTab).into());
        let tab_bar = row(tab_buttons);

        let active_tab = self.tabs.active_tab();
        let nav_bar = row(vec![
            button("◀")
                .flat()
                .on_press_maybe(if active_tab.history.can_go_back() { Some(Msg::GoBack) } else { None })
                .into(),
            hspace(2).into(),
            button("▶")
                .flat()
                .on_press_maybe(if active_tab.history.can_go_forward() { Some(Msg::GoForward) } else { None })
                .into(),
            hspace(2).into(),
            button("⟳").flat().on_press(Msg::Reload).into(),
            hspace(2).into(),
            button("⌂").flat().on_press(Msg::Home).into(),
            hspace(8).into(),
            input(&self.url_input, "Search DuckDuckGo or enter URL...", Msg::UrlChanged)
                .on_submit(Msg::Navigate(self.url_input.clone()))
                .width(Length::Fill)
                .into(),
            hspace(6).into(),
            button("Go").primary().on_press(Msg::Navigate(self.url_input.clone())).into(),
            hspace(4).into(),
            button("★").flat().on_press(Msg::AddBookmark).into(),
        ]);

        let mut bm_buttons: Vec<Element<Msg>> = Vec::new();
        bm_buttons.push(text("Bookmarks:").small().dim().into());
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
                    page_items.push(heading(h.clone()).color(0x0f172a).font("display").into());
                    page_items.push(vspace(14).into());
                }
                PageNode::Heading(2, h) => {
                    page_items.push(subheading(h.clone()).color(0x1e293b).font("title").into());
                    page_items.push(vspace(12).into());
                }
                PageNode::Heading(3, h) => {
                    page_items.push(text(h.clone()).strong().color(0x334155).font("strong").into());
                    page_items.push(vspace(8).into());
                }
                PageNode::Heading(_, h) => {
                    page_items.push(text(h.clone()).strong().color(0x475569).into());
                    page_items.push(vspace(6).into());
                }
                PageNode::Paragraph(p) => {
                    page_items.push(text(p.clone()).wrap().color(0x1e293b).fill().into());
                    page_items.push(vspace(8).into());
                }
                PageNode::Link { text: l_text, url: l_url } => {
                    let target = l_url.clone();
                    page_items.push(link(l_text.clone()).text_color(0x2563eb).on_press(Msg::Navigate(target)).into());
                    page_items.push(vspace(6).into());
                }
                PageNode::Bullet(b) => {
                    let bullet_row = row(vec![
                        text("• ").color(0x64748b).into(),
                        text(b.clone()).wrap().color(0x334155).fill().into(),
                    ]);
                    page_items.push(bullet_row.into());
                    page_items.push(vspace(4).into());
                }
                PageNode::Numbered(n, b) => {
                    let num_row = row(vec![
                        text(format!("{}. ", n)).color(0x64748b).into(),
                        text(b.clone()).wrap().color(0x334155).fill().into(),
                    ]);
                    page_items.push(num_row.into());
                    page_items.push(vspace(4).into());
                }
                PageNode::CodeBlock(c) => {
                    let code_box = container(text(c.clone()).mono().font("mono").color(0x0f172a))
                        .bg(0xf8fafc)
                        .border(0xe2e8f0, 1)
                        .radius(6)
                        .padding(12);
                    page_items.push(code_box.into());
                    page_items.push(vspace(10).into());
                }
                PageNode::Quote(q) => {
                    let quote_box = container(text(q.clone()).wrap().color(0x475569).fill())
                        .bg(0xf8fafc)
                        .border(0x94a3b8, 2)
                        .radius(4)
                        .padding(12);
                    page_items.push(quote_box.into());
                    page_items.push(vspace(10).into());
                }
                PageNode::Notice { title, text: n_text, is_error } => {
                    let (bg_col, border_col, title_col, text_col) = if *is_error {
                        (0xfef2f2, 0xfecaca, 0x991b1b, 0x7f1d1d)
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
                    .padding(14);
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
            .bg(0xffffff)
            .border(0xd6dae1, 1)
            .radius(8)
            .padding(Padding::symmetric(24, 32))
            .width(Length::Fill);

        let page_container = column(vec![
            page_card.into(),
            vspace(24).into(),
        ])
        .padding(Padding::symmetric(10, 16))
        .width(Length::Fill);

        let content_view = scroll(page_container).height(Length::Fill);

        let (status_text, status_tone) = &self.status;
        let is_https = active_tab.url().starts_with("https://");
        let is_local = active_tab.url().starts_with("file://") || active_tab.url().starts_with("about:");
        let sec_badge = if is_https {
            "🔒 Secure"
        } else if is_local {
            "📄 Local"
        } else {
            "🌐 HTTP"
        };

        let status_bar = row(vec![
            text(sec_badge).small().tone(Tone::Dim).into(),
            hspace(8).into(),
            text(status_text.clone()).tone(*status_tone).into(),
            space().into(),
            text(format!("Tabs: {} | hxsurf", self.tabs.tabs().len())).tone(Tone::Dim).into(),
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

    fn key(&self, press: &KeyPress) -> Option<Msg> {
        if press.mods.ctrl {
            match press.key {
                Key::Char('t') | Key::Char('T') => return Some(Msg::NewTab),
                Key::Char('w') | Key::Char('W') => return Some(Msg::CloseTab(self.tabs.active_index())),
                Key::Char('r') | Key::Char('R') => return Some(Msg::Reload),
                Key::Char('d') | Key::Char('D') => return Some(Msg::AddBookmark),
                Key::Char('q') | Key::Char('Q') => return Some(Msg::Quit),
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
                    MenuItem::new("About hxsurf", Msg::About),
                ],
            ),
        ]
    }
}

fn run() -> i32 {
    hxtoolkit::run(
        Settings::new("hxsurf")
            .size(1024, 680)
            .min_size(540, 380),
        BrowserApp::new(),
    )
}

#[cfg(target_os = "hamix")]
hxstd::entry!(run);

#[cfg(not(target_os = "hamix"))]
hxtoolkit::main!(run);
