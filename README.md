# HxSurf

HxSurf is a fast, lightweight, and native web browser for HamixOS, built on the NetSurf rendering engine and the `hxtoolkit` / `vellum` desktop toolkit.

## Features

- **Lightweight & Fast**: Engineered to run smoothly on low-resource machines without the overhead of heavy browser engines.
- **Standards Support**: HTML5 and CSS3 parsing powered by NetSurf's `libhubbub`, `libcss`, and `libdom`.
- **Native HamixOS Desktop Integration**: Blends with the Nook desktop environment using `hxtoolkit` widgets and theme colors.
- **Monorepo Architecture**: Integrates all necessary support libraries (`libwapcaplet`, `libparserutils`, `libhubbub`, `libcss`, `libdom`, `libnslog`, `libnspsl`, etc.) in one clean repository.

## Repository Layout

- `buildsystem/` - Build system definitions and toolchain integration.
- `libwapcaplet/` - String interning library.
- `libparserutils/` - Character encoding and low-level parsing helpers.
- `libhubbub/` - HTML5 parser.
- `libcss/` - CSS parser and cascade engine.
- `libdom/` - W3C DOM implementation.
- `libnslog/` - Logging infrastructure.
- `libnspsl/` - Public Suffix List handler.
- `libnsbmp/`, `libnsgif/` - Lightweight image decoders.
- `netsurf/` - Browser core layout and networking logic.
- `netsurf/frontends/hamix/` - Native HamixOS `hxtoolkit` frontend.

## Compilation

```bash
make TARGET=hamix
```
