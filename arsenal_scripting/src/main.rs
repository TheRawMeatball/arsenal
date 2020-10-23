use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(arsenal_scripting::ScriptingPlugin::new(
            &std::env::args()
                .skip(1)
                .next()
                .unwrap_or(".".to_string()),
        ))
        .run();
}