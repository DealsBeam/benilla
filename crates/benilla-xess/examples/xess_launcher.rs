use benilla_app::{AppExit, BuildId};
use benilla_xess::run;

fn main() -> AppExit {
    let build = BuildId {
        version: env!("CARGO_PKG_VERSION"),
        profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
        ..Default::default()
    };

    run(build)
}
