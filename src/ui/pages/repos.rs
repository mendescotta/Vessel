use adw::prelude::*;

use super::{issues_group, list_group, page, switch_row, toggle_in};
use crate::profile::validate::required_presets;
use crate::profile::RepoPreset;
use crate::ui::state::{self, Shared};

pub fn build(s: &Shared) -> gtk::Widget {
    let page = page(
        "Repositories",
        "Choose package sources. The official Void repository is always included.",
    );
    page.add(&issues_group(s, &["repos"]));

    let presets = adw::PreferencesGroup::builder()
        .title("Repositories")
        .description("The official Void repository is always used, after these.")
        .build();
    let fix = adw::ActionRow::builder()
        .title("The chosen init or userland needs more repositories")
        .build();
    let fix_btn = gtk::Button::builder()
        .label("Add required")
        .valign(gtk::Align::Center)
        .css_classes(["suggested-action"])
        .build();
    fix.add_suffix(&fix_btn);
    presets.add(&fix);
    {
        let s = s.clone();
        fix_btn.connect_clicked(move |_| {
            state::update(&s, |p| {
                for r in required_presets(p) {
                    toggle_in(&mut p.repos.presets, r, true);
                }
            })
        });
    }
    let refresh_fix = {
        let fix = fix.clone();
        move |p: &crate::profile::Profile| {
            fix.set_visible(
                required_presets(p)
                    .iter()
                    .any(|r| !p.repos.presets.contains(r)),
            )
        }
    };
    refresh_fix(&s.borrow().profile);
    state::on_change(s, move |p, _, _| refresh_fix(p));

    for r in RepoPreset::ALL.iter().copied() {
        presets.add(&switch_row(
            s,
            r.label(),
            None,
            move |p| p.repos.presets.contains(&r),
            move |p, on| toggle_in(&mut p.repos.presets, r, on),
        ));
    }
    page.add(&presets);

    page.add(&list_group(
        s,
        "Custom repositories",
        "URLs or local paths, used before the presets.",
        |p| &p.repos.custom,
        |p| &mut p.repos.custom,
        false,
    ));
    page.upcast()
}
