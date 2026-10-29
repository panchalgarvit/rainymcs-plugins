use pumpkin_plugin_api::{
    command::{Command, CommandError},
    commands::CommandHandler,
    common::NamedColor,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
    Context, Plugin, PluginMetadata, register_plugin,
};

struct BroadcastCommandHandler;

impl CommandHandler for BroadcastCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("📢 [ANNOUNCEMENT] Welcome to the server!")
            .color_named(NamedColor::Gold);
        sender.send_message(msg);
        Ok(0)
    }
}

struct RainyChat;

impl Plugin for RainyChat {
    fn new() -> Self {
        RainyChat
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "rainy-chat".into(),
            version: "1.0.0".into(),
            authors: vec!["RainyMCS Team".into()],
            description: "Server announcement /broadcast and chat tools".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let perm_bc = "rainy.broadcast";
        let _ = context.register_permission(&Permission {
            node: perm_bc.into(),
            description: "Access /broadcast".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let bc_cmd = Command::new(&["broadcast".to_string(), "bc".to_string()], "Broadcast announcement")
            .execute(BroadcastCommandHandler);
        context.register_command(bc_cmd, perm_bc);

        tracing::info!("[RainyChat] Loaded /broadcast!");
        Ok(())
    }
}

register_plugin!(RainyChat);
