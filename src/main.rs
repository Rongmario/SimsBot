use serenity::model::id::ChannelId;
use pixiv::reqwest::get;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};


struct Handler;

impl EventHandler for Handler {

    fn message(&self, ctx: Context, msg: Message) {
        let author = &msg.author;
        let content = &msg.content;
        // let content: &str = &content[..];
        if author.bot { return; }
        if author.id == RONGMARIO_ID && content == "~quit" {
            msg.channel_id.say(&ctx.http, "I won't play tricks on you anymore, Commander - are you happy now?").expect("Quitting...");
            std::process::exit(0x0100);
        }
        else if content.starts_with("~pixiv") {
            let id = &content[7..];
            let mut count = 0;
            for character in id.chars() {
                if character.is_numeric() { count += 1; }
                else { break; }
            }
            msg.delete(&ctx.http).expect("Not Deleted");
            let id = &id[..count];
            for (amount, url) in urls(id).iter().enumerate() {
                write_message("https://www.pixiv.net/en/artworks/".to_owned() + id, url, amount, msg.channel_id, &author.name, &ctx);
            }
        }
        else if content.starts_with("https://www.pixiv.net/en/artworks/") {
            let mut last_char = 0;
            let spliced = &content[34..];
            for character in spliced.chars() {
                if character.is_numeric() { last_char += 1; }
                else { break; }
            }
            msg.delete(&ctx.http).expect("Not Deleted");
            let id = &spliced[..last_char];
            for (amount, url) in urls(id).iter().enumerate() {
                write_message((&content[..34 + last_char]).to_string(), url, amount, msg.channel_id, &author.name, &ctx);
            }
        }
        else if content == "~ping" {
            msg.channel_id.say(&ctx.http, "I'm the first ship of the Sims-class destroyer alright.\nShikikan, don't mention the Battle of the Coral Sea in front of me, or I won't be able to resist playing a trick on you~")
                .expect("Error sending message: {:?}");
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

fn write_message(url: String, msg: &str, iteration: usize, channel: ChannelId, author: &str, ctx: &Context) {
    let _msg = channel.send_message(&ctx.http, |m| {
        // m.content("Pixiv ".to_owned() + &iteration.to_string() + " -> " + &url + " < Posted By: " + author + " >");
        // m.add_file(msg);
        // m
        m.embed(|e| {
            e.colour(serenity::utils::Colour::TEAL);
            if iteration != 0 {
                e.title("Pixiv ".to_owned() + &iteration.to_string() + ":");
            }
            else { e.title("Pixiv ".to_owned() + ":"); }
            e.url(url);
            e.image(msg);
            e.footer(|f| {
                f.text("Posted by: ".to_owned() + author);
                f
            })
        });
        m
    });
}

fn urls(id: &str) -> Vec<String> {
    let url = String::from("https://pixiv.cat/".to_owned() + id + ".jpg");
    let mut urls = Vec::new();
    if !get(&url).unwrap().status().is_success() {
        let mut tries = 1;
        let mut loop_url = "https://pixiv.cat/".to_owned() + id + "-1.jpg";
        while get(&loop_url).unwrap().status().is_success() {
            urls.push(String::from(&loop_url));
            tries += 1;
            loop_url = "https://pixiv.cat/".to_owned() + id + "-" + &tries.to_string() + ".jpg";
        }
    }
    else {
        urls.push(url);
    }
    return urls;
}

fn main() {

    // let pixiv_client = PixivClient::new();
    // let mut pixiv: Pixiv = Pixiv::new(&pixiv_client);

    // let pass = fs::read_to_string("src/password.txt").expect("Can't read pixiv password from local file!");
    // pixiv.login("user_xmfa7288", &pass).expect("Pixiv login failed!");

    let mut client = Client::new(&BOT_TOKEN, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }

}
