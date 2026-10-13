use adw::prelude::*;

use super::{enum_row, issues_group, page, stack, text_row};
use crate::profile::{Arch, RepoPreset};
use crate::ui::state::{self, Change, Shared};

pub fn build(s: &Shared) -> gtk::Widget {
    let page = page("System", "The base system: which init and core userland it ships, and how the live session identifies itself.");
    page.add(&issues_group(
        s,
        &["name", "arch", "kernel", "live", "repos"],
    ));
    page.add(&stack::group(s));

    let base = adw::PreferencesGroup::builder().title("Base").build();
    base.add(&text_row(
        s,
        "Profile name",
        |p| p.name.clone(),
        |p, v| p.name = v,
    ));
    base.add(&text_row(
        s,
        "Kernel package",
        |p| p.kernel.clone(),
        |p, v| p.kernel = v,
    ));
    let arch = enum_row(
        s,
        "C library",
        Arch::ALL,
        Arch::label,
        |p| p.arch,
        |p, arch| {
            p.arch = arch;
            if arch == Arch::X86_64Musl {
                p.repos.presets.retain(|r| *r != RepoPreset::Multilib);
            }
        },
    );
    arch.set_subtitle("glibc supports multilib; musl does not");
    base.add(&arch);
    page.add(&base);

    let live = adw::PreferencesGroup::builder()
        .title("Live session")
        .description("The live user and root default to password voidlinux. A custom root password is stored in the profile and build script with owner-only permissions.")
        .build();
    live.add(&text_row(
        s,
        "Hostname",
        |p| p.live.hostname.clone(),
        |p, v| p.live.hostname = v,
    ));
    live.add(&text_row(
        s,
        "Live user",
        |p| p.live.user.clone(),
        |p, v| p.live.user = v,
    ));
    let password = adw::PasswordEntryRow::builder()
        .title("Root password (optional)")
        .text(
            s.borrow()
                .profile
                .live
                .root_password
                .as_deref()
                .unwrap_or(""),
        )
        .build();
    {
        let s = s.clone();
        password.connect_changed(move |row| {
            let value = row.text().to_string();
            state::update(&s, |p| {
                p.live.root_password = (!value.is_empty()).then_some(value)
            });
        });
    }
    {
        let password = password.clone();
        state::on_change(s, move |p, _, change| {
            if change == Change::Replace {
                let value = p.live.root_password.as_deref().unwrap_or("");
                if password.text() != value {
                    password.set_text(value);
                }
            }
        });
    }
    live.add(&password);
    live.add(&text_row(
        s,
        "Locale",
        |p| p.live.locale.clone(),
        |p, v| p.live.locale = v,
    ));
    live.add(&text_row(
        s,
        "Keymap",
        |p| p.live.keymap.clone(),
        |p, v| p.live.keymap = v,
    ));
    live.add(&text_row(
        s,
        "Timezone",
        |p| p.live.timezone.clone(),
        |p, v| p.live.timezone = v,
    ));
    page.add(&live);

    page.upcast()
}
