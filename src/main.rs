use std::env;
use std::error::Error;
use std::io::{self, Read};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // let resp = reqwest::get("https://news.ycombinator.com").await?;
    // let body = resp.text().await?;

    let args: Vec<String> = env::args().collect();
    let instructions = args.join(" ");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let request = CreateChatCompletionRequestArgs::default()
        // .max_tokens(512u16)
        .model("gpt-3.5-turbo-16k")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(format!("
                  You are a unix command line tool that takes input into stdin. You
                  follow the instructions provided. Your results will be written to
                  stdout and will be processed by other unix command line tools.

                  Do not include any prefix to your response data, just the raw data
                  itself. Do not explain anything.

                  Unless the instructions specify otherwise, output the same type of
                  data that you input. For example, if the input is JSON then the
                  output should be JSON. If the input is CSV rows then the output
                  should be CSV rows.

                  If the output is a machine readable format, be sure to output
                  correct valid syntax including adding correct quotes or escapes.

                  Instructions from user: {}",
                  instructions
                ))
                .build()?,
            // ChatCompletionRequestMessageArgs::default()
            //     .role(Role::System)
            //     .content(format!("Instruction from user: {}", instructions))
            //     .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(input)
                .build()?,
            // ChatCompletionRequestMessageArgs::default()
            //     .role(Role::Assistant)
            //     .content("The Los Angeles Dodgers won the World Series in 2020.")
            //     .build()?,
            // ChatCompletionRequestMessageArgs::default()
            //     .role(Role::User)
            //     .content("Where was it played?")
            //     .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        // eprintln!(
        //     "{}: Role: {}  Content: {:?}",
        //     choice.index, choice.message.role, choice.message.content
        // );
        println!("{}", choice.message.content.unwrap());
    }

    Ok(())
}
