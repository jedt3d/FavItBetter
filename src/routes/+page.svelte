<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Database, ExternalLink, RefreshCw, Search, Upload, WandSparkles } from '@lucide/svelte';

  type BookmarkRow = {
    id: number;
    sourceBrowser: string;
    sourceProfile: string;
    sourcePath: string;
    folderPath: string;
    title: string;
    originalUrl: string;
    canonicalUrl: string;
    cleanedUrl: string;
    dateAdded: string | null;
    status: string;
    archiveReason: string | null;
    linkCheckState: string;
    lastCheckedAt: string | null;
    httpStatus: number | null;
    checkAttempts: number;
    checkError: string | null;
    importedAt: string;
  };

  type ImportResult = {
    importedCount: number;
    activeCount: number;
    sourceBrowser: string;
    sourcePath: string;
    report: string;
  };

  type CleanResult = {
    activeCountBefore: number;
    activeCountAfter: number;
    duplicateCount: number;
    queryCleanedCount: number;
    removedTrackingParamCount: number;
    archivedCount: number;
    deadLinkCount: number;
    errors: string[];
    report: string;
  };

  type BookmarkListResult = {
    rows: BookmarkRow[];
    totalCount: number;
    activeCount: number;
    archivedCount: number;
    filteredCount: number;
    limit: number;
    offset: number;
  };

  type SortKey = 'title' | 'sourceBrowser' | 'folderPath' | 'originalUrl' | 'status';
  type StatusFilter = 'active' | 'archived' | 'all';

  let bookmarks: BookmarkRow[] = [];
  let selectedId: number | null = null;
  let query = '';
  let statusFilter: StatusFilter = 'active';
  let sortKey: SortKey = 'title';
  let sortDirection: 1 | -1 = 1;
  const tauriRuntimeHelp =
    'This screen is running in a normal browser, so Tauri commands are unavailable.\nRun `npm run tauri -- dev` and use the launched FavItBetter desktop window instead of http://127.0.0.1:5173/.';
  let report = 'Ready. Import a Chromium Bookmarks JSON file from Google Chrome or Microsoft Edge.';
  let isTauriRuntime = false;
  let isImporting = false;
  let isCleaning = false;
  let isLoadingList = false;
  let sourceBrowser = 'chrome';
  let fileInput: HTMLInputElement;
  let splitHost: HTMLElement;
  let leftWidth = 520;
  let pageSize = 250;
  let pageIndex = 0;
  let totalCount = 0;
  let activeCount = 0;
  let archivedCount = 0;
  let filteredCount = 0;
  let searchDebounce: ReturnType<typeof setTimeout> | null = null;

  $: selectedBookmark =
    bookmarks.find((bookmark) => bookmark.id === selectedId) ?? bookmarks[0] ?? null;
  $: previewUrl = selectedBookmark?.cleanedUrl ?? selectedBookmark?.originalUrl ?? '';
  $: canClean = isTauriRuntime && activeCount > 0 && !isImporting && !isCleaning;
  $: pageStart = filteredCount === 0 ? 0 : pageIndex * pageSize + 1;
  $: pageEnd = Math.min((pageIndex + 1) * pageSize, filteredCount);
  $: pageCount = Math.max(Math.ceil(filteredCount / pageSize), 1);
  $: canPageBack = pageIndex > 0 && !isLoadingList;
  $: canPageForward = pageEnd < filteredCount && !isLoadingList;

  onMount(() => {
    isTauriRuntime = hasTauriRuntime();
    if (!isTauriRuntime) {
      report = tauriRuntimeHelp;
      return;
    }

    void refreshBookmarks();
  });

  function hasTauriRuntime(): boolean {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  }

  function setSort(nextKey: SortKey) {
    if (sortKey === nextKey) {
      sortDirection = sortDirection === 1 ? -1 : 1;
    } else {
      sortKey = nextKey;
      sortDirection = 1;
    }

    pageIndex = 0;
    void refreshBookmarks();
  }

  function setStatusFilter(nextFilter: StatusFilter) {
    if (statusFilter === nextFilter) return;
    statusFilter = nextFilter;
    pageIndex = 0;
    void refreshBookmarks();
  }

  function handleSearchInput(event: Event) {
    query = (event.currentTarget as HTMLInputElement).value;
    scheduleSearchRefresh();
  }

  function scheduleSearchRefresh() {
    pageIndex = 0;
    if (searchDebounce) {
      clearTimeout(searchDebounce);
    }

    searchDebounce = setTimeout(() => {
      void refreshBookmarks();
    }, 180);
  }

  function previousPage() {
    if (!canPageBack) return;
    pageIndex -= 1;
    void refreshBookmarks();
  }

  function nextPage() {
    if (!canPageForward) return;
    pageIndex += 1;
    void refreshBookmarks();
  }

  async function refreshBookmarks() {
    if (!isTauriRuntime) return;

    isLoadingList = true;
    try {
      const result = await invoke<BookmarkListResult>('list_bookmarks', {
        request: {
          query,
          statusFilter,
          sortKey,
          sortDirection: sortDirection === 1 ? 'asc' : 'desc',
          limit: pageSize,
          offset: pageIndex * pageSize
        }
      });

      if (result.rows.length === 0 && result.filteredCount > 0 && pageIndex > 0) {
        pageIndex = Math.max(Math.ceil(result.filteredCount / pageSize) - 1, 0);
        isLoadingList = false;
        await refreshBookmarks();
        return;
      }

      bookmarks = result.rows;
      totalCount = result.totalCount;
      activeCount = result.activeCount;
      archivedCount = result.archivedCount;
      filteredCount = result.filteredCount;

      if (bookmarks.length > 0 && !bookmarks.some((bookmark) => bookmark.id === selectedId)) {
        selectedId = bookmarks[0].id;
      } else if (bookmarks.length === 0) {
        selectedId = null;
      }
    } catch (error) {
      report = `Could not load local project database.\n${formatError(error)}`;
    } finally {
      isLoadingList = false;
    }
  }

  function openImportPicker() {
    if (!isTauriRuntime) {
      report = tauriRuntimeHelp;
      return;
    }

    fileInput.click();
  }

  async function handleFileSelected(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    if (!isTauriRuntime) {
      report = tauriRuntimeHelp;
      target.value = '';
      return;
    }

    const file = target.files?.[0];
    if (!file) return;

    isImporting = true;
    report = `Importing ${file.name}...`;

    try {
      const content = await file.text();
      const result = await invoke<ImportResult>('import_bookmarks_json', {
        request: {
          sourceBrowser,
          sourcePath: file.name,
          content
        }
      });

      pageIndex = 0;
      await refreshBookmarks();
      report = result.report;
    } catch (error) {
      report = `Import failed for ${file.name}.\n${formatError(error)}`;
    } finally {
      isImporting = false;
      target.value = '';
    }
  }

  async function cleanBookmarks() {
    if (!canClean) return;

    isCleaning = true;
    report = 'Cleaning local bookmark pool...';

    try {
      const result = await invoke<CleanResult>('clean_bookmarks');
      pageIndex = 0;
      await refreshBookmarks();
      report = result.report;
    } catch (error) {
      report = `Clean failed.\n${formatError(error)}`;
    } finally {
      isCleaning = false;
    }
  }

  function startResize(event: PointerEvent) {
    event.preventDefault();
    window.addEventListener('pointermove', resizePanels);
    window.addEventListener('pointerup', stopResize, { once: true });
  }

  function resizePanels(event: PointerEvent) {
    const bounds = splitHost.getBoundingClientRect();
    const minLeft = 360;
    const minRight = 420;
    const requested = event.clientX - bounds.left;
    leftWidth = Math.min(Math.max(requested, minLeft), bounds.width - minRight);
  }

  function stopResize() {
    window.removeEventListener('pointermove', resizePanels);
  }

  function formatError(error: unknown): string {
    if (error instanceof Error) return error.message;
    if (typeof error === 'string') return error;
    return JSON.stringify(error);
  }

  function formatCount(value: number): string {
    return new Intl.NumberFormat().format(value);
  }
