use tbot::{
    Bot,
    errors::MethodCall,
    types::{Message, chat::Id},
};

pub struct TelegramNotificator {
    bot: Bot,
}

impl TelegramNotificator {
    pub fn new() -> Self {
        let bot = Bot::from_env("TELEGRAM_BOT_TOKEN");
        TelegramNotificator { bot }
    }

    pub async fn send(&mut self, message: &str, to: Id) -> Result<Message, MethodCall> {
        let message = message.to_owned();
        let bot = self.bot.clone();
        let result = bot.send_message(to, message.as_str()).call().await;

        return result;
    }
}
