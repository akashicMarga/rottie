use crate::config;
use db::rocks_db::get_related_chunks;
use rottie_inference::embeddings::embedding_model;
use rottie_inference::phi3;
use serde_yaml::Error;
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;

pub struct Orchestrator {
    session_id: uuid::Uuid,
    conversation_tracker: Vec<HashMap<String, String>>,
    lm_model: rottie_inference::phi3::phi3,
    emb_model: rottie_inference::embeddings::embedding_model,
    config: Result<config::Config, Error>,
}

impl Orchestrator {
    pub fn new(session_id: uuid::Uuid, config: Result<config::Config, Error>) -> Self {
        let mut base_llm = rottie_inference::phi3::phi3::init();
        let mut conversation_tracker: Vec<HashMap<String, String>> = Vec::new();
        let mut embedding_model = rottie_inference::embeddings::embedding_model::load_model();

        Self {
            session_id,
            conversation_tracker: conversation_tracker,
            lm_model: base_llm,
            emb_model: embedding_model,
            config: config,
        }
    }

    pub async fn run(&mut self) {
        let initial_system_prompt = match &self.config {
            Ok(ref config) => config.orchestrator.llm_info.chat_template.clone(),
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                return; // or handle the error appropriately
            }
        };

        let mut input_text = String::new();
        let mut history = String::new();
        history.push_str("Based on prompt, query , Rottie will will talk like a human and will reason user chat and just reply with it's own reply. you are Rottie here an agent which is following the prompt and history and current user query to answer the next response. be precise and only give response and nothing else.");

        history.push_str(&format!("this is userer prompt and your personality you have to act based on this prompt and do your reasoning {}", initial_system_prompt));

        use std::io::{self};
        while input_text.to_lowercase().trim() != "quit" {
            print!("User: ");
            io::stdout().flush().unwrap();
            input_text.clear();
            io::stdin()
                .read_line(&mut input_text)
                .expect("Failed to read line");

            if input_text.to_lowercase().trim() == "quit" {
                break;
            }

            history.push_str(&format!(
                "This is the user query in this turn that you have to answer {}",
                input_text
            ));

            // println!("history: {}", history);
            let embeddings: Vec<f32> = self
                .emb_model
                .get_embeddings(&input_text)
                .unwrap()
                .reshape((384,))
                .unwrap()
                .to_vec1()
                .unwrap();
            let k = get_related_chunks(embeddings).await.unwrap();
            let mut context = vec![];
            for reference in k.iter() {
                let releted = reference.get_adjacent_chunks(1, 1).await.unwrap();
                context.extend(releted);
            }

            println!("context: {:?}", context);

            match self.lm_model.generate(history.clone()) {
                Ok(response) => {
                    println!("Rottie: {:?}", response);
                    let mut turn: HashMap<String, String> = HashMap::new();
                    turn.insert("User".to_string(), input_text.clone());
                    turn.insert("Rottie".to_string(), response.clone());
                    turn.insert("history".to_string(), history.clone());
                    self.conversation_tracker.push(turn);
                    history.push_str(&format!("rottie response in previous turn {}", response));
                }
                Err(e) => eprintln!("Error generating text: {}", e),
            }
            // println!("Conversation tracker: {:?}", conversation_tracker);
        }
    }
}