</script>

<svelte:head>
  <title>FavItBetter</title>
</svelte:head>

<main class="app-shell">
  <header class="toolbar">
    <div class="brand">
      <Database size={20} aria-hidden="true" />
      <div>
        <h1>FavItBetter</h1>
        <p>{formatCount(activeCount)} active / {formatCount(archivedCount)} archived</p>
      </div>
    </div>

    <div class="toolbar-actions">
      <label class="browser-select">
        <span>Source</span>
        <select bind:value={sourceBrowser} aria-label="Source browser">
          <option value="chrome">Google Chrome</option>
          <option value="edge">Microsoft Edge</option>
        </select>
      </label>

      <button
        type="button"
        class="primary-button"
        onclick={openImportPicker}
        disabled={isImporting || !isTauriRuntime}
        title={isTauriRuntime ? 'Import Chromium Bookmarks JSON' : 'Run with npm run tauri -- dev'}
      >
        {#if isImporting}
          <RefreshCw size={18} aria-hidden="true" class="spin" />
          Importing
        {:else}
          <Upload size={18} aria-hidden="true" />
          Import
        {/if}
      </button>

      <button
        type="button"
        class="secondary-button"
        onclick={cleanBookmarks}
        disabled={!canClean}
        title={isTauriRuntime
          ? 'Clean tracking parameters and archive duplicates'
          : 'Run with npm run tauri -- dev'}
      >
        {#if isCleaning}
          <RefreshCw size={18} aria-hidden="true" class="spin" />
          Cleaning
        {:else}
          <WandSparkles size={18} aria-hidden="true" />
          Clean
        {/if}
      </button>

      <input
        bind:this={fileInput}
        class="hidden-input"
        type="file"
        accept="application/json,.json"
        onchange={handleFileSelected}
      />
    </div>
  </header>

  <section class="workspace" bind:this={splitHost} style={`--left-width: ${leftWidth}px`}>
    <section class="table-pane" aria-label="Bookmark list">
      <div class="table-tools">
        <label class="search-box">
          <Search size={17} aria-hidden="true" />
          <input
            value={query}
            type="search"
            placeholder="Search bookmarks"
            oninput={handleSearchInput}
          />
        </label>
        <div class="status-tabs" aria-label="Bookmark status filter">
          <button
            type="button"
            class:active={statusFilter === 'active'}
            onclick={() => setStatusFilter('active')}
          >
            Active
          </button>
          <button
            type="button"
            class:active={statusFilter === 'archived'}
            onclick={() => setStatusFilter('archived')}
          >
            Archived
          </button>
          <button
            type="button"
            class:active={statusFilter === 'all'}
            onclick={() => setStatusFilter('all')}
          >
            All
          </button>
        </div>
        <span class="row-count">
          {#if isLoadingList}
            Loading
          {:else}
            {formatCount(pageStart)}-{formatCount(pageEnd)} of {formatCount(filteredCount)}
          {/if}
        </span>
      </div>

      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>
                <button type="button" onclick={() => setSort('title')}>Title</button>
              </th>
              <th>
                <button type="button" onclick={() => setSort('sourceBrowser')}>Source</button>
              </th>
              <th>
                <button type="button" onclick={() => setSort('folderPath')}>Folder</button>
              </th>
              <th>
                <button type="button" onclick={() => setSort('status')}>Status</button>
              </th>
            </tr>
          </thead>
          <tbody>
            {#each bookmarks as bookmark (bookmark.id)}
              <tr
                class:active={selectedBookmark?.id === bookmark.id}
                onclick={() => (selectedId = bookmark.id)}
              >
                <td>
                  <strong>{bookmark.title || 'Untitled'}</strong>
                  <small>{bookmark.cleanedUrl || bookmark.originalUrl}</small>
                </td>
                <td>{bookmark.sourceBrowser}</td>
                <td>{bookmark.folderPath}</td>
                <td>
                  <span class="status" class:archived-status={bookmark.status !== 'active'}>
                    {bookmark.status}
                  </span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>

        {#if bookmarks.length === 0}
          <div class="empty-state">No bookmarks match the current search.</div>
        {/if}
      </div>

      <div class="table-pager">
        <span>
          Page {formatCount(pageIndex + 1)} of {formatCount(pageCount)} · {formatCount(totalCount)} total
        </span>
        <div>
          <button type="button" onclick={previousPage} disabled={!canPageBack}>Previous</button>
          <button type="button" onclick={nextPage} disabled={!canPageForward}>Next</button>
        </div>
      </div>
    </section>

    <button
      type="button"
      class="splitter"
      aria-label="Resize bookmark table and preview"
      onpointerdown={startResize}
    ></button>

    <section class="preview-pane" aria-label="Bookmark preview">
      {#if selectedBookmark}
        <div class="preview-header">
          <div>
            <h2>{selectedBookmark.title || 'Untitled'}</h2>
            <p>{selectedBookmark.cleanedUrl || selectedBookmark.originalUrl}</p>
          </div>
          <a href={selectedBookmark.cleanedUrl || selectedBookmark.originalUrl} target="_blank" rel="noreferrer">
            <ExternalLink size={17} aria-hidden="true" />
            Open
          </a>
        </div>

        {#if previewUrl.startsWith('http://') || previewUrl.startsWith('https://')}
          <iframe title="Bookmark preview" src={previewUrl}></iframe>
        {:else}
          <div class="preview-empty">Preview supports HTTP and HTTPS URLs.</div>
        {/if}
      {:else}
        <div class="preview-empty">Import bookmarks and select a row to preview it.</div>
      {/if}
    </section>
  </section>

  <section class="report-pane" aria-label="Import report">
    <textarea readonly bind:value={report}></textarea>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    min-width: 960px;
    min-height: 640px;
    color: #1e293b;
    background: #eef2f4;
    font-family:
      Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  :global(button),
  :global(input),
  :global(select),
  :global(textarea) {
    font: inherit;
  }

  .app-shell {
    display: grid;
    grid-template-rows: 64px minmax(0, 1fr) 132px;
    height: 100vh;
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    padding: 0 18px;
    border-bottom: 1px solid #cfd8dc;
    background: #f9fbfb;
  }

  .brand,
  .toolbar-actions,
  .browser-select,
  .search-box,
  .status-tabs,
  .table-pager,
  .table-pager > div,
  .preview-header,
  .preview-header a {
    display: flex;
    align-items: center;
  }

  .brand {
    gap: 10px;
    min-width: 240px;
  }

  .brand :global(svg) {
    color: #0f766e;
  }

  h1,
  h2,
  p {
    margin: 0;
  }

  h1 {
    font-size: 18px;
    font-weight: 720;
  }

  .brand p,
  .preview-header p {
    color: #64748b;
    font-size: 12px;
  }

  .toolbar-actions {
    gap: 12px;
  }

  .browser-select {
    gap: 8px;
    color: #475569;
    font-size: 13px;
  }

  select,
  .search-box input {
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    background: #ffffff;
    color: #1e293b;
    outline: none;
  }

  select {
    height: 36px;
    padding: 0 32px 0 10px;
  }

  .primary-button {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 14px;
    border: 1px solid #0f766e;
    border-radius: 6px;
    background: #0f766e;
    color: #ffffff;
    cursor: pointer;
  }

  .primary-button:disabled {
    cursor: wait;
    opacity: 0.75;
  }

  .secondary-button {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 14px;
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    background: #ffffff;
    color: #64748b;
  }

  .secondary-button:disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }

  .hidden-input {
    display: none;
  }

  .workspace {
    display: grid;
    grid-template-columns: minmax(360px, var(--left-width)) 6px minmax(420px, 1fr);
    min-height: 0;
  }

  .table-pane,
  .preview-pane {
    min-width: 0;
    background: #ffffff;
  }

  .table-pane {
    display: grid;
    grid-template-rows: 52px minmax(0, 1fr) 40px;
    border-right: 1px solid #d6dee3;
  }

  .table-tools {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    border-bottom: 1px solid #e0e7eb;
    color: #64748b;
    font-size: 12px;
  }

  .search-box {
    flex: 1;
    gap: 8px;
    height: 32px;
    padding: 0 10px;
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    background: #ffffff;
  }

  .search-box input {
    width: 100%;
    height: 100%;
    border: 0;
  }

  .status-tabs {
    flex: 0 0 auto;
    overflow: hidden;
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    background: #ffffff;
  }

  .status-tabs button {
    height: 30px;
    min-width: 64px;
    border: 0;
    border-right: 1px solid #d6dee3;
    background: transparent;
    color: #64748b;
    cursor: pointer;
    font-size: 12px;
  }

  .status-tabs button:last-child {
    border-right: 0;
  }

  .status-tabs button.active {
    background: #e2f4f1;
    color: #0f766e;
    font-weight: 700;
  }

  .row-count {
    flex: 0 0 auto;
    min-width: 116px;
    text-align: right;
    white-space: nowrap;
  }

  .table-wrap {
    position: relative;
    min-height: 0;
    overflow-x: hidden;
    overflow-y: scroll;
    scrollbar-gutter: stable;
  }

  table {
    width: 100%;
    border-spacing: 0;
    table-layout: fixed;
    font-size: 13px;
  }

  th {
    position: sticky;
    top: 0;
    z-index: 1;
    border-bottom: 1px solid #d9e1e6;
    background: #f6f8f9;
    text-align: left;
  }

  th button {
    width: 100%;
    padding: 9px 10px;
    border: 0;
    background: transparent;
    color: #334155;
    cursor: pointer;
    text-align: left;
    font-weight: 680;
  }

  th:nth-child(1) {
    width: 42%;
  }

  th:nth-child(2) {
    width: 15%;
  }

  th:nth-child(3) {
    width: 31%;
  }

  th:nth-child(4) {
    width: 12%;
  }

  td {
    padding: 10px;
    border-bottom: 1px solid #edf1f3;
    vertical-align: top;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  tr {
    cursor: pointer;
  }

  tbody tr:hover {
    background: #f3f8f7;
  }

  tbody tr.active {
    background: #e2f4f1;
  }

  td strong,
  td small {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  td small {
    margin-top: 3px;
    color: #64748b;
  }

  .status {
    color: #0f766e;
    font-weight: 650;
  }

  .archived-status {
    color: #64748b;
  }

  .table-pager {
    justify-content: space-between;
    gap: 12px;
    padding: 0 12px;
    border-top: 1px solid #e0e7eb;
    background: #f6f8f9;
    color: #64748b;
    font-size: 12px;
  }

  .table-pager > div {
    gap: 8px;
  }

  .table-pager button {
    height: 28px;
    padding: 0 10px;
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    background: #ffffff;
    color: #334155;
    cursor: pointer;
  }

  .table-pager button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .empty-state,
  .preview-empty {
    display: grid;
    place-items: center;
    min-height: 180px;
    color: #64748b;
    font-size: 13px;
  }

  .splitter {
    width: 6px;
    border: 0;
    border-inline: 1px solid #cbd5dc;
    background: #dde5e8;
    cursor: col-resize;
  }

  .splitter:hover {
    background: #9fb5bd;
  }

  .preview-pane {
    display: grid;
    grid-template-rows: 72px minmax(0, 1fr);
    min-height: 0;
  }

  .preview-header {
    justify-content: space-between;
    gap: 12px;
    min-width: 0;
    padding: 12px 14px;
    border-bottom: 1px solid #d9e1e6;
    background: #fbfcfc;
  }

  .preview-header > div {
    min-width: 0;
  }

  .preview-header h2,
  .preview-header p {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .preview-header h2 {
    font-size: 15px;
    line-height: 1.35;
  }

  .preview-header a {
    flex: 0 0 auto;
    gap: 6px;
    height: 34px;
    padding: 0 10px;
    border: 1px solid #b8c4cc;
    border-radius: 6px;
    color: #0f766e;
    text-decoration: none;
  }

  iframe {
    width: 100%;
    height: 100%;
    border: 0;
    background: #ffffff;
  }

  .report-pane {
    border-top: 1px solid #cfd8dc;
    background: #f8fafb;
    padding: 10px 12px;
  }

  textarea {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    resize: none;
    border: 1px solid #c6d0d6;
    border-radius: 6px;
    padding: 10px;
    background: #ffffff;
    color: #334155;
    line-height: 1.45;
  }

  :global(.spin) {
    animation: spin 900ms linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
