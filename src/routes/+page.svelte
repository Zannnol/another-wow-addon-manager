<script lang="ts">
  import { onMount } from "svelte";
  import "../styles/app.css";
  import { createInstance, fetchInstances, formatError } from "$lib/tauriClient";
  import type { NewWowInstance, WowInstance } from "$lib/types/instance";

  type NavSection = "instances" | "addons" | "profiles" | "snapshots" | "logs";

  const NAV_ITEMS: { id: NavSection; label: string }[] = [
    { id: "instances", label: "WoW Instances" },
    { id: "addons", label: "Addons" },
    { id: "profiles", label: "Profiles" },
    { id: "snapshots", label: "Snapshots" },
    { id: "logs", label: "Activity Log" },
  ];

  let section: NavSection = "instances";
  let instances: WowInstance[] = [];
  let selectedInstanceId: number | null = null;

  let name = "";
  let flavorSlug = "retail";
  let rootPath = "";
  let addonsPath = "";
  let wtfPath = "";
  let suffixesRaw = "_Mainline.toc, .toc";
  let isDefault = true;

  let addonsTouched = false;
  let wtfTouched = false;
  let submitting = false;
  let formError = "";
  let statusText = "Ready · 100% local storage (SQLite)";
  let statusError = false;

  $: activeInstance = instances.find((i) => i.id === selectedInstanceId) ?? null;

  function setStatus(msg: string, err = false) {
    statusText = msg;
    statusError = err;
  }

  function handleRootInput() {
    const cleaned = rootPath.trim().replace(/[\\/]+$/, "");
    if (!cleaned) return;
    const sep = cleaned.includes("\\") ? "\\" : "/";
    if (!addonsTouched) {
      addonsPath = `${cleaned}${sep}Interface${sep}AddOns`;
    }
    if (!wtfTouched) {
      wtfPath = `${cleaned}${sep}WTF`;
    }
  }

  function parseSuffixes(raw: string): string[] {
    const parts = raw
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s.length > 0);
    return parts.length > 0 ? parts : [".toc"];
  }

  async function reloadInstances(preferredId?: number) {
    try {
      const list = await fetchInstances();
      instances = list;

      if (preferredId && list.some((i) => i.id === preferredId)) {
        selectedInstanceId = preferredId;
      } else if (
        selectedInstanceId === null ||
        !list.some((i) => i.id === selectedInstanceId)
      ) {
        const def = list.find((i) => i.is_default) ?? list[0] ?? null;
        selectedInstanceId = def ? def.id : null;
      }

      if (list.length > 0 && !addonsTouched) {
        isDefault = false;
      }
    } catch (err) {
      setStatus(formatError(err), true);
    }
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    formError = "";
    submitting = true;

    const payload: NewWowInstance = {
      name: name.trim(),
      flavor_slug: flavorSlug.trim(),
      root_path: rootPath.trim(),
      addons_path: addonsPath.trim(),
      wtf_path: wtfPath.trim().length > 0 ? wtfPath.trim() : null,
      toc_suffixes: parseSuffixes(suffixesRaw),
      is_default: isDefault,
    };

    try {
      const created = await createInstance(payload);
      name = "";
      rootPath = "";
      addonsPath = "";
      wtfPath = "";
      addonsTouched = false;
      wtfTouched = false;
      setStatus(`Instance "${created.name}" saved to SQLite.`);
      await reloadInstances(created.id);
    } catch (err) {
      const msg = formatError(err);
      formError = msg;
      setStatus(msg, true);
    } finally {
      submitting = false;
    }
  }

  onMount(() => {
    void reloadInstances();
  });
</script>

