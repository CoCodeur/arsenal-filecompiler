use anyhow::{Context, Result};
use log::info;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOpenOptions, SmbOptions};
use std::io::Read;
use std::path::Path;

use crate::models::NasConfig;

/// Client for accessing files on a NAS via SMB/CIFS protocol
pub struct NasConnection {
    client: SmbClient,
}

impl NasConnection {
    /// Connect to the NAS using the provided configuration
    pub fn connect(config: &NasConfig) -> Result<Self> {
        let smb_url = format!("smb://{}", config.server);
        info!("Connexion SMB à {}", smb_url);

        let client = SmbClient::new(
            SmbCredentials::default()
                .server(&smb_url)
                .share(&config.share_path)
                .username(&config.username)
                .password(&config.password)
                .workgroup("WORKGROUP"),
            SmbOptions::default().one_share_per_server(true),
        )
        .map_err(|e| anyhow::anyhow!("Impossible de se connecter au NAS: {:?}", e))?;

        Ok(Self { client })
    }

    /// List files in a directory on the NAS
    pub fn list_dir(&self, remote_path: &str) -> Result<Vec<SmbDirent>> {
        self.client
            .list_dir(remote_path)
            .map_err(|e| anyhow::anyhow!("Impossible de lister {} : {:?}", remote_path, e))
    }

    /// Read a file from the NAS into memory
    pub fn read_file(&self, remote_path: &str) -> Result<Vec<u8>> {
        let mut file = self
            .client
            .open_with(remote_path, SmbOpenOptions::default().read(true))
            .map_err(|e| anyhow::anyhow!("Impossible d'ouvrir {} : {:?}", remote_path, e))?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .with_context(|| format!("Impossible de lire : {}", remote_path))?;
        Ok(buf)
    }

    /// Copy a file from the NAS to a local path
    pub fn copy_to_local(&self, remote_path: &str, local_path: &Path) -> Result<()> {
        let data = self.read_file(remote_path)?;

        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Impossible de créer le dossier : {:?}", parent))?;
        }

        std::fs::write(local_path, &data)
            .with_context(|| format!("Impossible d'écrire : {:?}", local_path))?;

        Ok(())
    }

    /// Search for a PDF file matching a reference in a category directory tree.
    pub fn find_pdf(&self, category_path: &str, reference: &str) -> Result<Option<String>> {
        // Convert category like "Electricité / SR04 : Gestion / Motorisation"
        // to a filesystem path
        let parts: Vec<&str> = category_path
            .split('/')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut search_path = String::new();
        for part in &parts {
            if search_path.is_empty() {
                search_path = part.to_string();
            } else {
                search_path = format!("{}/{}", search_path, part);
            }
        }

        // Look for files matching the reference
        let target_name = format!("{}.pdf", reference);
        match self.list_dir(&search_path) {
            Ok(entries) => {
                for entry in entries {
                    let name = entry.name();
                    if name.eq_ignore_ascii_case(&target_name)
                        || name
                            .to_lowercase()
                            .starts_with(&reference.to_lowercase())
                    {
                        return Ok(Some(format!("{}/{}", search_path, name)));
                    }
                }
                Ok(None)
            }
            Err(_) => Ok(None),
        }
    }
}

/// Test if a NAS connection can be established
pub fn test_connection(config: &NasConfig) -> Result<String> {
    let conn = NasConnection::connect(config)?;
    match conn.list_dir("/") {
        Ok(entries) => {
            let dirs: Vec<_> = entries
                .iter()
                .filter(|e| matches!(e.get_type(), SmbDirentType::Dir))
                .collect();
            Ok(format!(
                "Connexion réussie ! {} dossiers trouvés à la racine du partage.",
                dirs.len()
            ))
        }
        Err(e) => Err(anyhow::anyhow!(
            "Connexion établie mais impossible de lister les fichiers : {}",
            e
        )),
    }
}
