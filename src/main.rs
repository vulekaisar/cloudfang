use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use dotenvy::dotenv;
use std::error::Error;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load environment variables
    dotenv().ok(); // start searching for .env file

    // Check if API key is set
    if std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("Warning: OPENAI_API_KEY not found in environment. Please set it in .env file.");
    }

    // 2. Initialize Client
    let client = Client::new();

    println!("🤖 Rust AI Agent initialized (Model: gpt-3.5-turbo)");
    println!("Type 'quit' or 'exit' to stop.");
    println!("--------------------------------------------------");

    let mut conversation_history: Vec<ChatCompletionRequestMessage> = Vec::new();

    // Add System Message
    conversation_history.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content("You are a helpful and concise AI assistant written in Rust. You answer questions clearly.")
            .build()?
            .into()
    );

    // 3. Chat Loop
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("quit") || user_input.eq_ignore_ascii_case("exit") {
            break;
        }

        if user_input.is_empty() {
            continue;
        }

        // Add user message to history
        conversation_history.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_input)
                .build()?
                .into(),
        );

        // Create request
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .messages(conversation_history.clone())
            .build()?;

        // Call API
        println!("Thinking...");
        match client.chat().create(request).await {
            Ok(response) => {
                for choice in response.choices {
                    if let Some(content) = choice.message.content {
                        println!("🤖: {}", content);

                        // Add assistant response to history to maintain context
                        // Note: For a real rigorous implementation, you'd map the response back to a Message enum
                        // For this simple example, we just print it. To maintain context, we would append it.
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        println!("--------------------------------------------------");
    }

    Ok(())
}
