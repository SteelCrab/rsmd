//! Simple dynamic loading functionality without external dependencies

/// Generate inline JavaScript for dynamic content loading (replaces HTMX)
pub fn dynamic_script() -> &'static str {
    r#"<script>
// Simple fetch-based dynamic loading (no external dependencies)
document.addEventListener('DOMContentLoaded', function() {
    const fileListContainer = document.querySelector('.file-list');
    const emptyState = document.getElementById('empty-state');
    const uploadArea = document.getElementById('upload-area');
    const directoryBody = document.querySelector('.directory-body');
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

    const toViewHref = (name) => {
        if (!name) return '/view/';
        return '/view/' + name.split('/').map(part => encodeURIComponent(part)).join('/');
    };

    const refreshFileList = (activeFile) => {
        if (!fileListContainer) return Promise.resolve();
        const currentPath = uploadArea.dataset.currentPath || '';
        uploadArea.dataset.currentPath = currentPath;
        if (directoryBody) {
            directoryBody.setAttribute('data-current-path', currentPath);
        }
        const prefix = currentPath ? currentPath + '/' : '';
        return fetch('/api/files')
            .then(r => r.json())
            .then(data => {
                if (!data || !Array.isArray(data.files)) return;

                const items = data.files
                    .filter(name => {
                        if (currentPath) {
                            if (!name.startsWith(prefix)) return false;
                            return !name.slice(prefix.length).includes('/');
                        }
                        return !name.includes('/');
                    })
                    .map(name => {
                        const displayName = currentPath ? name.slice(prefix.length) : name;
                        const shortName = displayName.split('/').pop() || displayName;
                        const href = escapeHtml(toViewHref(name));
                        const display = escapeHtml(shortName);
                        const pathLabel = escapeHtml(name);
                        return `<a class="file-entry" href="${href}">
    <span class="file-entry__icon">📄</span>
    <span class="file-entry__text">
        <span class="file-entry__name">${display}</span>
        <span class="file-entry__path">/${pathLabel}</span>
    </span>
    <span class="file-entry__arrow">→</span>
</a>`;
                    })
                    .join('');

                fileListContainer.innerHTML = items;

                if (emptyState) {
                    const hasFolders = !!document.querySelector('.folder-grid .folder-card');
                    if (items.trim().length === 0 && !hasFolders) {
                        emptyState.classList.remove('hidden');
                    } else {
                        emptyState.classList.add('hidden');
                    }
                }

                if (activeFile) {
                    window.location.href = toViewHref(activeFile);
                }
            })
            .catch(() => {});
    };

    const handleUpload = (file) => {
        if (!file) return;
        const currentPath = uploadArea.dataset.currentPath || '';
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
                'Content-Type': 'application/octet-stream',
                'X-Directory-Path': currentPath
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

    document.addEventListener('click', (event) => {
        const card = event.target.closest('.folder-card');
        if (!card || !directoryBody) return;
        const folderPath = card.dataset.path || '';
        directoryBody.setAttribute('data-current-path', folderPath);
        uploadArea.dataset.currentPath = folderPath;
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