<div class="app-shell">
  <aside class="sidebar">
    <div class="brand">Another WoW Addon Manager</div>
    <nav class="nav-list" aria-label="Primary navigation">
      {#each NAV_ITEMS as item}
        <button
          type="button"
          class="nav-btn {item.id === section ? 'is-active' : ''}"
          on:click={() => (section = item.id)}
        >
          {item.label}
        </button>
      {/each}
    </nav>
  </aside>

  <header class="topbar">
    <div class="topbar-left">
      <span class="topbar-label">Active instance:</span>
      <select class="select" style="width: 260px;" bind:value={selectedInstanceId}>
        {#if instances.length === 0}
          <option value={null}>No instance configured</option>
        {:else}
          {#each instances as inst}
            <option value={inst.id}>{inst.name} ({inst.flavor_slug})</option>
          {/each}
        {/if}
      </select>
    </div>
    <div class="topbar-right">
      <button type="button" class="btn" on:click={() => (section = "instances")}>
        + Manage instances
      </button>
    </div>
  </header>

  <main class="main-content">
    {#if section === "instances"}
      <div class="view-header">
        <h1>World of Warcraft Instances</h1>
        <p>Configure your local installations (Retail, Classic, Forever, etc.) with zero external tracking.</p>
      </div>

      <div class="grid-2">
        <section class="card">
          <h2>Add an instance</h2>
          <form on:submit={handleSubmit}>
            <div class="form-group">
              <label for="inst-name">Instance name *</label>
              <input
                id="inst-name"
                class="input"
                type="text"
                placeholder="e.g. Retail or Classic Era"
                bind:value={name}
                required
              />
            </div>

            <div class="form-group">
              <label for="inst-flavor">Flavor / client version *</label>
              <select id="inst-flavor" class="select" bind:value={flavorSlug}>
                <option value="retail">retail</option>
                <option value="classic_era">classic_era</option>
                <option value="classic_progression">classic_progression</option>
                <option value="forever">forever</option>
                <option value="custom">custom</option>
              </select>
            </div>

            <div class="form-group">
              <label for="inst-root">Client root directory *</label>
              <input
                id="inst-root"
                class="input mono"
                type="text"
                placeholder="e.g. C:\Program Files (x86)\World of Warcraft\_retail_"
                bind:value={rootPath}
                on:input={handleRootInput}
                required
              />
            </div>

            <div class="form-group">
              <label for="inst-addons">Interface/AddOns directory *</label>
              <input
                id="inst-addons"
                class="input mono"
                type="text"
                placeholder="Auto-filled or enter manually"
                bind:value={addonsPath}
                on:input={() => (addonsTouched = addonsPath.trim().length > 0)}
                required
              />
            </div>

            <div class="form-group">
              <label for="inst-wtf">WTF directory (optional)</label>
              <input
                id="inst-wtf"
                class="input mono"
                type="text"
                placeholder="Optional"
                bind:value={wtfPath}
                on:input={() => (wtfTouched = wtfPath.trim().length > 0)}
              />
            </div>

            <div class="form-group">
              <label for="inst-suffixes">.toc suffix priority (comma-separated)</label>
              <input
                id="inst-suffixes"
                class="input mono"
                type="text"
                bind:value={suffixesRaw}
              />
            </div>

            <label class="checkbox-row">
              <input type="checkbox" bind:checked={isDefault} />
              <span>Set as default instance</span>
            </label>

            <button type="submit" class="btn btn-primary" disabled={submitting}>
              {submitting ? "Saving..." : "Save instance"}
            </button>

            {#if formError}
              <div class="error-box">{formError}</div>
            {/if}
          </form>
        </section>

        <section class="card">
          <h2>Configured instances ({instances.length})</h2>
          {#if instances.length === 0}
            <p class="empty-state">
              No instances stored in SQLite yet. Use the form on the left to add one.
            </p>
          {:else}
            <div class="instance-list">
              {#each instances as inst}
                <article class="instance-item">
                  <div class="instance-item-top">
                    <span class="instance-title">{inst.name}</span>
                    <div class="badges">
                      <span class="badge">{inst.flavor_slug}</span>
                      {#if inst.is_default}
                        <span class="badge badge-default">Default</span>
                      {/if}
                    </div>
                  </div>
                  <div class="path-line">AddOns: {inst.addons_path}</div>
                  {#if inst.wtf_path}
                    <div class="path-line">WTF: {inst.wtf_path}</div>
                  {/if}
                  <div class="path-line">.toc: {inst.toc_suffixes.join(" > ")}</div>
                </article>
              {/each}
            </div>
          {/if}
        </section>
      </div>
    {:else}
      <div class="view-header">
        <h1>Coming soon</h1>
        <p>This view will be wired up once the corresponding Rust module is implemented.</p>
      </div>
      <div class="card">
        <p class="empty-state">
          Selected instance: <strong>{activeInstance ? activeInstance.name : "None"}</strong>.
        </p>
      </div>
    {/if}
  </main>

  <footer class="statusbar">
    <span class={statusError ? "status-err" : "status-ok"}>{statusText}</span>
    <span>Privacy-first · Zero telemetry</span>
  </footer>
</div>