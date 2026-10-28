use std::env;
use hxsurf::{BookmarkStore, TabManager};

fn main() {
    let args: Vec<String> = env::args().collect();
    let initial_url = if args.len() > 1 {
        args[1].as_str()
    } else {
        "about:hamix"
    };

    println!("HxSurf 0.1.0 (NetSurf engine core for HamixOS)");
    println!("Loading initial page: {}", initial_url);

    let mut tab_mgr = TabManager::new(initial_url);
    let bookmarks = BookmarkStore::new();

    println!("Active Tab: [#{}] {}", tab_mgr.active_tab().id, tab_mgr.active_tab().url());
    println!("Bookmarks available: {}", bookmarks.list().len());
    for b in bookmarks.list() {
        println!(" - {} ({})", b.title, b.url);
    }

    tab_mgr.open_tab("file:///usr/share/doc/hamix/index.html");
    println!("Opened second tab: [#{}] {}", tab_mgr.active_tab().id, tab_mgr.active_tab().url());
    println!("Total tabs: {}", tab_mgr.tabs().len());
}
