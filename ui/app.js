/* HopCraft Frontend Controller (Tauri v2 + Native Desktop + Internal Updater) */

(() => {
  const invoke = window.__TAURI__?.core?.invoke;
  const listenEvent = window.__TAURI__?.event?.listen;
  const STORAGE_KEY = "hopcraft_settings_v1";

  const state = {
    lang: "en",
    theme: "dark",
    includePrerelease: false,
    appInfo: {
      version: "1.0.0",
      arch: "x64",
      is_installed: false,
      repo_url: "https://github.com/HosseinGh23/hopcraft"
    },
    updateData: null,
    isCheckingUpdate: false,
    isDownloadingUpdate: false,
    lastJson: "",
    lastLines: 0,
    lastBytes: 0,
    lastExitIp: ""
  };

  const $ = (id) => document.getElementById(id);

  const el = {
    appVersionPill: $("app-version-pill"),
    subtitle: $("subtitle"),
    pillHop1: $("pill-hop1"),
    pillHop2: $("pill-hop2"),
    btnGithub: $("btn-github"),
    btnUpdate: $("btn-update"),
    updateDot: $("update-dot"),
    updatePanel: $("update-panel"),
    lblUpdateTitle: $("lbl-update-title"),
    updateModePill: $("update-mode-pill"),
    chkPrerelease: $("chk-prerelease"),
    lblPrerelease: $("lbl-prerelease"),
    updateStatusText: $("update-status-text"),
    updateVersionRow: $("update-version-row"),
    updateNewVer: $("update-new-ver"),
    updatePreBadge: $("update-pre-badge"),
    updateProgressWrap: $("update-progress-wrap"),
    updateProgressBar: $("update-progress-bar"),
    updateProgressText: $("update-progress-text"),
    btnCheckNow: $("btn-check-now"),
    lblCheckNow: $("lbl-check-now"),
    btnInstallNow: $("btn-install-now"),
    lblInstallNow: $("lbl-install-now"),
    btnLang: $("btn-lang"),
    langMenu: $("lang-menu"),
    btnTheme: $("btn-theme"),
    iconSun: $("icon-sun"),
    iconMoon: $("icon-moon"),
    chainSectionTitle: $("chain-section-title"),
    btnSwap: $("btn-swap"),
    lblSwap: $("lbl-swap"),
    btnClear: $("btn-clear"),
    lblClear: $("lbl-clear"),
    lblHop1: $("lbl-hop1"),
    hintHop1: $("hint-hop1"),
    hop1Input: $("hop1-input"),
    btnClearHop1: $("btn-clear-hop1"),
    btnPasteHop1: $("btn-paste-hop1"),
    lblHop2: $("lbl-hop2"),
    badgeExit: $("badge-exit"),
    hintHop2: $("hint-hop2"),
    hop2Input: $("hop2-input"),
    btnClearHop2: $("btn-clear-hop2"),
    btnPasteHop2: $("btn-paste-hop2"),
    chkMixed: $("chk-mixed"),
    lblSwitchMode: $("lbl-switch-mode"),
    mixedGroup: $("mixed-group"),
    splitGroup: $("split-group"),
    lblPort: $("lbl-port"),
    mixedPort: $("mixed-port"),
    chkSocks: $("chk-socks"),
    socksPort: $("socks-port"),
    chkHttp: $("chk-http"),
    httpPort: $("http-port"),
    lblListen: $("lbl-listen"),
    listenInput: $("listen-input"),
    btnGenerate: $("btn-generate"),
    lblGenerate: $("lbl-generate"),
    btnCopy: $("btn-copy"),
    lblCopy: $("lbl-copy"),
    btnSave: $("btn-save"),
    lblSave: $("lbl-save"),
    modeBadge: $("mode-badge"),
    outputSectionTitle: $("output-section-title"),
    metaLabel: $("meta-label"),
    jsonOutput: $("json-output"),
    toastContainer: $("toast-container")
  };

  // --- Native Desktop Behavior Guards ---
  document.addEventListener("contextmenu", (e) => {
    const target = e.target;
    const isInput = target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA");
    const isCode = target && target.closest && target.closest(".code-view");
    if (!isInput && !isCode) e.preventDefault();
  });

  document.addEventListener("keydown", (e) => {
    const ctrl = e.ctrlKey || e.metaKey;
    const key = e.key.toLowerCase();

    if (
      e.key === "F5" ||
      e.key === "F12" ||
      (ctrl && (key === "r" || key === "p" || key === "f" || key === "g" || key === "u" || key === "s" || key === "+" || key === "-" || key === "=" || key === "0")) ||
      (e.altKey && (e.key === "ArrowLeft" || e.key === "ArrowRight"))
    ) {
      e.preventDefault();
      if (ctrl && key === "s") saveJson();
      return;
    }

    if (ctrl && e.key === "Enter") {
      e.preventDefault();
      generateJson(true);
    }
  });

  document.addEventListener(
    "wheel",
    (e) => {
      if (e.ctrlKey) e.preventDefault();
    },
    { passive: false }
  );

  function tr(key, vars = {}) {
    const dict = window.HOPCRAFT_I18N[state.lang] || window.HOPCRAFT_I18N.en;
    let text = dict[key] ?? window.HOPCRAFT_I18N.en[key] ?? key;
    for (const [k, v] of Object.entries(vars)) {
      text = text.replaceAll(`{${k}}`, String(v));
    }
    return text;
  }

  function escapeHtml(str) {
    return str
      .replaceAll("&", "&amp;")
      .replaceAll("<", "&lt;")
      .replaceAll(">", "&gt;");
  }

  function highlightJson(jsonText) {
    const escaped = escapeHtml(jsonText);
    const tokenRe =
      /("(?:\\.|[^"\\])*")(?=\s*:)|("(?:\\.|[^"\\])*")|(-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)|\b(true|false|null)\b|([{}\[\]:,])/g;
    return escaped.replace(tokenRe, (m, key, str, num, bool, punct) => {
      if (key) return `<span class="tok-key">${key}</span>`;
      if (str) return `<span class="tok-str">${str}</span>`;
      if (num) return `<span class="tok-num">${num}</span>`;
      if (bool) return `<span class="tok-bool">${bool}</span>`;
      if (punct) return `<span class="tok-punct">${punct}</span>`;
      return m;
    });
  }

  function formatMB(bytes) {
    return (bytes / (1024 * 1024)).toFixed(2);
  }

  function showToast(title, body, isError = false) {
    const node = document.createElement("div");
    node.className = `toast ${isError ? "toast-err" : "toast-ok"}`;
    node.innerHTML = `
      <div>
        <div class="toast-title">${escapeHtml(title)}</div>
        <div class="toast-body">${escapeHtml(body)}</div>
      </div>
    `;
    el.toastContainer.appendChild(node);
    setTimeout(() => {
      node.style.opacity = "0";
      node.style.transition = "opacity 0.2s ease";
      setTimeout(() => node.remove(), 220);
    }, 2600);
  }

  function loadSettings() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (!raw) return;
      const data = JSON.parse(raw);
      if (data.lang && window.HOPCRAFT_I18N[data.lang]) state.lang = data.lang;
      if (data.theme === "light" || data.theme === "dark") state.theme = data.theme;
      if (typeof data.includePrerelease === "boolean") {
        state.includePrerelease = data.includePrerelease;
      }
      if (typeof data.mixedMode === "boolean") el.chkMixed.checked = data.mixedMode;
      if (data.mixedPort) el.mixedPort.value = String(data.mixedPort);
      if (typeof data.useSocks === "boolean") el.chkSocks.checked = data.useSocks;
      if (data.socksPort) el.socksPort.value = String(data.socksPort);
      if (typeof data.useHttp === "boolean") el.chkHttp.checked = data.useHttp;
      if (data.httpPort) el.httpPort.value = String(data.httpPort);
      if (data.listen) el.listenInput.value = String(data.listen);
    } catch (_) {}
    el.chkPrerelease.checked = state.includePrerelease;
  }

  function saveSettings() {
    try {
      localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({
          lang: state.lang,
          theme: state.theme,
          includePrerelease: state.includePrerelease,
          mixedMode: el.chkMixed.checked,
          mixedPort: el.mixedPort.value.trim(),
          useSocks: el.chkSocks.checked,
          socksPort: el.socksPort.value.trim(),
          useHttp: el.chkHttp.checked,
          httpPort: el.httpPort.value.trim(),
          listen: el.listenInput.value.trim() || "127.0.0.1"
        })
      );
    } catch (_) {}
  }

  function applyTheme() {
    document.documentElement.setAttribute("data-theme", state.theme);
    el.iconSun.classList.toggle("hidden", state.theme !== "dark");
    el.iconMoon.classList.toggle("hidden", state.theme === "dark");
    if (invoke) {
      invoke("set_window_theme", { theme: state.theme }).catch(() => {});
    }
  }

  function renderLanguageMenu() {
    el.langMenu.innerHTML = "";
    for (const item of window.HOPCRAFT_LANGUAGES) {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = `dropdown-item ${item.code === state.lang ? "active" : ""}`;
      btn.innerHTML = `<span>${item.name}</span><span>${item.code === state.lang ? "✓" : ""}</span>`;
      btn.addEventListener("click", () => {
        state.lang = item.code;
        el.langMenu.classList.add("hidden");
        applyLanguage();
        saveSettings();
      });
      el.langMenu.appendChild(btn);
    }
  }

  function updateModePillText() {
    const modeLabel = state.appInfo.is_installed
      ? tr("update_mode_installed")
      : tr("update_mode_portable");
    el.updateModePill.textContent = `${state.appInfo.arch} • ${modeLabel}`;
    el.appVersionPill.textContent = `v${state.appInfo.version}`;
  }

  function applyLanguage() {
    const langMeta =
      window.HOPCRAFT_LANGUAGES.find((l) => l.code === state.lang) ||
      window.HOPCRAFT_LANGUAGES[0];
    document.documentElement.lang = langMeta.code;
    document.documentElement.dir = langMeta.dir;

    const title = tr("window_title");
    document.title = title;
    if (invoke) {
      invoke("set_window_title", { title }).catch(() => {});
    }

    el.subtitle.textContent = tr("subtitle");
    el.btnGithub.title = tr("github_tooltip");
    el.btnUpdate.title = tr("update_tooltip");
    el.lblUpdateTitle.textContent = tr("update_title");
    el.lblPrerelease.textContent = tr("update_include_pre");
    el.updatePreBadge.textContent = tr("update_prerelease_badge");
    el.lblCheckNow.textContent = tr("update_btn_check");
    el.lblInstallNow.textContent = tr("update_btn_install");
    el.btnLang.title = tr("lang_tooltip");
    el.btnTheme.title = tr("theme_tooltip");

    el.chainSectionTitle.textContent = tr("chain_section");
    el.lblSwap.textContent = tr("btn_swap");
    el.lblClear.textContent = tr("btn_clear");
    el.lblHop1.textContent = tr("hop1_label");
    el.hintHop1.textContent = tr("hop1_hint");
    el.btnPasteHop1.title = tr("hop1_paste_tooltip");
    el.lblHop2.textContent = tr("hop2_label");
    el.badgeExit.textContent = tr("hop2_badge");
    el.hintHop2.textContent = tr("hop2_hint");
    el.btnPasteHop2.title = tr("hop2_paste_tooltip");

    const ph = tr("uri_placeholder");
    el.hop1Input.placeholder = ph;
    el.hop2Input.placeholder = ph;

    el.lblSwitchMode.textContent = el.chkMixed.checked
      ? tr("switch_mixed")
      : tr("switch_separate");
    el.lblPort.textContent = tr("port_label");
    el.lblListen.textContent = tr("listen_label");
    el.lblGenerate.textContent = tr("btn_generate");
    el.lblCopy.textContent = tr("btn_copy");
    el.lblSave.textContent = tr("btn_save");
    el.outputSectionTitle.textContent = tr("output_section");

    updateModePillText();
    renderUpdateState();
    renderLanguageMenu();
    updateMetaLabel();
    updateModeBadge();
    updateRoutePills();
  }

  // --- Updater Logic ---
  function renderUpdateState() {
    if (state.isDownloadingUpdate) return;

    if (state.isCheckingUpdate) {
      el.updateStatusText.textContent = tr("update_checking");
      el.updateVersionRow.classList.add("hidden");
      el.btnInstallNow.classList.add("hidden");
      el.btnCheckNow.disabled = true;
      return;
    }

    el.btnCheckNow.disabled = false;

    if (!state.updateData) {
      el.updateStatusText.textContent = tr("update_ready");
      el.updateVersionRow.classList.add("hidden");
      el.btnInstallNow.classList.add("hidden");
      return;
    }

    const u = state.updateData;
    if (u.has_update) {
      el.updateDot.classList.remove("hidden");
      el.updateStatusText.textContent = tr("update_available", { tag: u.tag_name });
      el.updateNewVer.textContent = u.tag_name;
      el.updatePreBadge.classList.toggle("hidden", !u.is_prerelease);
      el.updateVersionRow.classList.remove("hidden");
      el.btnInstallNow.classList.remove("hidden");
      el.btnInstallNow.disabled = false;
    } else {
      el.updateDot.classList.add("hidden");
      el.updateStatusText.textContent = tr("update_up_to_date", { ver: u.current_version });
      el.updateVersionRow.classList.add("hidden");
      el.btnInstallNow.classList.add("hidden");
    }
  }

  async function runUpdateCheck(silent = false) {
    if (!invoke || state.isCheckingUpdate || state.isDownloadingUpdate) return;
    state.isCheckingUpdate = true;
    renderUpdateState();

    try {
      const res = await invoke("check_for_updates", {
        includePrerelease: state.includePrerelease
      });
      state.updateData = res;
      state.isCheckingUpdate = false;
      renderUpdateState();

      if (!silent && res.has_update) {
        showToast(tr("update_title"), tr("update_available", { tag: res.tag_name }), false);
      }
    } catch (err) {
      state.isCheckingUpdate = false;
      el.btnCheckNow.disabled = false;
      if (!silent) {
        el.updateStatusText.textContent = tr("update_failed", { err: String(err) });
      }
    }
  }

  async function installAvailableUpdate() {
    if (!invoke || !state.updateData || state.isDownloadingUpdate) return;
    const u = state.updateData;

    if (!u.download_url || !u.asset_name) {
      el.updateStatusText.textContent = tr("update_no_asset", { tag: u.tag_name });
      invoke("open_external_url", { url: u.release_url }).catch(() => {});
      return;
    }

    state.isDownloadingUpdate = true;
    el.btnCheckNow.disabled = true;
    el.btnInstallNow.disabled = true;
    el.chkPrerelease.disabled = true;
    el.updateStatusText.textContent = tr("update_downloading");
    el.updateProgressWrap.classList.remove("hidden");
    el.updateProgressBar.style.width = "0%";
    el.updateProgressText.textContent = "0%";

    try {
      await invoke("download_and_install_update", {
        downloadUrl: u.download_url,
        assetName: u.asset_name,
        isInstaller: u.is_installer
      });
      el.updateStatusText.textContent = tr("update_applying");
    } catch (err) {
      state.isDownloadingUpdate = false;
      el.btnCheckNow.disabled = false;
      el.btnInstallNow.disabled = false;
      el.chkPrerelease.disabled = false;
      el.updateProgressWrap.classList.add("hidden");
      el.updateStatusText.textContent = tr("update_failed", { err: String(err) });
      showToast(tr("update_title"), String(err), true);
    }
  }

  if (listenEvent) {
    listenEvent("update-progress", (event) => {
      const p = event.payload || {};
      const pct = Math.min(100, Math.max(0, Number(p.percent || 0)));
      el.updateProgressBar.style.width = `${pct}%`;
      if (p.total > 0) {
        el.updateProgressText.textContent = `${pct}% (${formatMB(p.downloaded)} / ${formatMB(p.total)} MB)`;
      } else {
        el.updateProgressText.textContent = `${formatMB(p.downloaded || 0)} MB`;
      }
      if (pct >= 100) {
        el.updateStatusText.textContent = tr("update_applying");
      }
    });
  }

  // --- Chain Logic ---
  function updateMetaLabel() {
    if (!state.lastJson) {
      el.metaLabel.textContent = tr("meta_empty");
      el.jsonOutput.innerHTML = `<span class="code-placeholder">${escapeHtml(
        tr("json_placeholder")
      )}</span>`;
      return;
    }
    el.metaLabel.textContent = tr("meta_stats", {
      lines: state.lastLines,
      size: state.lastBytes,
      exit_ip: state.lastExitIp
    });
  }

  function refreshPortUi() {
    const mixed = el.chkMixed.checked;
    el.mixedGroup.classList.toggle("hidden", !mixed);
    el.splitGroup.classList.toggle("hidden", mixed);
    el.lblSwitchMode.textContent = mixed ? tr("switch_mixed") : tr("switch_separate");
    if (!mixed) {
      el.socksPort.disabled = !el.chkSocks.checked;
      el.httpPort.disabled = !el.chkHttp.checked;
    }
    updateModeBadge();
  }

  function updateModeBadge() {
    if (el.chkMixed.checked) {
      const p = el.mixedPort.value.trim() || "?";
      el.modeBadge.textContent = tr("badge_mixed", { port: p });
      el.modeBadge.className = "pill pill-accent";
      return;
    }

    const sOn = el.chkSocks.checked;
    const hOn = el.chkHttp.checked;
    const sp = el.socksPort.value.trim();
    const hp = el.httpPort.value.trim();

    if (sOn && hOn && sp && sp === hp) {
      el.modeBadge.textContent = tr("badge_auto_mixed", { port: sp });
      el.modeBadge.className = "pill pill-ok";
      return;
    }

    const parts = [];
    if (sOn) parts.push(`socks :${sp || "?"}`);
    if (hOn) parts.push(`http :${hp || "?"}`);

    if (parts.length > 0) {
      el.modeBadge.textContent = parts.join(" + ");
      el.modeBadge.className = "pill pill-muted";
    } else {
      el.modeBadge.textContent = tr("badge_no_inbound");
      el.modeBadge.className = "pill pill-err";
    }
  }

  let hop1Seq = 0;
  let hop2Seq = 0;

  async function updateSinglePill(pillEl, raw, defaultKey, isExit) {
    const value = raw.trim();
    const fallback = tr(defaultKey);
    if (!value) {
      pillEl.textContent = `● ${fallback}`;
      pillEl.className = "pill pill-muted";
      return;
    }
    if (!invoke) return;

    const seq = isExit ? ++hop2Seq : ++hop1Seq;
    try {
      const meta = await invoke("inspect_link", { uri: value });
      if ((isExit && seq !== hop2Seq) || (!isExit && seq !== hop1Seq)) return;
      let host = meta.address || "";
      if (host.length > 18) host = host.slice(0, 16) + "…";
      const suffix = isExit ? tr("pill_exit_suffix") : "";
      pillEl.textContent = `● ${meta.protocol}: ${host}${suffix}`;
      pillEl.className = "pill pill-ok";
    } catch (_) {
      if ((isExit && seq !== hop2Seq) || (!isExit && seq !== hop1Seq)) return;
      pillEl.textContent = `● ${fallback}: ${tr("pill_invalid")}`;
      pillEl.className = "pill pill-err";
    }
  }

  function updateInlineClearButtons() {
    el.btnClearHop1.classList.toggle("hidden", !el.hop1Input.value);
    el.btnClearHop2.classList.toggle("hidden", !el.hop2Input.value);
  }

  function updateRoutePills() {
    updateInlineClearButtons();
    updateSinglePill(el.pillHop1, el.hop1Input.value, "pill_hop1", false);
    updateSinglePill(el.pillHop2, el.hop2Input.value, "pill_hop2", true);
  }

  function parsePortField(raw, errCode) {
    const trimmed = raw.trim();
    if (!/^\d+$/.test(trimmed)) throw { code: errCode };
    const n = Number(trimmed);
    if (!Number.isInteger(n) || n < 1 || n > 65535) throw { code: errCode };
    return n;
  }

  function collectPorts() {
    const listen = el.listenInput.value.trim() || "127.0.0.1";
    if (el.chkMixed.checked) {
      const mixed_port = parsePortField(el.mixedPort.value, "err_invalid_mixed_port");
      return { listen, mixed_port, socks_port: null, http_port: null };
    }

    let socks_port = null;
    let http_port = null;
    if (el.chkSocks.checked) {
      socks_port = parsePortField(el.socksPort.value, "err_invalid_socks_port");
    }
    if (el.chkHttp.checked) {
      http_port = parsePortField(el.httpPort.value, "err_invalid_http_port");
    }
    if (socks_port !== null && http_port !== null && socks_port === http_port) {
      return { listen, mixed_port: socks_port, socks_port: null, http_port: null };
    }
    return { listen, mixed_port: null, socks_port, http_port };
  }

  function formatError(err) {
    if (err && typeof err === "object" && err.code) {
      return tr(err.code, { sample: err.sample || "" });
    }
    if (typeof err === "string") return err;
    return err?.message || "Unknown error";
  }

  async function generateJson(notify = true) {
    if (!invoke) {
      showToast(tr("notify_invalid_title"), "Tauri runtime not detected.", true);
      return false;
    }
    try {
      const ports = collectPorts();
      const resp = await invoke("build_chain", {
        req: {
          hop1: el.hop1Input.value,
          hop2: el.hop2Input.value,
          ...ports
        }
      });
      state.lastJson = resp.json;
      state.lastLines = resp.lines;
      state.lastBytes = resp.bytes;
      state.lastExitIp = resp.exit.address;

      el.jsonOutput.innerHTML = highlightJson(state.lastJson);
      updateMetaLabel();
      saveSettings();

      if (notify) {
        showToast(
          tr("notify_generated_title"),
          tr("notify_generated_body", { exit_ip: state.lastExitIp }),
          false
        );
      }
      return true;
    } catch (err) {
      showToast(tr("notify_invalid_title"), formatError(err), true);
      return false;
    }
  }

  async function pasteInto(inputEl) {
    try {
      let text = "";
      if (invoke) {
        text = await invoke("read_clipboard");
      } else if (navigator.clipboard) {
        text = await navigator.clipboard.readText();
      }
      if (text && text.trim()) {
        inputEl.value = text.trim();
        updateRoutePills();
      }
    } catch (_) {}
  }

  async function copyJson() {
    if (!(await generateJson(false))) return;
    try {
      if (invoke) {
        await invoke("write_clipboard", { text: state.lastJson });
      } else if (navigator.clipboard) {
        await navigator.clipboard.writeText(state.lastJson);
      }
      showToast(tr("notify_copied_title"), tr("notify_copied_body"), false);
    } catch (err) {
      showToast(tr("notify_invalid_title"), String(err), true);
    }
  }

  async function saveJson() {
    if (!(await generateJson(false))) return;
    try {
      const savedName = await invoke("save_json_file", {
        content: state.lastJson,
        title: tr("save_dialog_title"),
        filterJson: tr("filter_json"),
        filterAll: tr("filter_all")
      });
      if (savedName) {
        showToast(tr("notify_saved_title"), savedName, false);
      }
    } catch (err) {
      showToast(tr("notify_save_err_title"), String(err), true);
    }
  }

  // Filter port inputs to digits only
  for (const portEl of [el.mixedPort, el.socksPort, el.httpPort]) {
    portEl.addEventListener("input", () => {
      const digits = portEl.value.replace(/\D/g, "");
      if (portEl.value !== digits) portEl.value = digits;
      updateModeBadge();
    });
  }

  // Header buttons
  el.btnGithub.addEventListener("click", () => {
    if (invoke) {
      invoke("open_external_url", { url: state.appInfo.repo_url }).catch(() => {});
    }
  });

  el.btnUpdate.addEventListener("click", (e) => {
    e.stopPropagation();
    el.langMenu.classList.add("hidden");
    el.updatePanel.classList.toggle("hidden");
  });

  el.chkPrerelease.addEventListener("change", () => {
    state.includePrerelease = el.chkPrerelease.checked;
    state.updateData = null;
    saveSettings();
    renderUpdateState();
  });

  el.btnCheckNow.addEventListener("click", () => runUpdateCheck(false));
  el.btnInstallNow.addEventListener("click", installAvailableUpdate);

  el.btnLang.addEventListener("click", (e) => {
    e.stopPropagation();
    el.updatePanel.classList.add("hidden");
    el.langMenu.classList.toggle("hidden");
  });

  document.addEventListener("click", (e) => {
    if (!el.langMenu.contains(e.target) && !el.btnLang.contains(e.target)) {
      el.langMenu.classList.add("hidden");
    }
    if (!el.updatePanel.contains(e.target) && !el.btnUpdate.contains(e.target)) {
      el.updatePanel.classList.add("hidden");
    }
  });

  el.btnTheme.addEventListener("click", () => {
    state.theme = state.theme === "dark" ? "light" : "dark";
    applyTheme();
    saveSettings();
  });

  el.btnSwap.addEventListener("click", () => {
    const tmp = el.hop1Input.value;
    el.hop1Input.value = el.hop2Input.value;
    el.hop2Input.value = tmp;
    updateRoutePills();
  });

  el.btnClear.addEventListener("click", () => {
    el.hop1Input.value = "";
    el.hop2Input.value = "";
    state.lastJson = "";
    state.lastExitIp = "";
    updateMetaLabel();
    updateRoutePills();
  });

  el.hop1Input.addEventListener("input", updateRoutePills);
  el.hop2Input.addEventListener("input", updateRoutePills);
  el.btnClearHop1.addEventListener("click", () => {
    el.hop1Input.value = "";
    updateRoutePills();
  });
  el.btnClearHop2.addEventListener("click", () => {
    el.hop2Input.value = "";
    updateRoutePills();
  });

  el.btnPasteHop1.addEventListener("click", () => pasteInto(el.hop1Input));
  el.btnPasteHop2.addEventListener("click", () => pasteInto(el.hop2Input));

  el.chkMixed.addEventListener("change", () => {
    refreshPortUi();
    saveSettings();
  });
  el.chkSocks.addEventListener("change", () => {
    refreshPortUi();
    saveSettings();
  });
  el.chkHttp.addEventListener("change", () => {
    refreshPortUi();
    saveSettings();
  });

  el.btnGenerate.addEventListener("click", () => generateJson(true));
  el.btnCopy.addEventListener("click", copyJson);
  el.btnSave.addEventListener("click", saveJson);

  // Initialize
  loadSettings();
  applyTheme();
  refreshPortUi();
  applyLanguage();

  if (invoke) {
    invoke("get_app_info")
      .then((info) => {
        state.appInfo = info;
        updateModePillText();
      })
      .catch(() => {});

    requestAnimationFrame(() => {
      invoke("show_main_window").catch(() => {});
    });

    // Silent background update check on startup
    setTimeout(() => runUpdateCheck(true), 2000);
  }
})();
