use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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

lazy_static::lazy_static! {
    static ref BALANCES: Arc<Mutex<HashMap<String, u64>>> = Arc::new(Mutex::new(HashMap::new()));
}

struct BalanceCommandExecutor;

#[async_trait]
impl CommandExecutor for BalanceCommandExecutor {
    fn execute<'a>(
        &'a self,
        sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            let balance = 250; // Starter balance
            sender
                .send_message(
                    TextComponent::text("💰 Your Balance: ")
                        .color_named(NamedColor::Gold)
                        .add_child(
                            TextComponent::text(format!("${}", balance))
                                .color_named(NamedColor::Green),
                        ),
                )
                .await;
            Ok(())
        })
    }
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();
    log::info!("[RainyEconomy] Loading Economy system v1.0.0...");

    let bal_cmd = CommandTree::new(["bal", "balance"], "Check your coin balance")
        .execute(BalanceCommandExecutor);
    let perm = Permission::new("rainy.balance", "Check balance", PermissionDefault::Allow);

    server.register_permission(perm).await?;
    server.register_command(bal_cmd, "rainy.balance").await;

    log::info!("[RainyEconomy] Registered /bal and /balance!");
    Ok(())
}

#[plugin_impl]
pub struct RainyEconomy {}

impl RainyEconomy {
    pub fn new() -> Self {
        RainyEconomy {}
    }
}

impl Default for RainyEconomy {
    fn default() -> Self {
        Self::new()
    }
}
