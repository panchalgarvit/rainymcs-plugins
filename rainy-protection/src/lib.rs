use pumpkin_plugin_api::{
    command::{Command, CommandError},
    commands::CommandHandler,
    common::NamedColor,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
    Context, Plugin, PluginMetadata, register_plugin,
};

struct ClaimCommandHandler;

impl CommandHandler for ClaimCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("🛡 Protected 32x32 block area! Only you can build here.")
            .color_named(NamedColor::Green);
        sender.send_message(msg);
        Ok(0)
    }
}

struct RainyProtection;

impl Plugin for RainyProtection {
    fn new() -> Self {
        RainyProtection
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "rainy-protection".into(),
            version: "1.0.0".into(),
            authors: vec!["RainyMCS Team".into()],
            description: "Spawn zone protection and anti-grief land claims".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let perm_claim = "rainy.claim";
        let _ = context.register_permission(&Permission {
            node: perm_claim.into(),
            description: "Access /claim".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let claim_cmd = Command::new(&["claim".to_string()], "Claim land against griefing")
            .execute(ClaimCommandHandler);
        context.register_command(claim_cmd, perm_claim);

        tracing::info!("[RainyProtection] Loaded /claim!");
        Ok(())
    }
}

register_plugin!(RainyProtection);
