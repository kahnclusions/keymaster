#![allow(non_snake_case)]

pub mod app;

use app::{app, LocalKeybox};
use dioxus_desktop::{Config, WindowBuilder};

use color_eyre::eyre::{eyre, Result};
use dioxus_desktop::tao::platform::macos::WindowBuilderExtMacOS;
use sequoia_ipc::keybox::{Keybox, KeyboxRecord};
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::Cert;

use std::fs::File;

fn main() -> Result<()> {
    color_eyre::install()?;

    let f = File::open(shellexpand::tilde("~/.gnupg/pubring.kbx").to_string())?;
    let kbx = Keybox::from_reader(f).map_err(|e| eyre!("Error: {:?}", e))?;

    let certs = kbx
        .filter_map(|kbx_record| kbx_record.ok())
        .filter_map(|kbx_record| match kbx_record {
            KeyboxRecord::OpenPGP(r) => Some(r.cert()),
            _ => None,
        })
        .map(|v| v.map_err(|e| eyre!("Error: {:?}", e)))
        .collect::<Result<Vec<Cert>>>()?;

    let keybox = LocalKeybox { certs };

    let window = WindowBuilder::new()
        .with_title_hidden(true)
        .with_movable_by_window_background(false)
        .with_fullsize_content_view(true)
        .with_titlebar_buttons_hidden(false)
        .with_titlebar_transparent(true);

    dioxus_desktop::launch_with_props(
        app,
        keybox,
        Config::new()
            .with_window(window)
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script><style>* { -webkit-font-smoothing: antialiased; }</style>".to_string()),
    );
    Ok(())
}
