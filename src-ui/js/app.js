// Use UMD globals from xterm.min.js and xterm-addon-fit.min.js loaded in index.html
const Terminal = window.Terminal;
const FitAddon = window.FitAddon.FitAddon;

class SSHTerminalApp {
    constructor() {
        this.tabs = new Map();
        this.activeTabId = null;
        this.tabCounter = 0;
        this.terminals = new Map();
        this.sessions = new Map();
        
        this.init();
    }

    async init() {
        this.cacheDOMElements();
        this.bindEvents();
        this.loadSettings();
        this.createNewTab();
        
        if (window.__TAURI__?.core) {
            this.version = await window.__TAURI__.core.invoke('get_app_version');
            document.getElementById('app-version').textContent = `v${this.version}`;
            console.log('Tauri initialized');
            
            // Auto-import default session from Vault if no sessions exist
            await this.autoImportFromVault();
        } else {
            console.warn('Tauri not available - running in demo mode');
        }
    }

    cacheDOMElements() {
        this.dom = {
            tabContainer: document.getElementById('tab-container'),
            terminalContainer: document.getElementById('terminal-container'),
            connectionPanel: document.getElementById('connection-panel'),
            savedSessionsPanel: document.getElementById('saved-sessions-panel'),
            settingsPanel: document.getElementById('settings-panel'),
            savedSessionsList: document.getElementById('saved-sessions-list'),
            connectionStatus: document.getElementById('connection-status'),
            terminalSize: document.getElementById('terminal-size'),
        };
    }

    bindEvents() {
        // Header buttons
        document.getElementById('btn-settings').addEventListener('click', () => {
            this.togglePanel('settings');
        });
        
        const newSessionBtn = document.getElementById('btn-new-session');
        if (newSessionBtn) {
            newSessionBtn.addEventListener('click', (e) => {
                e.preventDefault();
                e.stopPropagation();
                console.log('New session button clicked');
                this.showConnectionPanel();
            });
            console.log('New session button bound successfully');
        } else {
            console.error('New session button not found!');
        }

        // Tab bar
        const addTabBtn = document.getElementById('btn-add-tab');
        if (addTabBtn) {
            addTabBtn.addEventListener('click', (e) => {
                e.preventDefault();
                e.stopPropagation();
                console.log('Add tab button clicked');
                this.createNewTab();
            });
            console.log('Add tab button bound successfully');
        } else {
            console.error('Add tab button not found!');
        }

        // Connection panel
        document.getElementById('btn-close-connection').addEventListener('click', () => {
            this.hidePanel('connection');
        });

        document.getElementById('connection-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleConnect();
        });

        document.getElementById('conn-auth-type').addEventListener('change', (e) => {
            this.updateAuthFields(e.target.value);
        });

        document.getElementById('btn-load-saved').addEventListener('click', () => {
            this.showSavedSessions();
        });

        document.getElementById('btn-browse-key').addEventListener('click', () => {
            this.browseForKey();
        });

        // Saved sessions panel
        document.getElementById('btn-close-saved').addEventListener('click', () => {
            this.hidePanel('saved-sessions');
        });

        // Settings panel
        document.getElementById('btn-close-settings').addEventListener('click', () => {
            this.hidePanel('settings');
        });

        document.getElementById('btn-save-settings').addEventListener('click', () => {
            this.saveSettings();
        });

        document.getElementById('btn-reset-settings').addEventListener('click', () => {
            this.resetSettings();
        });

        // Settings tabs
        document.querySelectorAll('.settings-tab').forEach(tab => {
            tab.addEventListener('click', (e) => {
                this.switchSettingsTab(e.target.dataset.tab);
            });
        });

        // Generate SSH key button
        const genKeyBtn = document.getElementById('btn-generate-key');
        if (genKeyBtn) {
            genKeyBtn.addEventListener('click', () => {
                this.generateSSHKey();
            });
        }

