use pumpkin_plugin_api::{
    command::{Command, CommandError},
    commands::CommandHandler,
    common::NamedColor,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
    Context, Plugin, PluginMetadata, register_plugin,
};

struct BalanceCommandHandler;

impl CommandHandler for BalanceCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("💰 Your Balance: ")
            .color_named(NamedColor::Gold)
            .add_child(
                TextComponent::text("$250.00 Coins")
                    .color_named(NamedColor::Green),
            );
        sender.send_message(msg);
        Ok(0)
    }
}

struct RainyEconomy;

impl Plugin for RainyEconomy {
    fn new() -> Self {
        RainyEconomy
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "rainy-economy".into(),
            version: "1.0.0".into(),
            authors: vec!["RainyMCS Team".into()],
            description: "Player balances and economy system".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let perm_bal = "rainy.balance";
        let _ = context.register_permission(&Permission {
            node: perm_bal.into(),
            description: "Access /bal and /balance".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let bal_cmd = Command::new(&["bal".to_string(), "balance".to_string()], "Check your balance")
            .execute(BalanceCommandHandler);
        context.register_command(bal_cmd, perm_bal);

        tracing::info!("[RainyEconomy] Loaded /bal and /balance!");
        Ok(())
    }
}

register_plugin!(RainyEconomy);
