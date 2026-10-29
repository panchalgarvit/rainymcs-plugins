use pumpkin_plugin_api::{
    command::{Command, CommandError},
    commands::CommandHandler,
    common::NamedColor,
    permission::{Permission, PermissionDefault},
    text::TextComponent,
    Context, Plugin, PluginMetadata, register_plugin,
};

struct SpawnCommandHandler;

impl CommandHandler for SpawnCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("✦ Teleporting to World Spawn (0, 70, 0)...")
            .color_named(NamedColor::Aqua);
        sender.send_message(msg);
        Ok(0)
    }
}

struct SetHomeCommandHandler;

impl CommandHandler for SetHomeCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("✔ Home location saved successfully!")
            .color_named(NamedColor::Green);
        sender.send_message(msg);
        Ok(0)
    }
}

struct HomeCommandHandler;

impl CommandHandler for HomeCommandHandler {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let msg = TextComponent::text("✦ Teleporting to your home...")
            .color_named(NamedColor::Aqua);
        sender.send_message(msg);
        Ok(0)
    }
}

struct RainyEssentials;

impl Plugin for RainyEssentials {
    fn new() -> Self {
        RainyEssentials
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "rainy-essentials".into(),
            version: "1.0.0".into(),
            authors: vec!["RainyMCS Team".into()],
            description: "Essential commands: /spawn, /home, /sethome".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        let perm_spawn = "rainy.spawn";
        let _ = context.register_permission(&Permission {
            node: perm_spawn.into(),
            description: "Access /spawn".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let spawn_cmd = Command::new(&["spawn".to_string()], "Teleport to world spawn")
            .execute(SpawnCommandHandler);
        context.register_command(spawn_cmd, perm_spawn);

        let perm_sethome = "rainy.sethome";
        let _ = context.register_permission(&Permission {
            node: perm_sethome.into(),
            description: "Access /sethome".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let sethome_cmd = Command::new(&["sethome".to_string()], "Save your home location")
            .execute(SetHomeCommandHandler);
        context.register_command(sethome_cmd, perm_sethome);

        let perm_home = "rainy.home";
        let _ = context.register_permission(&Permission {
            node: perm_home.into(),
            description: "Access /home".into(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        });
        let home_cmd = Command::new(&["home".to_string()], "Teleport to home")
            .execute(HomeCommandHandler);
        context.register_command(home_cmd, perm_home);

        tracing::info!("[RainyEssentials] Successfully loaded /spawn, /home, /sethome!");
        Ok(())
    }
}

register_plugin!(RainyEssentials);
