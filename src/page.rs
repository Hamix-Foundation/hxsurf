extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageNode {
    Heading(u8, String),
    Paragraph(String),
    Link { text: String, url: String },
    Bullet(String),
    Numbered(usize, String),
    CodeBlock(String),
    Quote(String),
    Notice { title: String, text: String, is_error: bool },
    Divider,
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
    pub nodes: Vec<PageNode>,
    pub status: String,
}

impl Default for Page {
    fn default() -> Self {
        Self::new("about:hamix")
    }
}

impl Page {
    pub fn new(url: &str) -> Self {
        load_page(url)
    }

    pub fn empty() -> Self {
        Self {
            url: String::from("about:blank"),
            title: String::from("about:blank"),
            state: PageState::Empty,
            nodes: Vec::new(),
            status: String::from("Ready"),
        }
    }
}

pub fn decode_entities(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '&' {
            let mut entity = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ';' {
                    chars.next();
                    break;
                }
                if next_ch.is_whitespace() || entity.len() > 10 {
                    break;
                }
                entity.push(chars.next().unwrap());
            }
            if entity.starts_with('#') {
                let code_point = if entity.starts_with("#x") || entity.starts_with("#X") {
                    u32::from_str_radix(&entity[2..], 16).ok()
                } else {
                    entity[1..].parse::<u32>().ok()
                };
                if let Some(cp) = code_point.and_then(char::from_u32) {
                    out.push(cp);
                    continue;
                }
            }
            match entity.as_str() {
                "amp" => out.push('&'),
                "lt" => out.push('<'),
                "gt" => out.push('>'),
                "quot" => out.push('"'),
                "apos" | "#39" => out.push('\''),
                "nbsp" => out.push(' '),
                "mdash" | "#8212" => out.push('—'),
                "ndash" | "#8211" => out.push('–'),
                "copy" => out.push('©'),
                "reg" => out.push('®'),
                "trade" => out.push('™'),
                "bull" => out.push('•'),
                "hellip" => out.push('…'),
                _ => {
                    out.push('&');
                    out.push_str(&entity);
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn strip_tags(html: &str) -> String {
    let mut clean = String::with_capacity(html.len());
    let mut inside = false;
    for ch in html.chars() {
        if ch == '<' {
            inside = true;
        } else if ch == '>' {
            inside = false;
        } else if !inside {
            clean.push(ch);
        }
    }
    decode_entities(clean.trim())
}

pub fn resolve_url(base_url: &str, href: &str) -> String {
    let trimmed = href.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("file://") || trimmed.starts_with("about:") {
        return trimmed.to_string();
    }
    if trimmed.starts_with("//") {
        if base_url.starts_with("https://") {
            return format!("https:{}", trimmed);
        } else {
            return format!("http:{}", trimmed);
        }
    }
    if let Some(scheme_end) = base_url.find("://") {
        let scheme = &base_url[..scheme_end + 3];
        let rest = &base_url[scheme_end + 3..];
        let host = if let Some(slash) = rest.find('/') {
            &rest[..slash]
        } else {
            rest
        };
        if trimmed.starts_with('/') {
            return format!("{}{}{}", scheme, host, trimmed);
        }
        let base_dir = if let Some(last_slash) = base_url.rfind('/') {
            if last_slash > scheme_end + 2 {
                &base_url[..last_slash + 1]
            } else {
                base_url
            }
        } else {
            base_url
        };
        if base_dir.ends_with('/') {
            format!("{}{}", base_dir, trimmed)
        } else {
            format!("{}/{}", base_dir, trimmed)
        }
    } else {
        trimmed.to_string()
    }
}

#[cfg(target_os = "hamix")]
fn fetch_curl(url: &str) -> Result<String, String> {
    let tmp_file = "/tmp/hxsurf_download.tmp";
    let error_file = "/tmp/hxsurf_download.err";
    let _ = hxport::remove(tmp_file);
    let _ = hxport::remove(error_file);
    let pid = hxstd::sys::spawn(
        "/usr/bin/curl",
        &[
            "--silent",
            "--show-error",
            "--fail",
            "--location",
            "--connect-timeout",
            "8",
            "--max-time",
            "20",
            "--user-agent",
            "hxsurf/0.1",
            "--stderr",
            error_file,
            "--output",
            tmp_file,
            url,
        ],
        0,
    );
    if pid < 0 {
        return Err(String::from("curl process execution failed"));
    }
    let code = hxstd::sys::waitpid(pid, false);
    if code != 0 {
        let detail = hxport::read_to_string(error_file).unwrap_or_else(|| format!("curl exited with code {}", code));
        let _ = hxport::remove(tmp_file);
        let _ = hxport::remove(error_file);
        return Err(detail.trim().to_string());
    }
    let content = hxport::read_to_string(tmp_file).ok_or_else(|| String::from("Failed to read response file"))?;
    let _ = hxport::remove(tmp_file);
    let _ = hxport::remove(error_file);
    if content.is_empty() {
        return Err(String::from("Empty content returned"));
    }
    Ok(content)
}

#[cfg(not(target_os = "hamix"))]
fn fetch_curl(url: &str) -> Result<String, String> {
    let output = std::process::Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--fail",
            "--location",
            "--connect-timeout",
            "8",
            "--max-time",
            "20",
            "--user-agent",
            "hxsurf/0.1",
            url,
        ])
        .output()
        .map_err(|e| format!("curl execution error: {}", e))?;
    if !output.status.success() {
        return Err(format!("curl exited with code {:?}", output.status.code()));
    }
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.is_empty() {
        return Err(String::from("Empty response from host"));
    }
    Ok(text)
}

#[cfg(target_os = "hamix")]
fn fetch_socket(host: &str, port: u16, path: &str) -> Result<String, String> {
    use hxstd::net::{resolve, TcpStream};

    let ip = resolve(host).map_err(|e| format!("Cannot resolve host '{}': error {}", host, e))?;
    let mut stream = TcpStream::connect(ip, port, 6000).map_err(|e| format!("Cannot connect to {}:{}: error {}", host, port, e))?;
    let req = format!("GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: hxsurf/0.1.0\r\nAccept: text/html,text/plain,*/*\r\nConnection: close\r\n\r\n", path, host);
    if stream.send(req.as_bytes()) < 0 {
        return Err(String::from("Failed to send HTTP request"));
    }
    let mut data = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.recv(&mut buf, 6000);
        if n <= 0 {
            break;
        }
        data.extend_from_slice(&buf[..n as usize]);
        if data.len() > 1024 * 1024 {
            break;
        }
    }
    let full = String::from_utf8_lossy(&data).into_owned();
    if let Some((_, body)) = full.split_once("\r\n\r\n") {
        Ok(String::from(body))
    } else {
        Ok(full)
    }
}

