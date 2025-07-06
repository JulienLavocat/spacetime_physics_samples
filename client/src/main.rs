use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_spacetimedb::{
    ReadStdbConnectedEvent, ReadStdbDisconnectedEvent, StdbConnectedEvent,
    StdbConnectionErrorEvent, StdbDisconnectedEvent, StdbPlugin, tables,
};
use module_bindings::{DbConnection, PlayersTableAccess};

mod module_bindings;

const MODULE_NAME: &str = "stdb-physics";
const STDB_URI: &str = "https://stdb.jlavocat.eu";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "info,wgpu_core=warn,wgpu_hal=error,naga=warn,client=debug".into(),
            ..Default::default()
        }))
        .add_plugins(
            StdbPlugin::default()
                .with_connection(|connected, disconnected, errored, _| {
                    let conn = DbConnection::builder()
                        .with_module_name(MODULE_NAME)
                        .with_uri(STDB_URI)
                        .on_connect(move |_, _, _| {
                            connected.send(StdbConnectedEvent {}).unwrap();
                        })
                        .on_disconnect(move |_, err| {
                            disconnected.send(StdbDisconnectedEvent { err }).unwrap();
                        })
                        .on_connect_error(move |_, err| {
                            errored.send(StdbConnectionErrorEvent { err }).unwrap();
                        });

                    let conn = conn.build().unwrap();

                    conn.run_threaded();

                    conn
                })
                .with_events(|plugin, app, db, _| {
                    tables!(players);
                }),
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(First, (on_connected, on_disconnected).chain())
        .run()
}

fn on_connected(mut events: ReadStdbConnectedEvent) {
    for _ in events.read() {
        info!("Connected to SpacetimeDB");
    }
}
fn on_disconnected(mut events: ReadStdbDisconnectedEvent) {
    for event in events.read() {
        info!("Disconnected from SpacetimeDB: {:?}", event.err);
    }
}
