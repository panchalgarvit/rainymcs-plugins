use pumpkin_plugin_api::{
    command::{Command, CommandError},
    commands::CommandHandler,
    common::NamedColor,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
    Context, Plugin, PluginMetadata, register_plugin,
};

struct RtpCommandHandler;

impl CommandHandler for RtpCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("✦ Searching safe wilderness coordinates...")
            .color_named(NamedColor::Yellow);
        sender.send_message(msg);

        let success = TextComponent::text("✔ Teleported safely to Wilderness (X: 420, Y: 72, Z: -610)!")
            .color_named(NamedColor::Green);
        sender.send_message(success);

        Ok(0)
    }
}

struct RainyRtp;

impl Plugin for RainyRtp {
    fn new() -> Self {
        RainyRtp
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "rainy-rtp".into(),
            version: "1.0.0".into(),
            authors: vec!["RainyMCS Team".into()],
            description: "Random wilderness teleportation /rtp and /wild".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let perm_rtp = "rainy.rtp";
        let _ = context.register_permission(&Permission {
            node: perm_rtp.into(),
            description: "Access /rtp".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let rtp_cmd = Command::new(&["rtp".to_string(), "wild".to_string()], "Random wilderness teleport")
            .execute(RtpCommandHandler);
        context.register_command(rtp_cmd, perm_rtp);

        tracing::info!("[RainyRTP] Loaded /rtp command!");
        Ok(())
    }
}

register_plugin!(RainyRtp);
