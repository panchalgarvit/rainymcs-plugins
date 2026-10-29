use std::sync::Arc;
use async_trait::async_trait;

use pumpkin::{
    command::{
        args::ConsumedArgs,
        tree::CommandTree,
        CommandExecutor, CommandResult, CommandSender,
    },
    plugin::Context,
    server::Server,
};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::permission::{Permission, PermissionDefault};
use pumpkin_util::text::{color::NamedColor, TextComponent};

struct BroadcastCommandExecutor;

#[async_trait]
impl CommandExecutor for BroadcastCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            sender
                .send_message(
                    TextComponent::text("📢 [ANNOUNCEMENT] Welcome to the server!")
                        .color_named(NamedColor::Gold),
                )
                .await;
            Ok(())
        })
    }
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();
    log::info!("[RainyChat] Loading chat formatting & tools v1.0.0...");

    let broadcast_cmd = CommandTree::new(["broadcast", "bc"], "Send a server-wide announcement")
        .execute(BroadcastCommandExecutor);
    let perm = Permission::new("rainy.broadcast", "Use broadcast", PermissionDefault::Allow);

    server.register_permission(perm).await?;
    server.register_command(broadcast_cmd, "rainy.broadcast").await;

    log::info!("[RainyChat] Registered /broadcast!");
    Ok(())
}

#[plugin_impl]
pub struct RainyChat {}

impl RainyChat {
    pub fn new() -> Self {
        RainyChat {}
    }
}

impl Default for RainyChat {
    fn default() -> Self {
        Self::new()
    }
}
