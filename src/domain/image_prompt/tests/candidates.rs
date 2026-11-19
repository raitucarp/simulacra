use super::super::*;

const A: &str = "A lone crow on a glass tower at dawn, soft violet light";
const B: &str = "An ink drawing of a crow made of glass, mist and quiet water";
const C: &str = "A paper collage of crows circling a lighthouse, warm palette";

#[test]
fn the_request_names_the_task_subject_and_the_assembled_draft() {
    let subject = ImageSubject { noun: "a crow".into(), details: vec!["made of glass".into()], palette_hint: None };
    let context = ImageContext { target: ImageTarget::Avatar, subject, theme_colours: Vec::new() };
    let prompt = candidates_prompt(&ImageChoices::for_target(ImageTarget::Avatar), &context);
    assert!(prompt.system.contains("JSON array"));
    assert!(prompt.user.contains("avatar prompts") && prompt.user.contains("Subject: a crow") && prompt.user.contains("made of glass"));
    assert!(prompt.user.contains("head-and-shoulders portrait"));
    let post = ImageContext { target: ImageTarget::Post, ..context };
    assert!(candidates_prompt(&ImageChoices::for_target(ImageTarget::Post), &post).user.contains("goes with this post"));
}

#[test]
fn a_json_array_is_read_even_inside_a_fence() {
    let answer = format!("Sure!\n```json\n[\"{A}\", \"{B}\", \"{C}\", \"one too many prompts here\"]\n```");
    assert_eq!(parse_candidates(&answer), vec![A.to_string(), B.into(), C.into()]);
    let objects = format!("[{{\"prompt\": \"{A}\"}}, {{\"prompt\": \"{B}\"}}]");
    assert_eq!(parse_candidates(&objects), vec![A.to_string(), B.into()]);
}

#[test]
fn lists_and_labels_are_cleaned() {
    let numbered = format!("Here are three prompts:\n1. \"{A}\"\n2) {B}\n3: {C},");
    assert_eq!(parse_candidates(&numbered), vec![A.to_string(), B.into(), C.into()]);
    let labelled = format!("**Prompt 1:** {A}\nPrompt 2 \u{2014} {B}\n- Option C: {C}\n\n\u{2022} {A}");
    assert_eq!(parse_candidates(&labelled), vec![A.to_string(), B.into(), C.into()], "labels stripped, duplicate dropped");
}

#[test]
fn noise_gives_nothing() {
    assert!(parse_candidates("").is_empty());
    assert!(parse_candidates("ok\n```\n[]\n```").is_empty());
    assert_eq!(parse_candidates(&format!("[1, 2, \"{A}\"]")), vec![A.to_string()]);
}
