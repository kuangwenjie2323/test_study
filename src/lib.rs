use flowsnet_platform_sdk::write_error_log;
use github_flows::{get_octo, listen_to_event, octocrab::models::events::payload::EventPayload};
use openai_flows::chat_completion;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "test_study",//github Repository仓库名
        "chat-with-chatgpt-test",//github organization组织名
        vec!["issue_comment", "issues"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octo = get_octo(Some(String::from("test_study")));
    let issues = octo.issues("test_study", "chat-with-chatgpt-test");

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion("Karewink", &format!("issue#{}", e.issue.number), &b)
                    {
                        if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                            write_error_log!(e.to_string());
                        }
                    }
                }
            }
        }

        // EventPayload::IssuesEvent(e) => {
        //     let title = e.issue.title;
        //     let body = e.issue.body.unwrap_or("".to_string());
        //     let q = title + "\n" + &body;
        //     if let Some(r) = chat_completion("Karewink", &format!("issue#{}", e.issue.number), &q) {
        //         if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
        //             write_error_log!(e.to_string());
        //         }
        //     }
        // }

        // _ => (),
    };
}
