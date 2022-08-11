use crate::error::CadencyError;
use crate::utils;
use serenity::{
    async_trait,
    client::Context,
    model::application::{
        command::Command,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
};

pub mod fib;
pub mod inspire;
#[cfg(feature = "audio")]
pub mod now;
#[cfg(feature = "audio")]
pub mod pause;
pub mod ping;
#[cfg(feature = "audio")]
pub mod play;
#[cfg(feature = "audio")]
pub mod resume;
#[cfg(feature = "audio")]
pub mod skip;
pub mod slap;
#[cfg(feature = "audio")]
pub mod stop;
#[cfg(feature = "audio")]
pub mod tracks;
pub mod urban;

pub use fib::Fib;
pub use inspire::Inspire;
#[cfg(feature = "audio")]
pub use now::Now;
#[cfg(feature = "audio")]
pub use pause::Pause;
pub use ping::Ping;
#[cfg(feature = "audio")]
pub use play::Play;
#[cfg(feature = "audio")]
pub use resume::Resume;
#[cfg(feature = "audio")]
pub use skip::Skip;
pub use slap::Slap;
#[cfg(feature = "audio")]
pub use stop::Stop;
#[cfg(feature = "audio")]
pub use tracks::Tracks;
pub use urban::Urban;

pub trait CadencyCommandConfig {
    fn use_deferred() -> bool {
        false
    }
}

#[async_trait]
pub trait CadencyCommandCatcher {
    async fn catch<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
        err: CadencyError
    ) -> Result<(), CadencyError> {
        return match err {
            CadencyError::Response => utils::create_response(ctx, command, ":x: **Something wen wrong**").await,
            CadencyError::Join => utils::create_response(ctx, command, "").await

        }
    }
}


#[async_trait]
pub trait CadencyCommand {    
    async fn register(ctx: &Context) -> Result<Command, serenity::Error>;
    async fn cmd<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError> {
        if Self::use_deferred() {
            utils::voice::create_deferred_response(ctx, command).await?;
        };
        match Self::execute(ctx, command).await {
            Ok(_) => Ok(()),
            Err(err) => {
                return Self::catch(ctx, command, err).await
            }
        }
    }
    async fn execute<'a>(
        ctx: &Context,
        command: &'a mut ApplicationCommandInteraction,
    ) -> Result<(), CadencyError>;    
}

/// Submit global slash commands to the discord api.
/// As global commands are cached for 1 hour, the activation ca take some time.
/// For local testing it is recommended to create commands with a guild scope.
pub async fn setup_commands(ctx: &Context) -> Result<(), serenity::Error> {
    tokio::try_join!(
        Ping::register(ctx),
        Inspire::register(ctx),
        Fib::register(ctx),
        Urban::register(ctx),
        Slap::register(ctx)
    )?;
    #[cfg(feature = "audio")]
    tokio::try_join!(
        Play::register(ctx),
        Now::register(ctx),
        Skip::register(ctx),
        Pause::register(ctx),
        Resume::register(ctx),
        Stop::register(ctx),
        Tracks::register(ctx)
    )?;
    Ok(())
}

pub async fn command_not_implemented(
    ctx: &Context,
    command: ApplicationCommandInteraction,
) -> Result<(), CadencyError> {
    error!("The following command is not known: {:?}", command);
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Unknown command"))
        })
        .await
        .map_err(|err| {
            error!("Interaction response failed: {}", err);
            CadencyError::Response
        })
}
