# HxSurf

HxSurf is a fast, lightweight, and native web browser developed for **HamixOS**, built on the battle-tested **NetSurf** rendering engine with full **Rust** integration and native frontend bindings for `hxtoolkit` and `vellum`.

Designed to respect user freedom, system resources, and low-spec hardware, HxSurf provides modern web browsing capabilities with an extraordinarily small memory and CPU footprint.

---

## Highlights

- **Ultra-Lightweight & Responsive**: Starts in milliseconds and operates efficiently within 15–30 MB of RAM, making it suitable for older machines, embedded platforms, and fast desktop workflows.
- **Standards-Compliant Core**: Powered by NetSurf's modular C libraries:
  - **HTML5 parsing** via `libhubbub`
  - **CSS3 cascading and styling** via `libcss`
  - **W3C DOM Level 2 & 3** via `libdom`
  - **Fast string interning** via `libwapcaplet`
  - **Lightweight image decoding** (BMP, GIF, SVG) via `libnsbmp`, `libnsgif`, and `libsvgtiny`
- **Native HamixOS Look & Feel**: Integrates seamlessly with the Nook desktop environment using `hxtoolkit`, adhering to the system's retro aesthetic, widget styling, and color palette.
- **First-Class Rust Support**: Native Rust library (`hxsurf`) and application targets providing idiomatic Rust data models, tab management, bookmarks, history, and FFI wrappers for the C engine.
- **Tabbed Browsing**: Multi-tab management with clean switching, opening, and closing workflows.
- **Fast Navigation & Bookmarks**: Quick access toolbar, URL autocomplete, history back/forward navigation, and persistent bookmarks.

---

## Monorepo Layout

HxSurf integrates all core NetSurf libraries into a unified monorepo for straightforward builds and dependency management:

```
hxsurf/
├── buildsystem/              # NetSurf build system helpers and compiler toolchain rules
├── libwapcaplet/             # String interning library
├── libparserutils/           # Low-level parser helpers and character set conversion
├── libhubbub/                # Standards-compliant HTML5 parser
├── libcss/                   # CSS3 parser, property resolution, and cascading engine
├── libdom/                   # W3C DOM implementation
├── libnslog/                 # NetSurf structured logging framework
├── libnspsl/                 # Public Suffix List implementation
├── libnsbmp/                 # Lightweight BMP decoder
├── libnsgif/                 # Lightweight GIF decoder
├── libsvgtiny/               # Lightweight SVG rendering library
├── netsurf/                  # Browser core logic, layout engine, cache, and networking
│   ├── content/              # Resource loaders, MIME mapping, cache handles
│   ├── desktop/              # Window management, text selection, navigation
│   └── frontends/
│       ├── hamix/            # Native HamixOS frontend (C callbacks & bridge)
│       └── ...               # Additional frontends (monkey, GTK, framebuffer)
├── src/                      # Rust language bindings and application modules
│   ├── lib.rs                # Rust crate entry point
│   ├── engine.rs             # FFI bindings to NetSurf core
│   ├── tab.rs                # Tab management and state
│   ├── page.rs               # Document representation and links
│   ├── bookmarks.rs          # Bookmarks storage and search
│   ├── history.rs            # Navigation history back/forward stacks
│   └── main.rs               # Standalone browser executable
├── Cargo.toml                # Rust package definition
├── Makefile                  # Main GNU Make compilation entry
└── LICENSE                   # Complete licensing details
```

---

## Rust Integration

HxSurf provides a Rust API in addition to the traditional C frontends:

```rust
use hxsurf::{BookmarkStore, TabManager};

fn main() {
    let mut tabs = TabManager::new("about:hamix");
    println!("Active URL: {}", tabs.active_tab().url());

    tabs.open_tab("https://hamixos.org");
    println!("Total tabs: {}", tabs.tabs().len());

    let bookmarks = BookmarkStore::new();
    for item in bookmarks.list() {
        println!("Bookmark: {} -> {}", item.title, item.url);
    }
}
```

The Rust module architecture:
- `hxsurf::engine`: Safe abstraction over the C NetSurf lifecycle (`hamix_init`, `hamix_shutdown`, `hamix_window_create`, `hamix_poll`).
- `hxsurf::tab`: Multi-tab coordinator managing tab lifecycles, active index, and view states.
- `hxsurf::page`: Page document model containing URLs, titles, loading status, and extracted links.
- `hxsurf::bookmarks`: Storage and lookup for user-saved bookmarks.
- `hxsurf::history`: Forward and backward navigation stacks with URL and title caching.

---

## Building and Installation

### Prerequisites

Ensure you have a C99 compiler (`gcc` or `clang`), GNU Make, and `rustc` / `cargo` installed.

### 1. Building the Rust Crate and CLI

```bash
cargo build --release
```

Run the compiled executable:

```bash
./target/release/hxsurf https://hamixos.org
```

### 2. Building the C Core via GNU Make

To build the browser using the NetSurf GNU Make build system:

```bash
make TARGET=hamix
```

To build other supported targets (e.g., monkey test frontend):

```bash
make TARGET=monkey
```

---

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+T` | Open a new tab |
| `Ctrl+W` | Close the current tab |
| `Ctrl+L` | Focus address bar |
| `Ctrl+R` / `F5` | Reload the current page |
| `Alt+Left` | Navigate back |
| `Alt+Right` | Navigate forward |
| `Ctrl+D` | Add current page to bookmarks |
| `Ctrl+Q` | Quit HxSurf |

---

## License

HxSurf is licensed under the **GNU General Public License version 2 (GPL-2.0)** with a specific exception allowing linkage against OpenSSL. All visual artwork and iconography are licensed under the **MIT License**.

See the full text in the [`LICENSE`](LICENSE) file.