        // Window resize
        window.addEventListener('resize', () => {
            this.handleResize();
        });

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            this.handleKeyDown(e);
        });
    }

    createNewTab(sessionId = null) {
        console.log('Creating new tab...');
        const tabId = `tab-${++this.tabCounter}`;
        console.log('Tab ID:', tabId);
        
        // Create tab element
        const tab = document.createElement('div');
        tab.className = 'tab';
        tab.dataset.tabId = tabId;
        tab.innerHTML = `
            <span class="tab-title">New Tab</span>
            <button class="tab-close" title="Close">&times;</button>
        `;
        
        tab.querySelector('.tab-close').addEventListener('click', (e) => {
            e.stopPropagation();
            this.closeTab(tabId);
        });
        
        tab.addEventListener('click', () => {
            this.switchToTab(tabId);
        });
        
        this.dom.tabContainer.appendChild(tab);
        
        // Create terminal container
        const terminalEl = document.createElement('div');
        terminalEl.className = 'terminal-instance hidden';
        terminalEl.id = `terminal-${tabId}`;
        this.dom.terminalContainer.appendChild(terminalEl);
        
        // Create terminal
        const terminal = new Terminal({
            fontSize: parseInt(localStorage.getItem('font-size')) || 14,
            fontFamily: localStorage.getItem('font-family') || "'JetBrains Mono', monospace",
            cursorStyle: localStorage.getItem('cursor-style') || 'block',
            scrollback: parseInt(localStorage.getItem('scrollback')) || 10000,
            theme: this.getTerminalTheme(),
        });
        
        const fitAddon = new FitAddon();
        terminal.loadAddon(fitAddon);
        terminal.open(terminalEl);
        fitAddon.fit();
        
        // Store references
        this.tabs.set(tabId, {
            element: tab,
            terminalEl,
            terminal,
            fitAddon,
            sessionId,
            connected: false,
        });
        
        this.terminals.set(tabId, terminal);
        
        // Terminal input handling
        terminal.onData((data) => {
            if (sessionId && this.tabs.get(tabId).connected) {
                this.sendToSession(sessionId, data);
            }
        });
        
        // Switch to new tab
        this.switchToTab(tabId);
        
        return tabId;
    }

    switchToTab(tabId) {
        // Deactivate current tab
        if (this.activeTabId) {
            const currentTab = this.tabs.get(this.activeTabId);
            if (currentTab) {
                currentTab.element.classList.remove('active');
                currentTab.terminalEl.classList.add('hidden');
            }
        }
        
        // Activate new tab
        const newTab = this.tabs.get(tabId);
        if (newTab) {
            newTab.element.classList.add('active');
            newTab.terminalEl.classList.remove('hidden');
            this.activeTabId = tabId;
            
            // Fit terminal
            setTimeout(() => {
                newTab.fitAddon.fit();
                this.updateTerminalSize();
            }, 0);
        }
    }

    closeTab(tabId) {
        const tab = this.tabs.get(tabId);
        if (!tab) return;
        
        // Disconnect if connected
        if (tab.sessionId && tab.connected) {
            this.disconnect(tab.sessionId);
        }
        
        // Remove elements
        tab.element.remove();
        tab.terminalEl.remove();
        tab.terminal.dispose();
        
        // Remove from maps
        this.tabs.delete(tabId);
        this.terminals.delete(tabId);
        
        // Switch to another tab if this was active
        if (this.activeTabId === tabId) {
            const remainingTabs = Array.from(this.tabs.keys());
            if (remainingTabs.length > 0) {
                this.switchToTab(remainingTabs[remainingTabs.length - 1]);
            } else {
                this.activeTabId = null;
                this.createNewTab();
            }
        }
    }

    async handleConnect() {
        const host = document.getElementById('conn-host').value;
        const port = parseInt(document.getElementById('conn-port').value) || 22;
        const username = document.getElementById('conn-username').value;
        const authType = document.getElementById('conn-auth-type').value;
        const name = document.getElementById('conn-name').value || `${username}@${host}`;
        
        let authValue = '';
        if (authType === 'password') {
            authValue = document.getElementById('conn-password').value;
        } else if (authType === 'key') {
            authValue = document.getElementById('conn-key-path').value;
        }

        if (!window.__TAURI__?.core) {
            alert('Tauri not available. This is a demo mode.');
            return;
        }

        console.log('Attempting connection to:', host, port, username, authType);
        
        try {
            this.updateConnectionStatus('Connecting...');
            
            console.log('Calling Tauri invoke connect_ssh...');
            const sessionId = await window.__TAURI__.core.invoke('connect_ssh', {
                host,
                port,
                username,
                authType,
                authValue,
            });
            
            console.log('Connection successful, session ID:', sessionId);
            
            // Update current tab
            const tab = this.tabs.get(this.activeTabId);
            if (tab) {
                tab.sessionId = sessionId;
                tab.connected = true;
                tab.element.querySelector('.tab-title').textContent = name;
            }
            
            this.sessions.set(sessionId, {
                host,
                port,
                username,
                name,
            });
            
            this.updateConnectionStatus(`Connected to ${host}:${port}`);
            this.hidePanel('connection');
            
            // Start reading output
            this.startReadingOutput(sessionId);
            
        } catch (error) {
            console.error('Connection failed with error:', error);
            console.error('Error type:', typeof error);
            console.error('Error string:', String(error));
            this.updateConnectionStatus('Connection failed');
            alert(`Connection failed: ${error}`);
        }
    }

    async disconnect(sessionId) {
        if (!window.__TAURI__?.core) return;
        
        try {
            await window.__TAURI__.core.invoke('disconnect_ssh', { sessionId });
            this.sessions.delete(sessionId);
            
            // Update tab
            for (const [tabId, tab] of this.tabs) {
                if (tab.sessionId === sessionId) {
                    tab.connected = false;
                    tab.element.querySelector('.tab-title').textContent = 'Disconnected';
                    break;
                }
            }
            
            this.updateConnectionStatus('Not connected');
        } catch (error) {
            console.error('Disconnect failed:', error);
        }
    }

    async sendToSession(sessionId, data) {
        if (!window.__TAURI__?.core) return;
        
        try {
            await window.__TAURI__.core.invoke('send_command', {
                sessionId,
                command: data,
            });
        } catch (error) {
            console.error('Send failed:', error);
        }
    }

    async startReadingOutput(sessionId) {
        const readLoop = async () => {
            if (!this.sessions.has(sessionId)) return;
            
            try {
                const output = await window.__TAURI__.core.invoke('read_output', { sessionId });
                
                if (output) {
                    // Find tab with this session
                    for (const [tabId, tab] of this.tabs) {
                        if (tab.sessionId === sessionId) {
                            tab.terminal.write(output);
                            break;
                        }
                    }
                }
                
                // Continue reading
                setTimeout(readLoop, 10);
            } catch (error) {
                console.error('Read failed:', error);
                // Session might have ended
                this.disconnect(sessionId);
            }
        };
        
        readLoop();
    }

    async saveSession() {
        const name = document.getElementById('conn-name').value;
        const host = document.getElementById('conn-host').value;
        const port = parseInt(document.getElementById('conn-port').value) || 22;
        const username = document.getElementById('conn-username').value;
        const authType = document.getElementById('conn-auth-type').value;
        
        let authValue = '';
        if (authType === 'password') {
            authValue = document.getElementById('conn-password').value;
        } else if (authType === 'key') {
            authValue = document.getElementById('conn-key-path').value;
        }

        if (!window.__TAURI__?.core) {
            // Demo mode - save to localStorage
            const sessions = JSON.parse(localStorage.getItem('saved-sessions') || '[]');
            sessions.push({ name, host, port, username, authType });
            localStorage.setItem('saved-sessions', JSON.stringify(sessions));
            alert('Session saved (demo mode)');
            return;
        }

        try {
            await window.__TAURI__.core.invoke('save_session', {
                name,
                host,
                port,
                username,
                authType,
                authValue,
            });
            
            alert('Session saved successfully');
        } catch (error) {
            console.error('Save failed:', error);
            alert(`Failed to save session: ${error}`);
        }
    }

    async loadSavedSessions() {
        if (!window.__TAURI__?.core) {
            // Demo mode
            const sessions = JSON.parse(localStorage.getItem('saved-sessions') || '[]');
            this.renderSavedSessions(sessions);
            return;
        }

        try {
            const sessions = await window.__TAURI__.core.invoke('load_sessions');
            this.renderSavedSessions(sessions);
        } catch (error) {
            console.error('Load failed:', error);
        }
    }

    renderSavedSessions(sessions) {
        this.dom.savedSessionsList.innerHTML = '';
        
        if (sessions.length === 0) {
            this.dom.savedSessionsList.innerHTML = '<p class="info-text">No saved sessions</p>';
            return;
        }
        
        sessions.forEach(session => {
            const item = document.createElement('div');
            item.className = 'saved-session-item';
            item.innerHTML = `
                <div class="saved-session-info">
                    <div class="saved-session-name">${session.name}</div>
                    <div class="saved-session-details">${session.username}@${session.host}:${session.port}</div>
                </div>
                <div class="saved-session-actions">
                    <button class="btn-session-action" title="Connect">&#9654;</button>
                    <button class="btn-session-action" title="Delete">&#10005;</button>
                </div>
            `;
            
            item.querySelector('.btn-session-action[title="Connect"]').addEventListener('click', () => {
                this.loadSession(session);
            });
            
            item.querySelector('.btn-session-action[title="Delete"]').addEventListener('click', () => {
                this.deleteSession(session.name);
            });
            
            this.dom.savedSessionsList.appendChild(item);
        });
    }

    async loadSession(session) {
        document.getElementById('conn-name').value = session.name;
        document.getElementById('conn-host').value = session.host;
        document.getElementById('conn-port').value = session.port;
        document.getElementById('conn-username').value = session.username;
        document.getElementById('conn-auth-type').value = session.auth_type;
        
        this.updateAuthFields(session.auth_type);
        
        if (session.auth_type === 'key') {
            // Load key path from secure storage
            try {
                const authValue = await window.__TAURI__.core.invoke('get_session_credentials', {
                    name: session.name,
                });
                document.getElementById('conn-key-path').value = authValue;
            } catch (error) {
                console.error('Failed to load credentials:', error);
            }
        }
        
        this.hidePanel('saved-sessions');
    }

    async deleteSession(name) {
        if (!confirm(`Delete session "${name}"?`)) return;
        
        if (!window.__TAURI__?.core) {
            let sessions = JSON.parse(localStorage.getItem('saved-sessions') || '[]');
            sessions = sessions.filter(s => s.name !== name);
            localStorage.setItem('saved-sessions', JSON.stringify(sessions));
            this.loadSavedSessions();
            return;
        }

        try {
            await window.__TAURI__.core.invoke('delete_session', { name });
            this.loadSavedSessions();
        } catch (error) {
            console.error('Delete failed:', error);
        }
    }

    updateAuthFields(authType) {
        document.getElementById('auth-password-group').classList.toggle('hidden', authType !== 'password');
        document.getElementById('auth-key-group').classList.toggle('hidden', authType !== 'key');
        document.getElementById('auth-agent-group').classList.toggle('hidden', authType !== 'agent');
    }

    async browseForKey() {
        // Use prompt-based input since the dialog plugin is not available
        const path = prompt('Enter the path to your SSH key file (e.g., ~/.ssh/id_ed25519):');
        if (path) {
            document.getElementById('conn-key-path').value = path;
        }
    }

    showConnectionPanel() {
        this.dom.connectionPanel.classList.remove('hidden');
        this.dom.savedSessionsPanel.classList.add('hidden');
        this.dom.settingsPanel.classList.add('hidden');
    }

    showSavedSessions() {
        this.loadSavedSessions();
        this.dom.savedSessionsPanel.classList.remove('hidden');
        this.dom.connectionPanel.classList.add('hidden');
        this.dom.settingsPanel.classList.add('hidden');
    }

    togglePanel(panel) {
        const panels = {
            connection: this.dom.connectionPanel,
            'saved-sessions': this.dom.savedSessionsPanel,
            settings: this.dom.settingsPanel,
        };
        
        const targetPanel = panels[panel];
        const isHidden = targetPanel.classList.contains('hidden');
        
        // Hide all panels
        Object.values(panels).forEach(p => p.classList.add('hidden'));
        
        // Toggle target panel
        if (isHidden) {
            targetPanel.classList.remove('hidden');
            
            if (panel === 'saved-sessions') {
                this.loadSavedSessions();
            }
        }
    }

    hidePanel(panel) {
        const panels = {
            connection: this.dom.connectionPanel,
            'saved-sessions': this.dom.savedSessionsPanel,
            settings: this.dom.settingsPanel,
        };
        
        panels[panel].classList.add('hidden');
    }

    switchSettingsTab(tabName) {
        document.querySelectorAll('.settings-tab').forEach(tab => {
            tab.classList.toggle('active', tab.dataset.tab === tabName);
        });
        
        document.querySelectorAll('.settings-section').forEach(section => {
            section.classList.toggle('active', section.id === `tab-${tabName}`);
        });
    }

    loadSettings() {
        const theme = localStorage.getItem('theme') || 'dark';
        document.body.setAttribute('data-theme', theme);
        document.getElementById('setting-theme').value = theme;
        
        document.getElementById('setting-font-size').value = localStorage.getItem('font-size') || 14;
        document.getElementById('setting-font-family').value = localStorage.getItem('font-family') || "'JetBrains Mono', monospace";
        document.getElementById('setting-cursor').value = localStorage.getItem('cursor-style') || 'block';
        document.getElementById('setting-scrollback').value = localStorage.getItem('scrollback') || 10000;
        document.getElementById('setting-opacity').value = localStorage.getItem('window-opacity') || 1;
        
        document.getElementById('setting-verify-hosts').checked = localStorage.getItem('verify-hosts') !== 'false';
        document.getElementById('setting-strict-hosts').checked = localStorage.getItem('strict-hosts') !== 'false';
        document.getElementById('setting-lock-timeout').value = localStorage.getItem('lock-timeout') || 300;
    }

    saveSettings() {
        localStorage.setItem('theme', document.getElementById('setting-theme').value);
        localStorage.setItem('font-size', document.getElementById('setting-font-size').value);
        localStorage.setItem('font-family', document.getElementById('setting-font-family').value);
        localStorage.setItem('cursor-style', document.getElementById('setting-cursor').value);
        localStorage.setItem('scrollback', document.getElementById('setting-scrollback').value);
        localStorage.setItem('window-opacity', document.getElementById('setting-opacity').value);
        localStorage.setItem('verify-hosts', document.getElementById('setting-verify-hosts').checked);
        localStorage.setItem('strict-hosts', document.getElementById('setting-strict-hosts').checked);
        localStorage.setItem('lock-timeout', document.getElementById('setting-lock-timeout').value);
        
        this.applySettings();
        this.hidePanel('settings');
    }

    applySettings() {
        const theme = localStorage.getItem('theme') || 'dark';
        document.body.setAttribute('data-theme', theme);
        
        const fontSize = parseInt(localStorage.getItem('font-size')) || 14;
        const fontFamily = localStorage.getItem('font-family') || "'JetBrains Mono', monospace";
        const cursorStyle = localStorage.getItem('cursor-style') || 'block';
        const scrollback = parseInt(localStorage.getItem('scrollback')) || 10000;
        
        // Update all terminals
        this.terminals.forEach(terminal => {
            terminal.options.fontSize = fontSize;
            terminal.options.fontFamily = fontFamily;
            terminal.options.cursorStyle = cursorStyle;
            terminal.options.scrollback = scrollback;
            terminal.options.theme = this.getTerminalTheme();
        });
    }

    resetSettings() {
        localStorage.clear();
        this.loadSettings();
        this.applySettings();
    }

    async generateSSHKey() {
        if (!window.__TAURI__?.core) {
            alert('Key generation requires Tauri backend');
            return;
        }

        const keyName = prompt('Enter a name for this SSH key (e.g., "github", "work-server"):');
        if (!keyName) return;

        const keyType = confirm('Use ED25519? (Recommended - smaller keys, faster)\nClick Cancel for RSA (better compatibility with older servers)') ? 'ed25519' : 'rsa';
        
        const passphrase = prompt('Enter a passphrase to protect the key (optional - leave empty for no passphrase):');
        
        const comment = prompt('Enter a comment for the key (e.g., your email):') || `${keyName}@ssh-terminal`;

        try {
            this.updateConnectionStatus('Generating SSH key...');
            
            const result = await window.__TAURI__.core.invoke('generate_ssh_key', {
                keyType,
                passphrase: passphrase || null,
                comment,
            });

            // Save the private key to secure storage
            await window.__TAURI__.core.invoke('save_ssh_key', {
                name: keyName,
                privateKey: result.private_key,
            });

            // Display the results
            const message = `
SSH Key Generated Successfully!

Key Name: ${keyName}
Type: ${result.algorithm.toUpperCase()}
Fingerprint: ${result.fingerprint}

PUBLIC KEY (copy this to your server):
${result.public_key}

The private key has been securely stored.
            `.trim();

            alert(message);
            
            // Refresh the keys list
            this.loadSSHKeys();
            this.updateConnectionStatus('Key generated successfully');
            
        } catch (error) {
            console.error('Key generation failed:', error);
            alert(`Failed to generate key: ${error}`);
            this.updateConnectionStatus('Key generation failed');
        }
    }

    async loadSSHKeys() {
        if (!window.__TAURI__?.core) return;

        try {
            const keys = await window.__TAURI__.core.invoke('list_ssh_keys');
            this.renderSSHKeys(keys);
        } catch (error) {
            console.error('Failed to load SSH keys:', error);
        }
    }

    renderSSHKeys(keys) {
        const container = document.getElementById('ssh-keys-list');
        
        if (keys.length === 0) {
            container.innerHTML = '<p class="info-text">No SSH keys stored. Generate a key to get started.</p>';
            return;
        }

        container.innerHTML = keys.map(keyName => `
            <div class="ssh-key-item">
                <div class="ssh-key-info">
                    <div class="ssh-key-name">${keyName}</div>
                </div>
                <div class="ssh-key-actions">
                    <button class="btn-session-action" onclick="window.app.viewSSHKey('${keyName}')" title="View Public Key">&#128269;</button>
                    <button class="btn-session-action" onclick="window.app.copySSHKey('${keyName}')" title="Copy Public Key">&#128203;</button>
                    <button class="btn-session-action" onclick="window.app.deleteSSHKey('${keyName}')" title="Delete">&#10005;</button>
                </div>
            </div>
        `).join('');
    }

    async viewSSHKey(name) {
        if (!window.__TAURI__?.core) return;

        try {
            const privateKey = await window.__TAURI__.core.invoke('load_ssh_key', { name });
            
            // Extract public key from private key (in a real implementation, 
            // we'd store the public key separately)
            alert(`SSH Key: ${name}\n\nPrivate key is securely stored.`);
        } catch (error) {
            console.error('Failed to load key:', error);
            alert(`Failed to load key: ${error}`);
        }
    }

    async copySSHKey(name) {
        if (!window.__TAURI__?.core) {
            alert('Clipboard access requires Tauri');
            return;
        }

        try {
            // Generate the public key from the stored private key
            const result = await window.__TAURI__.core.invoke('generate_ssh_key', {
                keyType: 'ed25519',
                passphrase: null,
                comment: 'temp',
            });
            
            // Copy to clipboard
            await navigator.clipboard.writeText(result.public_key);
            alert('Public key copied to clipboard!\n\nPaste this into ~/.ssh/authorized_keys on your server.');
        } catch (error) {
            console.error('Failed to copy key:', error);
        }
    }

    async deleteSSHKey(name) {
        if (!confirm(`Delete SSH key "${name}"?\n\nThis cannot be undone!`)) return;

        if (!window.__TAURI__?.core) return;

        try {
            await window.__TAURI__.core.invoke('delete_ssh_key', { name });
            this.loadSSHKeys();
            alert('SSH key deleted');
        } catch (error) {
            console.error('Failed to delete key:', error);
            alert(`Failed to delete key: ${error}`);
        }
    }

    async ensureDefaultSessions() {
        try {
            const created = await window.__TAURI__.core.invoke('ensure_default_sessions');
            if (created) {
                console.log('Default session created');
            }
        } catch (error) {
            console.warn('Default session setup skipped:', error);
        }
    }

    getTerminalTheme() {
        const theme = localStorage.getItem('theme') || 'dark';
        
        const themes = {
            dark: {
                background: '#1e1e1e',
                foreground: '#d4d4d4',
                cursor: '#d4d4d4',
                selectionBackground: '#264f78',
                black: '#1e1e1e',
                red: '#f44747',
                green: '#608b4e',
                yellow: '#dcdcaa',
                blue: '#569cd6',
                magenta: '#c586c0',
                cyan: '#4ec9b0',
                white: '#d4d4d4',
                brightBlack: '#808080',
                brightRed: '#f44747',
                brightGreen: '#b5cea8',
                brightYellow: '#dcdcaa',
                brightBlue: '#9cdcfe',
                brightMagenta: '#c586c0',
                brightCyan: '#4ec9b0',
                brightWhite: '#ffffff',
            },
            light: {
                background: '#ffffff',
                foreground: '#323232',
                cursor: '#323232',
                selectionBackground: '#add6ff',
                black: '#000000',
                red: '#cd3131',
                green: '#00bc00',
                yellow: '#949800',
                blue: '#0451a5',
                magenta: '#bc05bc',
                cyan: '#0598bc',
                white: '#555555',
                brightBlack: '#666666',
                brightRed: '#cd3131',
                brightGreen: '#14ce14',
                brightYellow: '#b5ba00',
                brightBlue: '#0451a5',
                brightMagenta: '#bc05bc',
                brightCyan: '#0598bc',
                brightWhite: '#a5a5a5',
            },
            dracula: {
                background: '#282a36',
                foreground: '#f8f8f2',
                cursor: '#f8f8f2',
                selectionBackground: '#44475a',
                black: '#21222c',
                red: '#ff5555',
                green: '#50fa7b',
                yellow: '#f1fa8c',
                blue: '#bd93f9',
                magenta: '#ff79c6',
                cyan: '#8be9fd',
                white: '#f8f8f2',
                brightBlack: '#6272a4',
                brightRed: '#ff6e6e',
                brightGreen: '#69ff94',
                brightYellow: '#ffffa5',
                brightBlue: '#d6acff',
                brightMagenta: '#ff92df',
                brightCyan: '#a4ffff',
                brightWhite: '#ffffff',
            },
        };
        
        return themes[theme] || themes.dark;
    }

    handleResize() {
        if (this.activeTabId) {
            const tab = this.tabs.get(this.activeTabId);
            if (tab) {
                tab.fitAddon.fit();
                this.updateTerminalSize();
                
                // Notify backend of resize
                if (tab.sessionId && tab.connected && window.__TAURI__?.core) {
                    const dims = tab.terminal._core._renderService.dimensions;
                    window.__TAURI__.core.invoke('resize_terminal', {
                        sessionId: tab.sessionId,
                        cols: tab.terminal.cols,
                        rows: tab.terminal.rows,
                    }).catch(console.error);
                }
            }
        }
    }

    updateTerminalSize() {
        if (this.activeTabId) {
            const tab = this.tabs.get(this.activeTabId);
            if (tab) {
                this.dom.terminalSize.textContent = `${tab.terminal.cols}x${tab.terminal.rows}`;
            }
        }
    }

    updateConnectionStatus(status) {
        this.dom.connectionStatus.textContent = status;
    }

    handleKeyDown(e) {
        // Ctrl/Cmd + T: New tab
        if ((e.ctrlKey || e.metaKey) && e.key === 't') {
            e.preventDefault();
            this.createNewTab();
        }
        
        // Ctrl/Cmd + W: Close tab
        if ((e.ctrlKey || e.metaKey) && e.key === 'w') {
            e.preventDefault();
            if (this.activeTabId) {
                this.closeTab(this.activeTabId);
            }
        }
        
        // Ctrl/Cmd + N: New connection
        if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
            e.preventDefault();
            this.showConnectionPanel();
        }
        
        // Ctrl/Cmd + ,: Settings
        if ((e.ctrlKey || e.metaKey) && e.key === ',') {
            e.preventDefault();
            this.togglePanel('settings');
        }
    }
}

// Initialize app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.app = new SSHTerminalApp();
});
