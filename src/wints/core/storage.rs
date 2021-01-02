use crate::core::config::WintsConfig;
use crate::core::module::WintsModule;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct WintsStorage {
    local_basedir: PathBuf,
    local_modules: HashMap<String, WintsModule>,
    local_ignore_path: PathBuf,
    local_config: Option<WintsConfig>,

    global_basedir: PathBuf,
    global_modules: HashMap<String, WintsModule>,
    global_ignore_path: PathBuf,
    global_config: Option<WintsConfig>,
}

impl WintsStorage {
    pub fn load(local_basedir: PathBuf, global_basedir: PathBuf) -> Result<WintsStorage> {
        let mut storage = WintsStorage {
            local_basedir: local_basedir.clone(),
            local_modules: Default::default(),
            local_ignore_path: local_basedir.join("ignore"),
            local_config: None,
            global_basedir: global_basedir.clone(),
            global_modules: Default::default(),
            global_ignore_path: global_basedir.join("ignore"),
            global_config: None,
        };

        for entry in glob::glob(
            format!(
                "{}/modules/*.yaml",
                storage.local_basedir.as_path().display()
            )
            .as_str(),
        )? {
            if let Ok(path) = entry {
                let module_name = path
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                let module = WintsStorage::load_module(&path)?;
                storage.local_modules.insert(module_name, module);
            }
        }

        for entry in glob::glob(
            format!(
                "{}/modules/*.yaml",
                storage.global_basedir.as_path().display()
            )
            .as_str(),
        )? {
            if let Ok(path) = entry {
                let module_name = path
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                let module = WintsStorage::load_module(&path)?;
                storage.global_modules.insert(module_name, module);
            }
        }

        let local_config_path = storage.local_basedir.join("options.yaml");
        if local_config_path.exists() {
            storage.local_config = Some(WintsStorage::load_config(&local_config_path)?);
        }
        let global_config_path = storage.global_basedir.join("options.yaml");
        if global_config_path.exists() {
            storage.global_config = Some(WintsStorage::load_config(&global_config_path)?);
        }

        Ok(storage)
    }

    pub fn store(&self) -> Result<()> {
        for local_module in &self.local_modules {
            let path = self
                .local_basedir
                .join("modules")
                .join(format!("{}.yaml", local_module.0));
            WintsStorage::store_module(&local_module.1, path)?;
        }
        for global_module in &self.global_modules {
            let path = self
                .global_basedir
                .join("modules")
                .join(format!("{}.yaml", global_module.0));
            WintsStorage::store_module(&global_module.1, path)?;
        }
        if let Some(local_config) = &self.local_config {
            let path = self.local_basedir.join("options.yaml");
            WintsStorage::store_config(local_config, path)?;
        }
        if let Some(global_config) = &self.global_config {
            let path = self.global_basedir.join("options.yaml");
            WintsStorage::store_config(global_config, path)?;
        }
        Ok(())
    }

    pub fn ignore_url(&mut self, url: String, global_config: bool) {
        match global_config {
            true => {
                let global_config = match self.global_config.clone() {
                    None => WintsConfig {
                        version: 1,
                        ignored_urls: Some(vec![url]),
                    },
                    Some(mut gc) => {
                        let mut ignored_urls = gc.ignored_urls.unwrap_or_default();
                        ignored_urls.push(url);
                        gc.ignored_urls = Some(ignored_urls);
                        gc
                    }
                };
                self.global_config = Some(global_config);
            }
            false => {
                let local_config = match self.local_config.clone() {
                    None => WintsConfig {
                        version: 1,
                        ignored_urls: Some(vec![url]),
                    },
                    Some(mut lc) => {
                        let mut ignored_urls = lc.ignored_urls.unwrap_or_default();
                        ignored_urls.push(url);
                        lc.ignored_urls = Some(ignored_urls);
                        lc
                    }
                };
                self.local_config = Some(local_config);
            }
        }
    }

    pub fn ignore_glob(&self, glob: String, global_config: bool) -> Result<()> {
        let ignore_path = match global_config {
            true => self.global_ignore_path.clone(),
            false => self.local_ignore_path.clone(),
        };
        fs::create_dir_all(ignore_path.parent().unwrap())?;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(ignore_path)?;
        writeln!(file, "{}", glob)?;
        Ok(())
    }

    pub fn contextualise_urls(
        &mut self,
        module_name: String,
        is_global_module: bool,
        context: &str,
        urls: Vec<String>,
    ) {
        let module = match self.find_module_mut(module_name.clone(), is_global_module) {
            None => self.new_module(module_name, is_global_module),
            Some(module_mut) => module_mut,
        };
        WintsModule::contextualise_urls(module, context.to_string(), urls);
    }

    pub fn list_of_all_urls(&self, module_name: String) -> Vec<String> {
        match self.find_module(module_name) {
            None => vec![],
            Some(module) => module.list_of_all_urls(),
        }
    }

    pub fn ignored_urls(&self) -> Vec<String> {
        let mut ignored_urls: Vec<String> = Vec::new();
        if let Some(local_config) = self.local_config.clone() {
            ignored_urls.extend(
                local_config
                    .ignored_urls
                    .unwrap_or_default()
                    .iter()
                    .cloned(),
            );
        }
        if let Some(global_config) = self.global_config.clone() {
            ignored_urls.extend(
                global_config
                    .ignored_urls
                    .unwrap_or_default()
                    .iter()
                    .cloned(),
            );
        }
        ignored_urls
    }

    pub fn list_of_ignore_files(&self) -> Vec<PathBuf> {
        vec![
            self.local_ignore_path.clone(),
            self.global_ignore_path.clone(),
        ]
    }

    pub fn find_module(&self, module_name: String) -> Option<&WintsModule> {
        self.local_modules
            .get(&module_name)
            .or_else(move || self.global_modules.get(&module_name))
    }

    fn store_config(config: &WintsConfig, path: PathBuf) -> Result<()> {
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(&path)?;
        let config_content = serde_yaml::to_string(&config)?;
        file.write_all(config_content.as_bytes())?;
        Ok(())
    }

    fn store_module(module: &WintsModule, path: PathBuf) -> Result<()> {
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(&path)?;
        let module_content = serde_yaml::to_string(&module)?;
        file.write_all(module_content.as_bytes())?;
        Ok(())
    }

    fn load_config(path: &PathBuf) -> Result<WintsConfig> {
        let file = File::open(path)?;
        let config: WintsConfig = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    fn load_module(path: &PathBuf) -> Result<WintsModule> {
        let file = File::open(path)?;
        let module: WintsModule = serde_yaml::from_reader(file)?;
        Ok(module)
    }

    fn find_module_mut(
        &mut self,
        module_name: String,
        is_global_module: bool,
    ) -> Option<&mut WintsModule> {
        let lm = &mut self.local_modules;
        let gm = &mut self.global_modules;
        match is_global_module {
            true => gm.get_mut(&module_name),
            false => lm
                .get_mut(&module_name)
                .or_else(move || gm.get_mut(&module_name)),
        }
    }

    fn new_module(&mut self, module_name: String, is_global_module: bool) -> &mut WintsModule {
        let module = WintsModule::empty_module();
        self.add_module(module_name.clone(), module, is_global_module);
        self.find_module_mut(module_name, is_global_module).unwrap()
    }

    pub fn add_module(&mut self, module_name: String, module: WintsModule, is_global_module: bool) {
        match is_global_module {
            true => self.global_modules.insert(module_name, module),
            false => self.local_modules.insert(module_name, module),
        };
    }
}
