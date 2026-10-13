use adw::prelude::*;

use super::{enum_row, switch_row};
use crate::profile::stack::select;
use crate::profile::validate::required_presets;
use crate::profile::{Init, Profile, Userland};
use crate::ui::state::{self, Shared};

pub fn group(s: &Shared) -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::builder()
        .title("Base system")
        .description("Choose the init and which packages may be added to the ISO.")
        .build();
    group.add(&enum_row(
        s,
        "Init system",
        Init::ALL,
        Init::label,
        |p| p.init,
        |p, init| select(p, init, Userland::Gnu),
    ));
    let userland = adw::ActionRow::builder()
        .title("Core userland")
        .subtitle(s.borrow().profile.userland.label())
        .build();
    group.add(&userland);
    group.add(&switch_row(
        s,
        "Vanilla package set",
        Some("Runit: official Void console image. Dinit: required boot packages only; no Caerus, Voyage or voidhw."),
        |p| p.vanilla,
        |p, on| {
            p.vanilla = on;
            if on {
                p.repos.presets = required_presets(p);
                p.repos.custom.clear();
                p.packages.extra.clear();
                p.packages.exclude.clear();
            }
        },
    ));
    let summary = adw::ActionRow::builder()
        .title("Required base packages")
        .subtitle_selectable(true)
        .build();
    group.add(&summary);
    let refresh = move |p: &Profile| {
        summary.set_subtitle(&crate::generate::init::base_packages(p.init).join(", "));
    };
    refresh(&s.borrow().profile);
    state::on_change(s, move |p, _, _| refresh(p));
    group
}
