use adw::prelude::*;

use super::{issues_group, list_group, page, path_row};
use crate::ui::state::Shared;

pub fn build(s: &Shared) -> gtk::Widget {
    let page = page("Packages & customisation", "Add packages and services after choosing the base system. Vanilla profiles leave the package lists empty.");
    page.add(&issues_group(
        s,
        &["packages", "services", "overlay_dir", "post_rootfs_hook"],
    ));
    page.add(&list_group(
        s,
        "Extra packages",
        "Installed in addition to the selected components.",
        |p| &p.packages.extra,
        |p| &mut p.packages.extra,
        true,
    ));
    page.add(&list_group(
        s,
        "Excluded packages",
        "Ignored by xbps, including as dependencies.",
        |p| &p.packages.exclude,
        |p| &mut p.packages.exclude,
        false,
    ));
    page.add(&list_group(
        s,
        "Enable services",
        "Added to the selected components' services.",
        |p| &p.services.enable,
        |p| &mut p.services.enable,
        false,
    ));
    page.add(&list_group(
        s,
        "Disable services",
        "Removed even if a component enables them.",
        |p| &p.services.disable,
        |p| &mut p.services.disable,
        false,
    ));
    let files = adw::PreferencesGroup::builder()
        .title("Customisation")
        .build();
    files.add(&path_row(
        s,
        "Overlay folder",
        true,
        |p| p.overlay_dir.clone(),
        |p, v| p.overlay_dir = v,
    ));
    files.add(&path_row(
        s,
        "Post-rootfs hook script",
        false,
        |p| p.post_rootfs_hook.clone(),
        |p, v| p.post_rootfs_hook = v,
    ));
    page.add(&files);
    page.upcast()
}
