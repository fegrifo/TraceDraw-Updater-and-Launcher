#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result, bail};
use eframe::egui;
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

const REPO: &str = "tracedraw/tracedraw";
const MIME_TYPES: &[(&str, &str)] = &[
    (".cdr", "application/x-coreldraw"),
    (".tdraw", "application/x-tracedraw"),
    (".cmx", "image/x-coreldraw-cmx"),
    (".svg", "image/svg+xml"),
    (".svgz", "image/svg+xml-compressed"),
    (".ai", "application/illustrator"),
    (".eps", "image/x-eps"),
    (".dxf", "image/vnd.dxf"),
    (".plt", "application/vnd.hp-hpgl"),
];

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

enum Message {
    Release(String),
    Installed { version: String, status: String },
    Error(String),
}

struct Launcher {
    install_dir: PathBuf,
    local_version: String,
    latest_version: String,
    status: String,
    busy: bool,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl Launcher {
    fn new() -> Self {
        let base = dirs_fallback_home().join(".local/opt/tracedraw");
        let (tx, rx) = mpsc::channel();
        let mut app = Self {
            install_dir: base,
            local_version: "não instalado".into(),
            latest_version: "verificando…".into(),
            status: "Pronto".into(),
            busy: false,
            tx,
            rx,
        };
        app.refresh_local_version();
        app.check_latest();
        app
    }

    fn refresh_local_version(&mut self) {
        self.local_version = if self.install_dir.join("tracedraw").is_file() {
            "instalado".into()
        } else {
            "não instalado".into()
        };
    }

    fn check_latest(&mut self) {
        if self.busy { return; }
        self.busy = true;
        self.status = "Consultando as versões publicadas no GitHub…".into();
        let tx = self.tx.clone();
        thread::spawn(move || match get_release() {
            Ok(release) => { let _ = tx.send(Message::Release(release.tag_name)); }
            Err(e) => { let _ = tx.send(Message::Error(format!("Falha ao consultar o GitHub: {e:#}"))); }
        });
    }

    fn install_latest(&mut self) {
        if self.busy { return; }
        let target = self.install_dir.clone();
        let tx = self.tx.clone();
        self.busy = true;
        self.status = "Baixando e instalando…".into();
        thread::spawn(move || match download_and_install(&target) {
            Ok(version) => {
                let integration = register_desktop(&target);
                let result = match integration {
                    Ok(()) => format!("{version} — instalação e integração concluídas."),
                    Err(e) => format!("{version} — instalado; integração falhou: {e:#}"),
                };
                let _ = tx.send(Message::Installed { version, status: result });
            }
            Err(e) => { let _ = tx.send(Message::Error(format!("Não foi possível instalar: {e:#}"))); }
        });
    }

    fn install_desktop_entry(&mut self) {
        match register_desktop(&self.install_dir) {
            Ok(()) => self.status = "Atalho e associações de arquivos registrados para este usuário.".into(),
            Err(e) => self.status = format!("Falha ao registrar integração: {e:#}"),
        }
    }

    fn launch(&mut self) {
        let binary = self.install_dir.join("tracedraw");
        if !binary.is_file() {
            self.status = "TraceDraw ainda não está instalado. Clique em Baixar / Atualizar.".into();
            return;
        }
        match Command::new(binary).spawn() {
            Ok(_) => self.status = "TraceDraw aberto.".into(),
            Err(e) => self.status = format!("Não foi possível abrir TraceDraw: {e}"),
        }
    }

    fn receive_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            self.busy = false;
            match msg {
                Message::Release(v) => {
                    self.latest_version = v;
                    self.status = "Versão mais recente encontrada.".into();
                }
                Message::Installed { version, status } => {
                    self.local_version = version.clone();
                    self.latest_version = version;
                    self.status = status;
                }
                Message::Error(e) => self.status = e,
            }
        }
    }
}

