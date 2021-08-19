use teloxide::{Bot, payloads::AnswerInlineQuery, prelude::*, types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText}};
use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    // Create a new dispatcher to handle incoming messages
    let dp = Dispatcher::new(bot);
    dp.inline_queries_handler(|rx: DispatcherHandlerRx<Bot, InlineQuery>| {
        UnboundedReceiverStream::new(rx).for_each_concurrent(None, |msg| async move {
            // First, create your actual response
            let google_search = InlineQueryResultArticle::new(
                // Each item needs a unique ID, as well as the response container for the items.
                // These can be whatever, as long as they don't conflict.
                "01".to_string(),
                // What the user will actually see
                "Google Search",
                // What message will send when clicked/tapped
                InputMessageContent::Text(InputMessageContentText::new(format!(
                    "https://www.google.com/search?q={}",
                    msg.update.query,
                ))),
            );
            // You can also construct them from the struct itself, if you want a little more control.
            // Please refer to the documentation for more detailed information about each field.
            // https://docs.rs/teloxide/0.5.1/teloxide/types/struct.InlineQueryResultArticle.html
            let ddg_search = InlineQueryResultArticle {
                id: "02".to_string(), // again, anything -- as long as it's unique in this context
                title: "DuckDuckGo Search".to_string(),
                input_message_content: InputMessageContent::Text(InputMessageContentText::new(
                    format!("https://duckduckgo.com/?q={}", msg.update.query.to_string()),
                )),
                reply_markup: None,
                url: Some("https://duckduckgo.com/about".to_string()), // Note: This is the url that will open if they click the thumbnail
                hide_url: None,
                description: Some("DuckDuckGo Search".to_string()),
                thumb_url: Some(
                    "https://duckduckgo.com/assets/logo_header.v108.png".to_string(),
                ),
                thumb_width: Some(64),
                thumb_height: Some(64),
            };

            // Now put those responses into a "Result"
            // https://docs.rs/teloxide/0.5.1/teloxide/payloads/struct.AnswerInlineQuery.html
            let all_results = AnswerInlineQuery {
                inline_query_id: "03".to_string(), // again, anything -- as long as it's unique in this context
                results: vec![InlineQueryResult::Article(google_search), InlineQueryResult::Article(ddg_search)],
                cache_time: None,
                is_personal: None,
                next_offset: None,
                switch_pm_text: None,
                switch_pm_parameter: None,
            };

            // Send it off! One thing to note -- the ID we use here must be of the message we're responding to.
            let response = msg
                .requester
                .answer_inline_query(msg.update.id.to_string(), all_results.results)
                .send()
                .await;
            if response.is_err() {
                dbg!(response);
            }
        })
    })
    .dispatch()
    .await;
}
