PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS wow_instances (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL UNIQUE,
    flavor_slug     TEXT NOT NULL,
    root_path       TEXT NOT NULL,
    addons_path     TEXT NOT NULL,
    wtf_path        TEXT,
    toc_suffixes    TEXT NOT NULL DEFAULT '[]',
    is_default      INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS addons (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id         INTEGER NOT NULL REFERENCES wow_instances(id) ON DELETE CASCADE,
    slug                TEXT NOT NULL,
    display_name        TEXT NOT NULL,
    installed_version   TEXT,
    latest_version      TEXT,
    interface_version   TEXT,
    author              TEXT,
    notes               TEXT,
    provider_type       TEXT NOT NULL DEFAULT 'local',
    provider_ref        TEXT,
    auto_update         INTEGER NOT NULL DEFAULT 1,
    is_enabled          INTEGER NOT NULL DEFAULT 1,
    last_checked_at     TEXT,
    last_updated_at     TEXT,
    created_at          TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(instance_id, slug)
);

CREATE TABLE IF NOT EXISTS addon_directories (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    addon_id        INTEGER NOT NULL REFERENCES addons(id) ON DELETE CASCADE,
    folder_name     TEXT NOT NULL,
    toc_file        TEXT,
    sha256_summary  TEXT,
    is_present      INTEGER NOT NULL DEFAULT 1,
    UNIQUE(addon_id, folder_name)
);

CREATE TABLE IF NOT EXISTS addon_dependencies (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    addon_id        INTEGER NOT NULL REFERENCES addons(id) ON DELETE CASCADE,
    target_name     TEXT NOT NULL,
    dep_kind        TEXT NOT NULL CHECK (dep_kind IN ('required', 'optional')),
    is_satisfied    INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS profiles (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id     INTEGER NOT NULL REFERENCES wow_instances(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(instance_id, name)
);

CREATE TABLE IF NOT EXISTS profile_addons (
    profile_id      INTEGER NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    addon_id        INTEGER NOT NULL REFERENCES addons(id) ON DELETE CASCADE,
    enabled         INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (profile_id, addon_id)
);

CREATE TABLE IF NOT EXISTS snapshots (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id     INTEGER NOT NULL REFERENCES wow_instances(id) ON DELETE CASCADE,
    label           TEXT NOT NULL,
    scope           TEXT NOT NULL CHECK (scope IN ('addons', 'wtf', 'full')),
    archive_path    TEXT NOT NULL,
    size_bytes      INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS activity_logs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id     INTEGER REFERENCES wow_instances(id) ON DELETE SET NULL,
    action          TEXT NOT NULL,
    status          TEXT NOT NULL CHECK (status IN ('ok', 'warn', 'error')),
    message         TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_addons_instance ON addons(instance_id);
CREATE INDEX IF NOT EXISTS idx_addon_dirs_addon ON addon_directories(addon_id);
CREATE INDEX IF NOT EXISTS idx_addon_deps_addon ON addon_dependencies(addon_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_instance_date ON snapshots(instance_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_logs_instance_date ON activity_logs(instance_id, created_at DESC);