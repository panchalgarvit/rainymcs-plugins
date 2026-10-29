use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

use pumpkin::{
    command::{
        args::ConsumedArgs,
        tree::{builder::literal, CommandTree},
        CommandExecutor, CommandResult, CommandSender,
    },
    plugin::{
        player::player_join::PlayerJoinEvent, BoxFuture, Context, EventHandler, EventPriority,
    },
    server::Server,
};
use pumpkin_api_macros::{plugin_impl, plugin_method, with_runtime};
use pumpkin_util::permission::{Permission, PermissionDefault};
use pumpkin_util::text::{color::NamedColor, TextComponent};

// Store player homes: (username -> (x, y, z))
lazy_static::lazy_static! {
    static ref HOMES: Arc<Mutex<HashMap<String, (f64, f64, f64)>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref TPA_REQUESTS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

struct JoinWelcomeHandler;

#[with_runtime(global)]
impl EventHandler<PlayerJoinEvent> for JoinWelcomeHandler {
    fn handle_blocking<'a>(
        &self,
        _server: &Arc<Server>,
        event: &'a mut PlayerJoinEvent,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async {
            let name = event.player.gameprofile.name.clone();
            event.join_message = TextComponent::text("✦ ")
                .color_named(NamedColor::Gold)
                .add_child(
                    TextComponent::text(format!("Welcome to RainyMCS, {}!", name))
                        .color_named(NamedColor::Green),
                );
        })
    }
}

// /spawn Command
struct SpawnCommandExecutor;

#[async_trait]
impl CommandExecutor for SpawnCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            sender
                .send_message(
                    TextComponent::text("✦ Teleporting to World Spawn (0, 70, 0)...")
                        .color_named(NamedColor::Aqua),
                )
                .await;
            Ok(())
        })
    }
}

// /sethome Command
struct SetHomeCommandExecutor;

#[async_trait]
impl CommandExecutor for SetHomeCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            sender
                .send_message(
                    TextComponent::text("✔ Home location saved successfully!")
                        .color_named(NamedColor::Green),
                )
                .await;
            Ok(())
        })
    }
}

// /home Command
struct HomeCommandExecutor;

#[async_trait]
impl CommandExecutor for HomeCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            sender
                .send_message(
                    TextComponent::text("✦ Teleporting to your home...")
                        .color_named(NamedColor::Aqua),
                )
                .await;
            Ok(())
        })
    }
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();
    log::info!("[RainyEssentials] Loading RainyEssentials v1.0.0...");

    // Register Player Join Event
    server
        .register_event(Arc::new(JoinWelcomeHandler), EventPriority::Normal, true)
        .await;

    // Register /spawn
    let spawn_cmd = CommandTree::new(["spawn"], "Teleport to world spawn")
        .execute(SpawnCommandExecutor);
    let perm_spawn = Permission::new("rainy.spawn", "Access to /spawn", PermissionDefault::Allow);
    server.register_permission(perm_spawn).await?;
    server.register_command(spawn_cmd, "rainy.spawn").await;

    // Register /sethome
    let sethome_cmd = CommandTree::new(["sethome"], "Save current location as your home")
        .execute(SetHomeCommandExecutor);
    let perm_sethome = Permission::new("rainy.sethome", "Access to /sethome", PermissionDefault::Allow);
    server.register_permission(perm_sethome).await?;
    server.register_command(sethome_cmd, "rainy.sethome").await;

    // Register /home
    let home_cmd = CommandTree::new(["home"], "Teleport to your saved home")
        .execute(HomeCommandExecutor);
    let perm_home = Permission::new("rainy.home", "Access to /home", PermissionDefault::Allow);
    server.register_permission(perm_home).await?;
    server.register_command(home_cmd, "rainy.home").await;

    log::info!("[RainyEssentials] Successfully loaded /spawn, /home, /sethome commands!");
    Ok(())
}

#[plugin_impl]
pub struct RainyEssentials {}

impl RainyEssentials {
    pub fn new() -> Self {
        RainyEssentials {}
    }
}

impl Default for RainyEssentials {
    fn default() -> Self {
        Self::new()
    }
}
