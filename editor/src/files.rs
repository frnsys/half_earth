use std::{fmt::Display, path::PathBuf, sync::Arc};

use egui_file_dialog::{DialogMode, FileDialog};
use hes_engine::World;

use crate::{TOASTS, validate};

const EXTENSION: &str = "world";

pub struct FilePicker {
    file: Option<PathBuf>,
    dialog: FileDialog,
}

#[derive(Debug)]
pub enum FileError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Validation(Vec<String>),
}

impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        FileError::Io(e)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(e: serde_json::Error) -> Self {
        FileError::Json(e)
    }
}

impl Display for FileError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileError::Io(inner) => inner.to_string(),
                FileError::Json(inner) => inner.to_string(),
                FileError::Validation(inner) =>
                    inner.join(", ").to_string(),
            }
        )
    }
}

impl Default for FilePicker {
    fn default() -> Self {
        let mut dialog = FileDialog::new();

        dialog = dialog
            .add_file_filter(
                "World files",
                Arc::new(|path| {
                    path.extension().unwrap_or_default()
                        == EXTENSION
                }),
            )
            .default_file_filter("World files")
            .default_file_name("my.world");

        Self { file: None, dialog }
    }
}

impl FilePicker {
    pub fn filename(&self) -> Option<String> {
        self.file
            .as_ref()
            .map(|file| file.display().to_string())
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        world: &mut World,
    ) -> Result<(), FileError> {
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                let errs = validate::validate(world);
                if !errs.is_empty() {
                    return Err(FileError::Validation(errs));
                }

                let name = if let Some(name) = self.filename() {
                    format!("{name}.world")
                } else {
                    "my.world".to_string()
                };
                self.dialog.config_mut().default_file_name =
                    name;
                self.dialog.save_file();
            }
            if ui.button("Load").clicked() {
                self.dialog.pick_file();
            }
            Ok(())
        })
        .inner?;

        let mode = self.dialog.mode();
        self.dialog.update(ui.ctx());
        if let Some(path) = self.dialog.take_picked() {
            if mode == DialogMode::SaveFile {
                let data = serde_json::to_string(world)?;
                TOASTS.lock().success("Successfully saved.");
                fs_err::write(&path, data)?;
            } else {
                let data = fs_err::read_to_string(&path)?;
                *world = serde_json::from_str(&data)?;
            }
            self.file = Some(path);
        }
        Ok(())
    }
}
