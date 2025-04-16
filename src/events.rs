#![allow(dead_code, unused_variables)]
// remove later
use std::fmt;
use std::sync::LazyLock;
use twilight_gateway::{Event, EventTypeFlags, Shard, StreamExt};
use twilight_model::channel::Channel;
use twilight_model::gateway::payload::incoming;
use twilight_model::guild::{Guild, Member, PartialGuild};
use twilight_model::id::{Id, marker::*};

/// This constant represents most events
/// that the client will use
/// it includes most guild, direct message, moderation and voice events
/// however excludes stuff like auto-mod
/// it may be shortened or expanded.
static WANTED_EVENTS_BY_CLIENT: LazyLock<EventTypeFlags> = LazyLock::new(|| {
    EventTypeFlags::GUILD_PRESENCES
        | EventTypeFlags::GUILD_MEMBERS
        | EventTypeFlags::GUILD_MESSAGE_REACTIONS
        | EventTypeFlags::GUILD_MESSAGE_TYPING
        | EventTypeFlags::GUILDS
        | EventTypeFlags::GUILD_VOICE_STATES
        | EventTypeFlags::GUILD_INVITES
        | EventTypeFlags::GUILD_MESSAGES
        | EventTypeFlags::DIRECT_MESSAGES
        | EventTypeFlags::DIRECT_MESSAGE_REACTIONS
        | EventTypeFlags::DIRECT_MESSAGE_TYPING
        | EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::MESSAGE_DELETE
    // not sure: `MESSAGE_DELETE` reports bulk deletion
    // EventTypeFlags::MESSAGE_DELETE_BULK
});

#[derive(Debug)]
enum Operation {
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
struct EventProcessError(String);

impl fmt::Display for EventProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventProcessError(cause: {})", self.0)
    }
}

impl std::error::Error for EventProcessError {}

type Result<T> = std::result::Result<T, EventProcessError>;

/// Returns () implicitly
/// might return a value in the future
fn take_action_on_event(event: Event) -> () {
    use incoming::GuildCreate::*;
    use twilight_model::gateway::event::Event::*;

    match event {
        // intent: GUILDS
        GuildCreate(gc) => match *gc {
            Unavailable(guild) => {
                // unavailable guild
                // if anything
                // these guilds are often guilds discord couldn't read when opening the gateway
                // should be resolved via an request
                tracing::info!("unavailable guild, id: {}", guild.id.get())
            }
            Available(guild) => process_available_guild(guild),
        },
        GuildDelete(gd) => {
            // If the value is `None`
            // the user was forcibly removed.
            let banned_or_kicked = gd.unavailable.is_none();
            let is_guild_unavailable = gd.unavailable.is_some_and(|val| val);

            process_delete_guild(banned_or_kicked, is_guild_unavailable, gd.id);
        }
        GuildUpdate(gu) => {
            process_update_guild(gu.0);
        }
        ChannelCreate(cc) => channel_op(Operation::Create, cc.0).expect("no reason to fail"),
        ChannelUpdate(cu) => {
            let id = cu.0.id;

            channel_op(Operation::Update, cu.0).expect("no reason to fail");
        }
        ChannelDelete(cd) => channel_op(Operation::Delete, cd.0).expect("no reason to fail"),
        ChannelPinsUpdate(pins) => {
            // here no function would be needed to be fair
            // i don't know yet how to use it
            todo!()
        }
        ThreadCreate(tc) => {
            channel_op(Operation::Create, tc.0).expect("no reason to fail");
            todo!();
        }
        ThreadUpdate(tu) => {
            channel_op(Operation::Update, tu.0).expect("no reason to fail");
            todo!()
        }
        ThreadDelete(td) => {
            // treat this specially and just find the channel (Thread) by the marker
            // and remove it
            // Why?
            // this event is inconsistent and doesn't give a channel object
            todo!()
        }

        ThreadMemberUpdate(tmu) => {
            let member_id = tmu.member.member.expect("member should be present").user.id;

            member_fn(
                |member|
                // update our thread member
                // 
            {},
                member_id,
            )
        }
        ThreadMembersUpdate(tmus) => {
            channel_fn(
                |member| {
                    // register members in the thread!
                },
                tmus.id,
            );
        }
        StageInstanceCreate(sic) => {
            todo!()
        }
        StageInstanceDelete(sid) => {
            todo!()
        }
        StageInstanceUpdate(siu) => {
            todo!()
        }

        _ => unimplemented!(),
    }
}

fn process_available_guild(guild: Guild) {
    todo!()
}

fn process_delete_guild(banned_or_kicked: bool, is_now_unavailable: bool, id: Id<GuildMarker>) {
    // i assume some logic to remove the guild from the gui and etc etc
    todo!()
}

fn process_update_guild(guild: PartialGuild) {
    todo!()
}

fn channel_op(op: Operation, chan: Channel) -> Result<()> {
    match op {
        Operation::Create => {
            // Something to register a channel
        }

        Operation::Update => {
            // fetch and update channel
        }

        Operation::Delete => {
            // delete channel
        }
    }

    Ok(())
}

fn channel_fn<F>(func: F, chan: Id<ChannelMarker>)
where
    F: FnOnce(&mut Channel),
{
    let mut channel = todo!(); // some storage of channels?

    func(&mut channel);
}

fn member_fn<F>(func: F, member_id: Id<UserMarker>)
where
    F: FnOnce(&mut Member),
{
    let mut member = todo!(); // global discord client state? using the state struct

    func(&mut member)
}

async fn run_shard(mut shard: Shard) {
    let intents = WANTED_EVENTS_BY_CLIENT.clone();
    while let Some(event) = shard.next_event(intents).await {
        if event.is_err() {
            tracing::warn!("error while processing a event");
            continue;
        }

        let ev = event.unwrap_or_else(|_| unreachable!());
        take_action_on_event(ev);
    }
}
