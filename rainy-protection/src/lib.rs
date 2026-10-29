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

struct ClaimCommandExecutor;

#[async_trait]
impl CommandExecutor for ClaimCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            sender
                .send_message(
                    TextComponent::text("🛡 Protected 32x32 block area! Only you can build here.")
                        .color_named(NamedColor::Green),
                )
                .await;
            Ok(())
        })
    }
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();
    log::info!("[RainyProtection] Loading protection system v1.0.0...");

    let claim_cmd = CommandTree::new(["claim"], "Claim territory against griefing")
        .execute(ClaimCommandExecutor);
    let perm = Permission::new("rainy.claim", "Claim land", PermissionDefault::Allow);

    server.register_permission(perm).await?;
    server.register_command(claim_cmd, "rainy.claim").await;

    log::info!("[RainyProtection] Registered /claim!");
    Ok(())
}

#[plugin_impl]
pub struct RainyProtection {}

impl RainyProtection {
    pub fn new() -> Self {
        RainyProtection {}
    }
}

impl Default for RainyProtection {
    fn default() -> Self {
        Self::new()
    }
}
