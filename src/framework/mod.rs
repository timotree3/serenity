//! The framework is a customizable method of separating commands.
//!
//! This is used in combination with [`Client::with_framework`].
//!
//! The framework has a number of configurations, and can have any number of
//! commands bound to it. The primary purpose of it is to offer the utility of
//! not needing to manually match message content strings to determine if a
//! message is a command.
//!
//! Additionally, "checks" can be added to commands, to ensure that a certain
//! condition is met prior to calling a command; this could be a check that the
//! user who posted a message owns the bot, for example.
//!
//! Each command has a given named, and an associated function/closure. For
//! example, you might have two commands: `"ping"` and `"weather"`. These each
//! have an associated function that are called if the framework determines
//! that a message is of that command.
//!
//! Assuming a command prefix of `"~"`, then the following would occur with the
//! two previous commands:
//!
//! ```ignore
//! ~ping // calls the ping command's function
//! ~pin // does not
//! ~ ping // _does_ call it _if_ the `allow_whitespace` option is enabled
//! ~~ping // does not
//! ```
//!
//! # Examples
//!
//! Configuring a Client with a framework, which has a prefix of `"~"` and a
//! ping and about command:
//!
//! ```rust,ignore
//! use serenity::client::{Client, Context};
//! use serenity::model::Message;
//! use std::env;
//!
//! let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap());
//!
//! client.with_framework(|f| f
//!     .configure(|c| c.prefix("~"))
//!     .command("about", |c| c.exec_str("A simple test bot"))
//!     .command("ping", |c| c.exec(ping)));
//!
//! command!(about(_context, message) {
//!     let _ = message.channel_id.say("A simple test bot");
//! });
//!
//! command!(ping(_context, message) {
//!     let _ = message.channel_id.say("Pong!");
//! });
//! ```
//!
//! [`Client::with_framework`]: ../client/struct.Client.html#method.with_framework

#[cfg(feature = "standard_framework")]
pub mod standard;

#[cfg(feature = "standard_framework")]
pub use self::standard::StandardFramework;

use client::Context;
use model::Message;

#[cfg(feature = "standard_framework")]
use model::UserId;

/// This trait allows for serenity to either use its builtin framework, or yours.
pub trait Framework {
    fn dispatch(&mut self, Context, Message);

    #[doc(hidden)]
    #[cfg(feature = "standard_framework")]
    fn update_current_user(&mut self, UserId, bool) {}
}

impl<F: Framework + ?Sized> Framework for Box<F> {
    fn dispatch(&mut self, ctx: Context, msg: Message) {
        (**self).dispatch(ctx, msg);
    }

    #[cfg(feature = "standard_framework")]
    fn update_current_user(&mut self, id: UserId, is_bot: bool) {
        (**self).update_current_user(id, is_bot);
    }
}

impl<'a, F: Framework + ?Sized> Framework for &'a mut F {
    fn dispatch(&mut self, ctx: Context, msg: Message) {
        (**self).dispatch(ctx, msg);
    }

    #[cfg(feature = "standard_framework")]
    fn update_current_user(&mut self, id: UserId, is_bot: bool) {
        (**self).update_current_user(id, is_bot);
    }
}

