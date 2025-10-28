//! Simple dynamic loading functionality without external dependencies

/// Generate inline JavaScript for dynamic content loading (replaces HTMX)
pub fn dynamic_script() -> &'static str {
    r#"<script>
// Simple fetch-based dynamic loading (no external dependencies)
document.addEventListener('DOMContentLoaded', function() {
    const contentArea = document.getElementById('content-area');
    const fileListContainer = document.querySelector('.file-list');
    const emptyState = document.getElementById('empty-state');

    const setActiveLink = (link) => {
        document.querySelectorAll('.file-list li.active').forEach(li => li.classList.remove('active'));
        if (link && link.parentElement) {
            link.parentElement.classList.add('active');
        }
    };

    const loadContent = (url, href) => {
        if (!contentArea || !url) return;
        contentArea.style.opacity = '0.6';

        fetch(url, {
            headers: {
                'X-Requested-With': 'XMLHttpRequest'
            }
        })
            .then(r => r.text())
            .then(html => {
                contentArea.innerHTML = html;
                contentArea.style.opacity = '1';

                if (href) {
                    history.pushState({}, '', href);
                }

                const h1 = contentArea.querySelector('h1');
                if (h1) document.title = h1.textContent;
            })
            .catch(() => {
                contentArea.innerHTML = '<p style="color: #ef4444;">Error loading file</p>';
                contentArea.style.opacity = '1';
            });
    };

    document.addEventListener('click', function(event) {
        const link = event.target.closest('a[data-load]');
        if (!link) return;

        event.preventDefault();
        setActiveLink(link);
        loadContent(link.getAttribute('data-load'), link.getAttribute('href'));
    });

    window.addEventListener('popstate', function() {
        location.reload();
    });

    const uploadArea = document.getElementById('upload-area');
    if (!uploadArea) {
        return;
    }

    const statusEl = document.getElementById('upload-status');
    const browseButton = document.getElementById('upload-browse');
    const fileInput = document.getElementById('file-input');

    const setStatus = (message, state) => {
        if (!statusEl) return;
        statusEl.textContent = message || '';
        statusEl.className = 'upload-status';
        if (state) {
            statusEl.classList.add(state);
        }
    };

    const escapeHtml = (value) => {
        if (value == null) return '';
        return value
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#x27;');
    };

    const refreshFileList = (activeFile) => {
        if (!fileListContainer) return Promise.resolve();
        return fetch('/api/files')
            .then(r => r.json())
            .then(data => {
                if (!data || !Array.isArray(data.files)) return;

                const items = data.files.map(name => {
                    const display = escapeHtml(name);
                    return `<li><a href="/view/${display}" data-load="/api/content/${display}">${display}</a></li>`;
                }).join('');

                fileListContainer.innerHTML = items;

                if (emptyState) {
                    if (data.files.length === 0) {
                        emptyState.classList.remove('hidden');
                    } else {
                        emptyState.classList.add('hidden');
                    }
                }

                if (activeFile) {
                    const target = Array.from(fileListContainer.querySelectorAll('a[data-load]'))
                        .find(link => link.getAttribute('data-load') === `/api/content/${activeFile}`);
                    if (target) {
                        setActiveLink(target);
                        target.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
                        loadContent(target.getAttribute('data-load'), target.getAttribute('href'));
                    }
                }
            })
            .catch(() => {});
    };

    const handleUpload = (file) => {
        if (!file) return;
        const name = file.name.toLowerCase();
        if (!(name.endsWith('.md') || name.endsWith('.markdown'))) {
            setStatus(uploadArea.dataset.invalid || 'Invalid file type', 'error');
            return;
        }

        uploadArea.classList.add('uploading');
        setStatus(uploadArea.dataset.uploading || 'Uploading…', 'info');

        fetch('/api/upload', {
            method: 'POST',
            body: file,
            headers: {
                'X-File-Name': file.name,
                'Content-Type': 'application/octet-stream'
            }
        })
            .then(async response => {
                let data = {};
                try {
                    data = await response.json();
                } catch (_) {
                    data = {};
                }

                uploadArea.classList.remove('uploading');

                if (response.ok && data && data.success) {
                    setStatus(data.message || uploadArea.dataset.success || '', 'success');
                    const uploadedFile = data.file;
                    refreshFileList(uploadedFile);
                } else {
                    setStatus((data && data.message) || uploadArea.dataset.error || 'Upload failed', 'error');
                }
            })
            .catch(() => {
                uploadArea.classList.remove('uploading');
                setStatus(uploadArea.dataset.error || 'Upload failed', 'error');
            });
    };

    if (browseButton && fileInput) {
        browseButton.addEventListener('click', () => fileInput.click());
        fileInput.addEventListener('change', (event) => {
            const target = event.target;
            const file = target.files && target.files[0];
            handleUpload(file);
            target.value = '';
        });
    }

    uploadArea.addEventListener('dragover', (event) => {
        event.preventDefault();
        uploadArea.classList.add('dragover');
    });

    ['dragleave', 'dragend'].forEach(type => {
        uploadArea.addEventListener(type, (event) => {
            event.preventDefault();
            uploadArea.classList.remove('dragover');
        });
    });

    uploadArea.addEventListener('drop', (event) => {
        event.preventDefault();
        uploadArea.classList.remove('dragover');
        const file = event.dataTransfer && event.dataTransfer.files && event.dataTransfer.files[0];
        handleUpload(file);
    });
});
</script>"#
}

/// Render content-only HTML (for HTMX partial updates)
pub fn render_partial_content(html_content: &str) -> String {
    html_content.to_string()
}

/// Check if request is from dynamic loading (AJAX/fetch)
pub fn is_dynamic_request(hx_request_header: Option<&str>, xhr_header: Option<&str>) -> bool {
    // Check for HTMX header (legacy)
    if hx_request_header.is_some_and(|v| v == "true") {
        return true;
    }
    // Check for X-Requested-With header (vanilla JS fetch)
    xhr_header.is_some_and(|v| v == "XMLHttpRequest")
}
