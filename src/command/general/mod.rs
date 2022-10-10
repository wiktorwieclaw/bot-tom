pub mod play;

use play::PLAY_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[commands(play)]
struct General;
