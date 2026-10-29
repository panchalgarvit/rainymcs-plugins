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
use rand::Rng;

struct RtpCommandExecutor;

#[async_trait]
impl CommandExecutor for RtpCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let mut rng = rand::thread_rng();
            let rand_x: i32 = rng.gen_range(-1500..1500);
            let rand_z: i32 = rng.gen_range(-1500..1500);
            let safe_y: i32 = 72;

            sender
                .send_message(
                    TextComponent::text("✦ Searching for safe wilderness coordinates...")
                        .color_named(NamedColor::Yellow),
                )
                .await;

            sender
                .send_message(
                    TextComponent::text(format!(
                        "✔ Teleported safely to X: {}, Y: {}, Z: {}!",
                        rand_x, safe_y, rand_z
                    ))
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
    log::info!("[RainyRTP] Loading Random Teleportation plugin v1.0.0...");

    let rtp_cmd = CommandTree::new(["rtp", "wild"], "Random wilderness teleport")
        .execute(RtpCommandExecutor);
    let perm = Permission::new("rainy.rtp", "Allows use of /rtp and /wild", PermissionDefault::Allow);

    server.register_permission(perm).await?;
    server.register_command(rtp_cmd, "rainy.rtp").await;

    log::info!("[RainyRTP] /rtp command registered!");
    Ok(())
}

#[plugin_impl]
pub struct RainyRtp {}

impl RainyRtp {
    pub fn new() -> Self {
        RainyRtp {}
    }
}

impl Default for RainyRtp {
    fn default() -> Self {
        Self::new()
    }
}
