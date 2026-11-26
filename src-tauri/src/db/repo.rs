use crate::db::Database;
use crate::error::{AppError, AppResult};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WowInstance {
    pub id: i64,
    pub name: String,
    pub flavor_slug: String,
    pub root_path: String,
    pub addons_path: String,
    pub wtf_path: Option<String>,
    pub toc_suffixes: Vec<String>,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewWowInstance {
    pub name: String,
    pub flavor_slug: String,
    pub root_path: String,
    pub addons_path: String,
    pub wtf_path: Option<String>,
    pub toc_suffixes: Vec<String>,
    pub is_default: bool,
}

impl Database {
    pub fn list_instances(&self) -> AppResult<Vec<WowInstance>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, flavor_slug, root_path, addons_path, wtf_path,
                        toc_suffixes, is_default, created_at, updated_at
                 FROM wow_instances
                 ORDER BY is_default DESC, name ASC",
            )?;

            let rows = stmt.query_map([], |row| {
                let suffixes_raw: String = row.get(6)?;
                let toc_suffixes: Vec<String> =
                    serde_json::from_str(&suffixes_raw).unwrap_or_default();
                let is_default_int: i64 = row.get(7)?;

                Ok(WowInstance {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    flavor_slug: row.get(2)?,
                    root_path: row.get(3)?,
                    addons_path: row.get(4)?,
                    wtf_path: row.get(5)?,
                    toc_suffixes,
                    is_default: is_default_int != 0,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            })?;

            let mut result = Vec::new();
            for item in rows {
                result.push(item?);
            }
            Ok(result)
        })
    }

    pub fn insert_instance(&self, input: &NewWowInstance) -> AppResult<WowInstance> {
        if input.name.trim().is_empty() {
            return Err(AppError::Validation(
                "The instance name cannot be empty.".into(),
            ));
        }

        let suffixes_json = serde_json::to_string(&input.toc_suffixes)?;

        self.with_conn(|conn| {
            let tx = conn.transaction()?;

            if input.is_default {
                tx.execute("UPDATE wow_instances SET is_default = 0", [])?;
            }

            tx.execute(
                "INSERT INTO wow_instances
                 (name, flavor_slug, root_path, addons_path, wtf_path, toc_suffixes, is_default)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    input.name.trim(),
                    input.flavor_slug.trim(),
                    input.root_path,
                    input.addons_path,
                    input.wtf_path,
                    suffixes_json,
                    if input.is_default { 1 } else { 0 }
                ],
            )?;

            let id = tx.last_insert_rowid();
            tx.commit()?;

            let instance = conn
                .query_row(
                    "SELECT id, name, flavor_slug, root_path, addons_path, wtf_path,
                            toc_suffixes, is_default, created_at, updated_at
                     FROM wow_instances WHERE id = ?1",
                    [id],
                    |row| {
                        let suffixes_raw: String = row.get(6)?;
                        let toc_suffixes: Vec<String> =
                            serde_json::from_str(&suffixes_raw).unwrap_or_default();
                        let is_default_int: i64 = row.get(7)?;

                        Ok(WowInstance {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            flavor_slug: row.get(2)?,
                            root_path: row.get(3)?,
                            addons_path: row.get(4)?,
                            wtf_path: row.get(5)?,
                            toc_suffixes,
                            is_default: is_default_int != 0,
                            created_at: row.get(8)?,
                            updated_at: row.get(9)?,
                        })
                    },
                )
                .optional()?
                .ok_or_else(|| AppError::NotFound("Instance not found after insertion".into()))?;

            Ok(instance)
        })
    }

    pub fn delete_instance(&self, id: i64) -> AppResult<()> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM wow_instances WHERE id = ?1", [id])?;
            if affected == 0 {
                return Err(AppError::NotFound(format!("Instance #{id} not found")));
            }
            Ok(())
        })
    }

    pub fn log_activity(
        &self,
        instance_id: Option<i64>,
        action: &str,
        status: &str,
        message: &str,
    ) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO activity_logs (instance_id, action, status, message)
                 VALUES (?1, ?2, ?3, ?4)",
                params![instance_id, action, status, message],
            )?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_and_instance_crud() {
        let db = Database::open_in_memory().expect("Opening the DB memory");
        let temp_dir = tempfile::tempdir().expect("Temporary File");
        let fake_root = temp_dir.path().join("_retail_");
        let fake_addons = fake_root.join("Interface/AddOns");
        let fake_wtf = fake_root.join("WTF");

        let created = db
            .insert_instance(&NewWowInstance {
                name: "Retail".into(),
                flavor_slug: "retail".into(),
                root_path: fake_root.to_string_lossy().into_owned(),
                addons_path: fake_addons.to_string_lossy().into_owned(),
                wtf_path: Some(fake_wtf.to_string_lossy().into_owned()),
                toc_suffixes: vec!["_Mainline.toc".into(), ".toc".into()],
                is_default: true,
            })
            .expect("Insertion instance");

        assert_eq!(created.name, "Retail");
        assert!(created.is_default);
        assert_eq!(created.toc_suffixes, vec!["_Mainline.toc", ".toc"]);
    }
}