impl eframe::App for Launcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.receive_messages();
        ctx.request_repaint_after(Duration::from_millis(150));
        egui::CentralPanel::default().frame(egui::Frame::default().inner_margin(egui::Margin::same(28))).show(ctx, |ui| {
            ui.heading("TraceDraw Launcher");
            ui.add_space(18.0);
            ui.label(format!("Versão local: {}", self.local_version));
            ui.label(format!("Última versão no GitHub: {}", self.latest_version));
            ui.add_space(22.0);
            ui.label(&self.status);
            ui.add_space(22.0);
            ui.horizontal(|ui| {
                if ui.add_enabled(!self.busy, egui::Button::new("Baixar / Atualizar TraceDraw")).clicked() { self.install_latest(); }
                if ui.add_enabled(!self.busy, egui::Button::new("Abrir TraceDraw")).clicked() { self.launch(); }
            });
            ui.add_space(12.0);
            if ui.add_enabled(!self.busy, egui::Button::new("Registrar para abrir arquivos (CDR, SVG, TDraw…)")).clicked() { self.install_desktop_entry(); }
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label("Pasta de instalação:");
                ui.monospace(self.install_dir.display().to_string());
                if ui.button("Escolher…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_directory(&self.install_dir).pick_folder() { self.install_dir = path; self.refresh_local_version(); }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 390.0]).with_min_inner_size([620.0, 330.0]),
        ..Default::default()
    };
    eframe::run_native("TraceDraw Launcher", options, Box::new(|_cc| Ok(Box::new(Launcher::new()))))
}

fn client() -> Result<Client> {
    Ok(Client::builder().user_agent("TraceDraw-Launcher/0.1").build()?)
}

fn get_release() -> Result<Release> {
    let release = client()?.get(format!("https://api.github.com/repos/{REPO}/releases/latest"))
        .send()?.error_for_status()?.json::<Release>()?;
    Ok(release)
}

fn download_and_install(target: &Path) -> Result<String> {
    let release = get_release()?;
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        other => bail!("Arquitetura Linux não suportada pelo launcher: {other}"),
    };
    let suffix = format!("linux-{arch}.tar.gz");
    let asset = release.assets.iter().find(|a| a.name.ends_with(&suffix))
        .with_context(|| format!("O release {} não contém o pacote {suffix}", release.tag_name))?;
    let response = client()?.get(&asset.browser_download_url).send()?.error_for_status()?;
    let temp_root = std::env::temp_dir().join(format!("tracedraw-launcher-{}", std::process::id()));
    if temp_root.exists() { fs::remove_dir_all(&temp_root)?; }
    fs::create_dir_all(&temp_root)?;
    let archive_path = temp_root.join("release.tar.gz");
    let mut response = response;
    let mut archive_file = fs::File::create(&archive_path)?;
    io::copy(&mut response, &mut archive_file)?;
    drop(archive_file);
    let mut archive = tar::Archive::new(GzDecoder::new(fs::File::open(&archive_path)?));
    let staged = temp_root.join("payload");
    fs::create_dir_all(&staged)?;
    let mut found = [false, false];
    for item in archive.entries()? {
        let mut entry = item?;
        let Some(name) = entry.path()?.file_name().and_then(|s| s.to_str()).map(str::to_owned) else { continue; };
        let idx = match name.as_str() { "tracedraw" => 0, "tracedraw-cli" => 1, _ => continue };
        let dest = staged.join(&name);
        entry.unpack(&dest)?;
        found[idx] = true;
    }
    if !found.into_iter().all(|v| v) { bail!("O pacote não continha os executáveis tracedraw e tracedraw-cli"); }
    set_executable(&staged.join("tracedraw"))?;
    set_executable(&staged.join("tracedraw-cli"))?;
    fs::create_dir_all(target).with_context(|| format!("Não foi possível criar {}", target.display()))?;
    for name in ["tracedraw", "tracedraw-cli"] {
        let src = staged.join(name);
        let tmp = target.join(format!(".{name}.new"));
        let dest = target.join(name);
        fs::copy(&src, &tmp)?;
        fs::rename(&tmp, &dest).with_context(|| format!("Não foi possível substituir {name}"))?;
    }
    let _ = fs::remove_dir_all(temp_root);
    Ok(release.tag_name)
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<()> { Ok(()) }

fn register_desktop(install_dir: &Path) -> Result<()> {
    let home = dirs_fallback_home();
    let app_dir = home.join(".local/share/applications");
    let mime_dir = home.join(".local/share/mime/packages");
    fs::create_dir_all(&app_dir)?;
    fs::create_dir_all(&mime_dir)?;
    let binary = install_dir.join("tracedraw");
    let escaped_binary = binary.display().to_string().replace('\\', "\\\\").replace('"', "\\\"");
    let mime_list = MIME_TYPES.iter().map(|(_, mime)| *mime).collect::<Vec<_>>().join(";");
    let desktop = format!("[Desktop Entry]\nType=Application\nName=TraceDraw\nComment=Editor vetorial\nExec=\"{}\" %F\nTerminal=false\nCategories=Graphics;VectorGraphics;\nMimeType={};\n", escaped_binary, mime_list);
    fs::write(app_dir.join("tracedraw.desktop"), desktop)?;
    let mut globs = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<mime-info xmlns=\"http://www.freedesktop.org/standards/shared-mime-info\">\n");
    for (ext, mime) in MIME_TYPES {
        globs.push_str(&format!("  <mime-type type=\"{mime}\"><comment>TraceDraw document</comment><glob pattern=\"*{ext}\"/></mime-type>\n"));
    }
    globs.push_str("</mime-info>\n");
    fs::write(mime_dir.join("tracedraw.xml"), globs)?;
    let _ = Command::new("update-mime-database").arg(home.join(".local/share/mime")).status();
    let _ = Command::new("update-desktop-database").arg(&app_dir).status();
    for (_, mime) in MIME_TYPES {
        let _ = Command::new("xdg-mime").args(["default", "tracedraw.desktop", mime]).status();
    }
    Ok(())
}

fn dirs_fallback_home() -> PathBuf {
    std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
}

