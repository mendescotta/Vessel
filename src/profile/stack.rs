use super::validate::required_presets;
use super::{Init, Profile, Userland};

pub fn select(p: &mut Profile, init: Init, userland: Userland) {
    p.init = init;
    p.userland = userland;
    if p.vanilla {
        p.repos.presets.clear();
    }
    for r in required_presets(p) {
        if !p.repos.presets.contains(&r) {
            p.repos.presets.push(r);
        }
    }
}

#[cfg(test)]
#[path = "stack_tests.rs"]
mod tests;