#[cfg(not(target_os = "hamix"))]
fn fetch_socket(host: &str, port: u16, path: &str) -> Result<String, String> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&addr).map_err(|e| format!("Cannot connect to {}: {}", addr, e))?;
    let req = format!("GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: hxsurf/0.1.0\r\nAccept: text/html,text/plain,*/*\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(req.as_bytes()).map_err(|e| format!("Failed to send HTTP request: {}", e))?;
    let mut data = Vec::new();
    let _ = stream.read_to_end(&mut data);
    let full = String::from_utf8_lossy(&data).into_owned();
    if let Some((_, body)) = full.split_once("\r\n\r\n") {
        Ok(String::from(body))
    } else {
        Ok(full)
    }
}

fn fetch_network_content(url_string: &str) -> Result<String, String> {
    let curl_result = fetch_curl(url_string);
    let body = match curl_result {
        Ok(body) => return Ok(body),
        Err(err) => err,
    };

    if url_string.starts_with("https://") {
        return Err(format!("Secure connection failed: {}", body));
    }

    let without_scheme = if let Some(pos) = url_string.find("://") {
        &url_string[pos + 3..]
    } else {
        url_string
    };

    let (authority, path) = if let Some(slash) = without_scheme.find('/') {
        (&without_scheme[..slash], &without_scheme[slash..])
    } else {
        (without_scheme, "/")
    };

    let (host, port) = if let Some((h, p)) = authority.rsplit_once(':') {
        (h, p.parse::<u16>().unwrap_or(80))
    } else {
        (authority, 80u16)
    };

    fetch_socket(host, port, path)
}

fn encode_query(query: &str) -> String {
    let mut encoded = String::with_capacity(query.len());
    for byte in query.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{:02X}", byte));
        }
    }
    encoded
}

