use serenity::{model::channel::Message, prelude::*, utils::MessageBuilder};

const COMMAND_LIST: &str = "
!oshi: get a list of available commands
!oshi version: get the current oshi-chan release
";

pub async fn exec(ctx: &Context, msg: &Message) {
    let content = MessageBuilder::new()
        .push_line("Hello there, Human! My name is Oshi-Chan though you will address me as Oshi-Sama or face deez nuts. Here are a list of commands you can use to summon me.")
        .push_codeblock(COMMAND_LIST, Some("bash"))
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("introduce: error sending message: {:?}", why);
    }
}
