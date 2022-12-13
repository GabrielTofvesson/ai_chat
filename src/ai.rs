use std::time::SystemTime;

use openai_api::Client;

use crate::chat::Chat;

struct ContextManager {
    context: Option<String>,
    context_refresh_timer: u32,
    refresh_interval: u32,
}

impl ContextManager {
    pub fn new(refresh_interval: u32) -> ContextManager {
        ContextManager { context: Option::None, context_refresh_timer: 0, refresh_interval }
    }

    fn to_context_string(context: &String) -> String {
        format!("Context: {context}")
    }

    fn should_refresh_context(&self) -> bool {
        self.context.is_none() || self.context_refresh_timer >= self.refresh_interval
    }

    fn get_contextfree_count(&self) -> u32 {
        self.context_refresh_timer
    }

    fn refresh_context(&mut self, chat: &Chat, ai_client: &Client) {
        self.context.replace(ai_client.complete_prompt_sync(openai_api::api::CompletionArgs::builder()
                .prompt(format!("Explain the conversation concisely:\n{}Explanation:", self.get_chat_context(chat)))
                .engine("text-davinci-003")
                .max_tokens(512)
                .temperature(0.23)
                .top_p(0.9)
                .frequency_penalty(0.3)
                .presence_penalty(0.12)
                .build()
                .unwrap())
            .unwrap()
            .choices[0]
            .text
            .trim()
            .to_string());

            self.context_refresh_timer = 0;
    }

    pub fn get_context(&mut self, chat: &Chat, ai_client: &Client) -> &String {
        if self.should_refresh_context() {
            self.refresh_context(chat, ai_client);
        }

        return self.context.as_ref().unwrap()
    }

    pub fn on_message(&mut self) {
        self.context_refresh_timer += 1
    }

    pub fn get_chat_context(&self, chat: &Chat) -> String {
        let mut chat_context = String::new();
        if let Option::Some(context) = self.context.as_ref() {
            chat_context.push_str(&ContextManager::to_context_string(context));
            chat_context.push_str("\n");
        }

        let history = chat.get_history();
        let mut idx = 0;
        for chat_entry in &history[std::cmp::max(0, history.len() as i32 - self.context_refresh_timer as i32) as usize..] {
            chat_context.push_str(&format!("{idx}. {}: {}\n", chat_entry.sender, chat_entry.message));
            idx += 1;
        }

        chat_context
    }
}

pub struct Ai {
    client: Client,
    chat: Chat,
    context_manager: ContextManager
}

impl Ai {
    pub fn new(chat: Chat, token: String, context_refresh_interval: u32) -> Ai {
        let client = Client::new(token.as_str());
        Ai { client, chat, context_manager: ContextManager::new(context_refresh_interval) }
    }

    fn get_prompt(&self) -> String {
        format!("{}{}. AI:", self.context_manager.get_chat_context(&self.chat), self.context_manager.get_contextfree_count())
    }

    pub fn send_message(&mut self, message: String) -> String {
        self.chat.add_message("User".to_owned(), message, SystemTime::now());
        self.context_manager.on_message();

        let prompt = self.get_prompt();

        let response = self.client.complete_prompt_sync(openai_api::api::CompletionArgs::builder()
                .prompt(prompt)
                .engine("text-davinci-003")
                .max_tokens(1536)
                .temperature(0.35)
                .top_p(0.95)
                .frequency_penalty(0.5)
                .presence_penalty(0.33)
                .build()
                .unwrap())
            .unwrap()
            .choices[0]
            .text
            .trim()
            .to_string();
        
        self.context_manager.on_message();
        self.chat.add_message("AI".to_owned(), response.to_owned(), SystemTime::now());

        response
    }
}