pub fn parse_html(html: &str, base_url: &str) -> (String, Vec<PageNode>) {
    let mut nodes = Vec::new();
    let mut title = String::new();

    let mut filtered = String::with_capacity(html.len());
    let mut i = 0;
    let lower = html.to_lowercase();
    while i < html.len() {
        if lower[i..].starts_with("<script") {
            if let Some(end) = lower[i..].find("</script>") {
                i += end + 9;
                continue;
            }
        }
        if lower[i..].starts_with("<style") {
            if let Some(end) = lower[i..].find("</style>") {
                i += end + 8;
                continue;
            }
        }
        if lower[i..].starts_with("<!--") {
            if let Some(end) = lower[i..].find("-->") {
                i += end + 3;
                continue;
            }
        }
        if lower[i..].starts_with("<noscript") {
            if let Some(end) = lower[i..].find("</noscript>") {
                i += end + 11;
                continue;
            }
        }
        if let Some(ch) = html[i..].chars().next() {
            filtered.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }

    if let Some(start) = filtered.to_lowercase().find("<title>") {
        if let Some(end) = filtered.to_lowercase()[start..].find("</title>") {
            title = strip_tags(&filtered[start + 7..start + end]);
        }
    }

    let mut pos = 0;
    let s = filtered.as_str();
    while pos < s.len() {
        if let Some(tag_start) = s[pos..].find('<') {
            let text_before = strip_tags(&s[pos..pos + tag_start]);
            if !text_before.is_empty() {
                nodes.push(PageNode::Paragraph(text_before));
            }
            pos += tag_start;
            if let Some(tag_end) = s[pos..].find('>') {
                let tag_body = &s[pos + 1..pos + tag_end];
                let tag_name = tag_body.split_whitespace().next().unwrap_or("").to_lowercase();
                pos += tag_end + 1;

                if tag_name == "h1" || tag_name == "h2" || tag_name == "h3" || tag_name == "h4" || tag_name == "h5" || tag_name == "h6" {
                    let level = match tag_name.as_str() {
                        "h1" => 1,
                        "h2" => 2,
                        "h3" => 3,
                        _ => 4,
                    };
                    let close_tag = format!("</{}>", tag_name);
                    if let Some(close_pos) = s[pos..].to_lowercase().find(&close_tag) {
                        let h_text = strip_tags(&s[pos..pos + close_pos]);
                        if !h_text.is_empty() {
                            nodes.push(PageNode::Heading(level, h_text));
                        }
                        pos += close_pos + close_tag.len();
                    }
                } else if tag_name == "a" {
                    let mut href = String::new();
                    if let Some(h_idx) = tag_body.to_lowercase().find("href=") {
                        let rest = tag_body[h_idx + 5..].trim_start();
                        if let Some(quote) = rest.chars().next() {
                            if quote == '"' || quote == '\'' {
                                if let Some(end_quote) = rest[1..].find(quote) {
                                    href = String::from(&rest[1..1 + end_quote]);
                                }
                            }
                        }
                    }
                    if let Some(close_pos) = s[pos..].to_lowercase().find("</a>") {
                        let a_text = strip_tags(&s[pos..pos + close_pos]);
                        let label = if a_text.is_empty() { href.clone() } else { a_text };
                        if !href.is_empty() {
                            let resolved = resolve_url(base_url, &href);
                            nodes.push(PageNode::Link { text: label, url: resolved });
                        }
                        pos += close_pos + 4;
                    }
                } else if tag_name == "li" {
                    let close_pos = s[pos..].to_lowercase().find("</li>").or_else(|| s[pos..].to_lowercase().find("<li")).unwrap_or(s[pos..].len().min(400));
                    let item_text = strip_tags(&s[pos..pos + close_pos]);
                    if !item_text.is_empty() {
                        nodes.push(PageNode::Bullet(item_text));
                    }
                    if s[pos..].to_lowercase().starts_with("</li>") {
                        pos += 5;
                    } else {
                        pos += close_pos;
                    }
                } else if tag_name == "pre" || tag_name == "code" {
                    let close_tag = if tag_name == "pre" { "</pre>" } else { "</code>" };
                    if let Some(close_pos) = s[pos..].to_lowercase().find(close_tag) {
                        let code_text = decode_entities(&s[pos..pos + close_pos]);
                        if !code_text.is_empty() {
                            nodes.push(PageNode::CodeBlock(code_text));
                        }
                        pos += close_pos + close_tag.len();
                    }
                } else if tag_name == "blockquote" {
                    if let Some(close_pos) = s[pos..].to_lowercase().find("</blockquote>") {
                        let q_text = strip_tags(&s[pos..pos + close_pos]);
                        if !q_text.is_empty() {
                            nodes.push(PageNode::Quote(q_text));
                        }
                        pos += close_pos + 13;
                    }
                } else if tag_name == "hr" {
                    nodes.push(PageNode::Divider);
                } else if tag_name == "p" {
                    if let Some(close_pos) = s[pos..].to_lowercase().find("</p>") {
                        let p_raw = &s[pos..pos + close_pos];
                        if p_raw.contains("<a ") || p_raw.contains("<a>") {
                            let mut inner_pos = 0;
                            while inner_pos < p_raw.len() {
                                if let Some(a_start) = p_raw[inner_pos..].to_lowercase().find("<a ") {
                                    let before = strip_tags(&p_raw[inner_pos..inner_pos + a_start]);
                                    if !before.is_empty() {
                                        nodes.push(PageNode::Paragraph(before));
                                    }
                                    let rest_a = &p_raw[inner_pos + a_start..];
                                    if let Some(a_end_tag) = rest_a.find('>') {
                                        let a_tag_body = &rest_a[..a_end_tag];
                                        let mut href = String::new();
                                        if let Some(h_idx) = a_tag_body.to_lowercase().find("href=") {
                                            let h_val = a_tag_body[h_idx + 5..].trim_start();
                                            if let Some(q) = h_val.chars().next() {
                                                if q == '"' || q == '\'' {
                                                    if let Some(q_end) = h_val[1..].find(q) {
                                                        href = String::from(&h_val[1..1 + q_end]);
                                                    }
                                                }
                                            }
                                        }
                                        let after_tag = &rest_a[a_end_tag + 1..];
                                        if let Some(close_a) = after_tag.to_lowercase().find("</a>") {
                                            let label = strip_tags(&after_tag[..close_a]);
                                            if !href.is_empty() {
                                                nodes.push(PageNode::Link {
                                                    text: if label.is_empty() { href.clone() } else { label },
                                                    url: resolve_url(base_url, &href),
                                                });
                                            }
                                            inner_pos += a_start + a_end_tag + 1 + close_a + 4;
                                            continue;
                                        }
                                    }
                                    break;
                                } else {
                                    let tail = strip_tags(&p_raw[inner_pos..]);
                                    if !tail.is_empty() {
                                        nodes.push(PageNode::Paragraph(tail));
                                    }
                                    break;
                                }
                            }
                        } else {
                            let p_text = strip_tags(p_raw);
                            if !p_text.is_empty() {
                                nodes.push(PageNode::Paragraph(p_text));
                            }
                        }
                        pos += close_pos + 4;
                    }
                }
            } else {
                break;
            }
        } else {
            let tail = strip_tags(&s[pos..]);
            if !tail.is_empty() {
                nodes.push(PageNode::Paragraph(tail));
            }
            break;
        }
    }

    if title.is_empty() {
        title = base_url.to_string();
    }

    (title, nodes)
}

pub fn load_page(url: &str) -> Page {
    let trimmed = url.trim();
    if trimmed == "about:hamix" || trimmed == "about:home" || trimmed.is_empty() {
        return Page {
            url: String::from("about:hamix"),
            title: String::from("hxsurf"),
            state: PageState::Loaded,
            status: String::from("Ready"),
            nodes: vec![
                PageNode::Heading(1, String::from("hxsurf")),
                PageNode::Paragraph(String::from("A lightweight web browser for HamixOS. Enter a web address or search the web from the address bar.")),
                PageNode::Divider,
                PageNode::Heading(2, String::from("Quick Navigation")),
                PageNode::Link {
                    text: String::from("HamixOS System Documentation"),
                    url: String::from("file:///usr/share/doc/hamix/index.html"),
                },
                PageNode::Link {
                    text: String::from("DuckDuckGo HTML Search"),
                    url: String::from("https://html.duckduckgo.com/html/"),
                },
                PageNode::Link {
                    text: String::from("NetSurf Browser Project"),
                    url: String::from("https://www.netsurf-browser.org/"),
                },
                PageNode::Link {
                    text: String::from("HamixOS Project Portal"),
                    url: String::from("https://hamixos.org"),
                },
                PageNode::Divider,
                PageNode::Heading(2, String::from("Keyboard Shortcuts")),
                PageNode::Bullet(String::from("Ctrl+T: Open new tab")),
                PageNode::Bullet(String::from("Ctrl+W: Close active tab")),
                PageNode::Bullet(String::from("Ctrl+L: Jump to URL bar")),
                PageNode::Bullet(String::from("Ctrl+R / F5: Reload page")),
                PageNode::Bullet(String::from("Alt+Left: Navigate back in history")),
                PageNode::Bullet(String::from("Alt+Right: Navigate forward in history")),
                PageNode::Bullet(String::from("Ctrl+D: Bookmark current page")),
                PageNode::Bullet(String::from("Ctrl+Q: Quit browser")),
            ],
        };
    }

    if trimmed == "about:blank" {
        return Page::empty();
    }

    if trimmed == "about:help" {
        return Page {
            url: String::from("about:help"),
            title: String::from("hxsurf Help"),
            state: PageState::Loaded,
            status: String::from("Help & Documentation"),
            nodes: vec![
                PageNode::Heading(1, String::from("hxsurf Help")),
                PageNode::Paragraph(String::from("hxsurf is the standard web browser designed for the Nook desktop on HamixOS.")),
                PageNode::Divider,
                PageNode::Heading(2, String::from("Navigation")),
                PageNode::Paragraph(String::from("Enter a complete URL or a search query in the address bar. If no protocol is specified, hxsurf automatically resolves web domains.")),
                PageNode::Link {
                    text: String::from("Return to Home"),
                    url: String::from("about:hamix"),
                },
            ],
        };
    }

    if trimmed.starts_with("file://") {
        let path = &trimmed["file://".len()..];
        return match hxport::read_to_string(path) {
            Some(content) => {
                let (title, nodes) = parse_html(&content, trimmed);
                Page {
                    url: String::from(trimmed),
                    title,
                    state: PageState::Loaded,
                    nodes,
                    status: format!("Loaded {} bytes from {}", content.len(), path),
                }
            }
            None => Page {
                url: String::from(trimmed),
                title: String::from("File Not Found"),
                state: PageState::Error,
                status: format!("Error reading {}", path),
                nodes: vec![
                    PageNode::Notice {
                        title: String::from("File Not Found"),
                        text: format!("Could not read local file '{}'. Please check path and permissions.", path),
                        is_error: true,
                    },
                    PageNode::Link {
                        text: String::from("Return to Home"),
                        url: String::from("about:hamix"),
                    },
                ],
            },
        };
    }

    let url_string = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        String::from(trimmed)
    } else if trimmed.contains('.') && !trimmed.contains(' ') {
        format!("https://{}", trimmed)
    } else {
        format!("https://html.duckduckgo.com/html/?q={}", encode_query(trimmed))
    };

    match fetch_network_content(&url_string) {
        Ok(body) => {
            let (title, nodes) = parse_html(&body, &url_string);
            Page {
                url: url_string,
                title,
                state: PageState::Loaded,
                nodes,
                status: format!("Loaded {} bytes", body.len()),
            }
        }
        Err(err) => {
            Page {
                url: url_string,
                title: String::from("Network Error"),
                state: PageState::Error,
                status: err.clone(),
                nodes: vec![
                    PageNode::Notice {
                        title: String::from("Cannot Open Website"),
                        text: err,
                        is_error: true,
                    },
                    PageNode::Paragraph(String::from("Troubleshooting:")),
                    PageNode::Bullet(String::from("Ensure your network interface is active and connected.")),
                    PageNode::Bullet(String::from("Check if the remote host supports HTTP/HTTPS connections.")),
                    PageNode::Bullet(String::from("Verify that the domain name is spelled correctly.")),
                    PageNode::Divider,
                    PageNode::Link {
                        text: String::from("Back to Home Portal"),
                        url: String::from("about:hamix"),
                    },
                ],
            }
        }
    }
}
