# Architecture Comparison

This document compares the old server-side rendering architecture with the new client-side rendering + REST API architecture.

---

## 🏗️ Overview

### Previous Architecture (Server-Side Rendering)
Server generates complete HTML pages and sends them to the browser.

### Current Architecture (Client-Side Rendering + REST API)
Server provides JSON API, browser handles rendering with vanilla JavaScript.

---

## 🔄 Request Flow Comparison

### Previous (SSR)
```
1. Browser → GET /view/file.md
2. Server reads file
3. Server: pulldown-cmark (MD → HTML)
4. Server: html.rs (HTML + CSS inline)
5. Server → Complete HTML page
6. Browser displays
```

### Current (CSR + API)
```
1. Browser → GET /
2. Server → static/index.html (empty shell)
3. Browser loads app.js
4. JS → GET /api/files
5. Server → JSON { "files": [...] }
6. User clicks file → Hash route #/file.md
7. JS → GET /api/markdown/file.md
8. Server → JSON { "markdown": "..." }
9. Browser: marked.js (MD → HTML)
10. JS updates DOM
```

---

## 📂 File Structure

### Previous
```
src/
├── html.rs          # All HTML templates with inline CSS
├── ajax.rs          # Partial rendering for AJAX
├── server.rs        # Routes returning HTML
└── markdown.rs      # Server-side MD parsing
```

### Current
```
src/
├── server.rs        # Routes returning JSON API
└── markdown.rs      # Still used for caching

static/              # NEW: Client-side files
├── index.html       # Basic HTML shell
├── app.js           # Hash routing + rendering
└── style.css        # Extracted styles

External:
└── marked.js (CDN)  # Client-side MD parsing
```

---

## 🔌 API Endpoints

### New JSON Endpoints
```rust
GET /api/files
Response: { "files": ["file1.md", "file2.md"] }

GET /api/markdown/:filename
Response: { "markdown": "# Title\n\nContent..." }
```

### Legacy Endpoints (Still Available)
```rust
GET /                          → HTML directory page
GET /view/:filename            → HTML rendered page
GET /raw/:filename             → HTML raw markdown page
GET /api/content/:filename     → Partial HTML (AJAX)
```

---

## 🎨 Rendering Responsibility

| Aspect | Previous (SSR) | Current (CSR) |
|--------|---------------|--------------|
| **Markdown Parsing** | Server (`pulldown-cmark`) | Client (`marked.js`) |
| **HTML Generation** | Server (`html.rs`) | Client (DOM manipulation) |
| **CSS** | Inline in template | Separate `style.css` |
| **Routing** | Server routes | Hash routing (`#/file.md`) |
| **State Management** | Server per request | Client JS variables |

---

## ⚖️ Trade-offs

### Previous (SSR)

**Advantages:**
- ✅ SEO friendly (search engines can read HTML)
- ✅ Works without JavaScript
- ✅ Initial page load is complete
- ✅ Better for accessibility

**Disadvantages:**
- ❌ Full page reload on navigation
- ❌ Server CPU load (rendering HTML)
- ❌ Larger response size
- ❌ Slower perceived performance

### Current (CSR + API)

**Advantages:**
- ✅ Fast navigation (no page reload)
- ✅ Lower server load (just JSON)
- ✅ Smaller API responses
- ✅ Clear frontend/backend separation
- ✅ Better for SPA experience

**Disadvantages:**
- ❌ No SEO (search engines see empty HTML)
- ❌ Requires JavaScript enabled
- ❌ Initial load includes extra JS library
- ❌ Slower first meaningful paint

---

## 🧪 Technology Stack

### Previous
```
Backend:  Rust + Axum
Markdown: pulldown-cmark (server)
HTML:     Template strings in Rust
CSS:      Inline in templates
JS:       Minimal (fetch for AJAX)
```

### Current
```
Backend:  Rust + Axum (JSON API)
Markdown: marked.js (client, CDN)
HTML:     static/index.html (shell)
CSS:      static/style.css (separate)
JS:       static/app.js (routing + rendering)
```

---

## 📊 Data Flow Example

### Viewing a File

**Previous:**
```
User clicks "test.md"
  ↓
GET /view/test.md
  ↓
Server reads test.md
  ↓
Server: MD → HTML (pulldown-cmark)
  ↓
Server: Wrap in template (html.rs)
  ↓
Response: 50KB HTML + CSS
  ↓
Browser renders
```

**Current:**
```
User clicks "test.md"
  ↓
Hash changes to #/test.md
  ↓
JS: GET /api/markdown/test.md
  ↓
Server reads test.md
  ↓
Response: 2KB JSON { "markdown": "..." }
  ↓
Browser: marked.parse(markdown)
  ↓
JS: contentArea.innerHTML = html
  ↓
DOM updated (no reload)
```

---

## 🚀 Performance Comparison

| Metric | Previous (SSR) | Current (CSR) |
|--------|---------------|--------------|
| **Initial Load** | ~50KB | ~30KB (HTML+CSS+JS) + CDN |
| **Navigation** | Full reload | DOM update only |
| **Server CPU** | High (HTML gen) | Low (JSON only) |
| **Network** | Large HTML | Small JSON |
| **Perceived Speed** | Slower | Faster |

---

## 🔮 Use Cases

### When to use SSR (Previous)
- SEO is critical
- Content needs to be crawled by search engines
- Users may have JavaScript disabled
- Server-side caching is important

### When to use CSR (Current)
- Internal tool (like this viewer)
- SEO not needed
- Want SPA-like experience
- Prefer clear API separation
- **This project fits here** ✓

---

## 💡 Summary

The new architecture transforms this from a traditional server-rendered app to a modern **JAMstack-style** application:

- **J**avaScript: `app.js` + `marked.js`
- **A**PIs: `/api/files` + `/api/markdown/:filename`
- **M**arkup: Static `index.html`

This is ideal for a personal markdown viewer where SEO is unnecessary and a smooth, fast user experience is preferred.
