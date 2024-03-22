// globals.rs
//
// Copyright (c) 2024 Max Rodriguez
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::VERSION;
use adw::gtk::License;
use libadwaita as adw;

#[cfg(debug_assertions)]
pub static DEVELOPMENT_BUILD: bool = true;
#[cfg(not(debug_assertions))]
pub static DEVELOPMENT_BUILD: bool = false;

pub struct AboutInformation {
    pub app_name: &'static str,
    pub app_title: &'static str,
    pub app_version: &'static str,
    pub app_id: &'static str,
    pub app_repo: &'static str,
    pub authors: &'static [&'static str],
    pub artists: Option<&'static [&'static str]>,
    pub documenters: Option<&'static [&'static str]>,
    pub copyright: &'static str,
    pub license: &'static str,
    pub license_type: License,
    pub comments: &'static str,
}

pub static APP_INFO: AboutInformation = AboutInformation {
    app_name: env!("CARGO_PKG_NAME"),
    app_title: {
        match DEVELOPMENT_BUILD {
            false => "JW Library",
            true => "JW Library (Dev)",
        }
    },
    app_version: VERSION,
    app_repo: "https://github.com/maxrdz/jw-library-gtk",
    app_id: {
        match DEVELOPMENT_BUILD {
            false => "com.maxrdz.JWLibraryGTK",
            true => "com.maxrdz.JWLibraryGTK.Dev",
        }
    },
    authors: &[
        "Max Rodriguez <me@maxrdz.com>",
    ],
    artists: None,
    documenters: None,
    copyright: "© 2024 Max Rodriguez",
    license: "GNU General Public License v3.0",
    license_type: License::Gpl30,
    comments: "A FOSS Linux mobile friendly app alternative for JW Library.",
};
