// Markdown Viewer - Client-side App with Hash Routing

const API_BASE = '/api';
let currentFiles = [];

// Initialize marked.js options
marked.setOptions({
    breaks: true,
    gfm: true,
    headerIds: true,
    mangle: false
});

// Initialize app
async function init() {
    await loadFiles();
    setupHashRouting();
    handleHashChange();
}

// Load file list from API
async function loadFiles() {
    const loadingEl = document.getElementById('loading');
    const fileListEl = document.getElementById('file-list');

    try {
        const response = await fetch(`${API_BASE}/files`);
        if (!response.ok) throw new Error('Failed to load files');

        const data = await response.json();
        currentFiles = data.files || [];

        loadingEl.style.display = 'none';

        if (currentFiles.length === 0) {
            fileListEl.innerHTML = '<li class="no-files">No markdown files found</li>';
            return;
        }

        fileListEl.innerHTML = currentFiles.map(file => `
            <li>
                <a href="#/${file}" class="file-link">${escapeHtml(file)}</a>
            </li>
        `).join('');

        // Update directory info
        const dirInfo = document.getElementById('directory-info');
        dirInfo.textContent = `Found ${currentFiles.length} markdown file(s)`;

    } catch (error) {
        loadingEl.innerHTML = `<div class="error">Error: ${error.message}</div>`;
    }
}

// Load and render markdown file
async function loadMarkdownFile(filename) {
    const contentArea = document.getElementById('content-area');

    // Show loading state
    contentArea.innerHTML = '<div class="loading">Loading...</div>';
    contentArea.style.opacity = '0.6';

    try {
        const response = await fetch(`${API_BASE}/markdown/${encodeURIComponent(filename)}`);
        if (!response.ok) throw new Error('File not found');

        const data = await response.json();
        const markdown = data.markdown || '';

        // Convert markdown to HTML using marked.js
        const html = marked.parse(markdown);

        // Render content
        contentArea.innerHTML = html;
        contentArea.style.opacity = '1';

        // Update page title
        const h1 = contentArea.querySelector('h1');
        if (h1) {
            document.title = `${h1.textContent} - Markdown Viewer`;
        }

        // Highlight active file in sidebar
        updateActiveFile(filename);

    } catch (error) {
        contentArea.innerHTML = `<div class="error">
            <h2>Error</h2>
            <p>${error.message}</p>
        </div>`;
        contentArea.style.opacity = '1';
    }
}

// Update active file highlighting
function updateActiveFile(filename) {
    document.querySelectorAll('.file-link').forEach(link => {
        link.classList.remove('active');
        if (link.textContent.trim() === filename) {
            link.classList.add('active');
        }
    });
}

// Hash routing setup
function setupHashRouting() {
    window.addEventListener('hashchange', handleHashChange);
}

function handleHashChange() {
    const hash = window.location.hash;

    if (hash && hash.startsWith('#/')) {
        const filename = decodeURIComponent(hash.substring(2));
        loadMarkdownFile(filename);
    } else {
        // Show empty state
        const contentArea = document.getElementById('content-area');
        contentArea.innerHTML = '<div class="empty-state">← Select a file to view</div>';

        // Clear active states
        document.querySelectorAll('.file-link').forEach(link => {
            link.classList.remove('active');
        });
    }
}

// Utility: Escape HTML
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Start app when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
