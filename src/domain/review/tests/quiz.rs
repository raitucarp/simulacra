use super::{basic, no_posts};
use crate::domain::review::item::ReviewItem;
use crate::domain::review::quiz::*;
use crate::domain::review::{Confidence, Grade};
use crate::domain::{NetworkId, Post, PostId, ProfileId};

fn tagged(id: &str, back: &str, tag: &str) -> ReviewItem {
    ReviewItem { tags: vec![tag.into()], ..basic(id, "q", back) }
}

#[test]
fn distractors_prefer_shared_tags_then_meaning_and_skip_duplicates() {
    let card = tagged("q", "Spinoza", "philosophy");
    let pool = vec![
        card.clone(),
        tagged("a", "Leibniz", "philosophy"),
        tagged("b", "spinoza", "philosophy"),
        tagged("c", "Photosynthesis", "biology"),
        tagged("d", "Mitochondria", "biology"),
        tagged("e", "Descartes", "history"),
        tagged("f", "Leibniz!", "philosophy"),
    ];
    let near_descartes = |other: &ReviewItem| if other.id.as_str() == "e" { 1.0 } else { 0.0 };
    let answer_of = |item: &ReviewItem| item.answer_text(&no_posts);
    let picked = pick_distractors(&card, &pool, &ChoiceSources { answer_of: &answer_of, affinity: &near_descartes });
    assert_eq!(picked.len(), DISTRACTORS);
    assert_eq!(picked[0], "Leibniz", "a shared tag ranks first");
    assert_eq!(picked[1], "Descartes", "then the closest in meaning");
    assert!(!picked.iter().any(|p| normalize_answer(p) == "spinoza"), "never the answer itself");
}

#[test]
fn a_question_shuffles_options_reproducibly_and_needs_a_distractor() {
    let card = basic("q", "Who wrote the Ethics?", "Spinoza");
    let pool = vec![card.clone(), basic("a", "q", "Hume"), basic("b", "q", "Kant"), basic("c", "q", "Locke")];
    let (none, answer_of) = (|_: &ReviewItem| 0.0, |item: &ReviewItem| item.answer_text(&no_posts));
    let none = ChoiceSources { answer_of: &answer_of, affinity: &none };
    let one = build_choice(&card, &pool, &none, 7).unwrap();
    assert_eq!(one, build_choice(&card, &pool, &none, 7).unwrap());
    assert_eq!(one.options[one.correct], "Spinoza");
    assert_eq!(one.options.len(), 4);
    assert!(build_choice(&card, std::slice::from_ref(&card), &none, 7).is_none());
}

#[test]
fn typed_answers_forgive_case_accents_punctuation_and_typos() {
    assert_eq!(judge_recall("Émile Durkheim", "emile durkheim!"), RecallVerdict::Exact);
    assert_eq!(judge_recall("mitochondria", "mitocondria"), RecallVerdict::Close);
    assert_eq!(judge_recall("mitochondria", "ribosome"), RecallVerdict::Wrong);
    assert_eq!(judge_recall("anything", "   "), RecallVerdict::Wrong);
    let post = "word ".repeat(CHECKABLE_ANSWER_CHARS);
    assert_eq!(judge_recall(&post, "the gist in my words"), RecallVerdict::Compare, "a whole post is compared by the learner");
    assert_eq!(RecallVerdict::Compare.is_correct(), None);
    assert_eq!(RecallVerdict::Close.suggested_grade(), Grade::Hard);
    assert!(similarity("abc", "abd") > 0.6);
}

#[test]
fn confidence_and_correctness_decide_a_quiz_grade() {
    assert_eq!(Confidence::grade_for(Some(Confidence::Sure), false), Grade::Again);
    assert_eq!(Confidence::grade_for(Some(Confidence::Guess), true), Grade::Hard);
    assert_eq!(Confidence::grade_for(None, true), Grade::Good);
}

#[test]
fn an_order_puzzle_starts_shuffled_and_scores_moves() {
    let pieces: Vec<String> = ["one", "two", "three", "four"].map(String::from).to_vec();
    let puzzle = OrderPuzzle::new("t".into(), pieces.clone(), 3).unwrap();
    assert_ne!(puzzle.arrangement, vec![0, 1, 2, 3], "never starts solved");
    let mut solved = puzzle;
    for target in 0..4 {
        while solved.arrangement[target] != target {
            let at = solved.arrangement.iter().position(|p| *p == target).unwrap();
            solved = solved.move_up(at);
        }
    }
    let solved = solved.check();
    assert!(solved.score().is_perfect());
    assert_eq!(solved.score().pairs_in_order, 3);
    assert_eq!(solved.clone().move_up(1), solved, "a checked puzzle is frozen");
    assert!(OrderPuzzle::new("t".into(), pieces[..2].to_vec(), 1).is_none());
}

fn post(id: &str, reply_to: Option<&str>, minute: u32) -> Post {
    let mut post = Post::create(PostId::new(id), NetworkId::new("n"), ProfileId::new("p"), &format!("post {id}")).unwrap();
    post.reply_to_id = reply_to.map(PostId::new);
    post.created_at = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2026, 1, 1, 0, minute, 0).unwrap();
    post
}

#[test]
fn a_thread_line_runs_from_the_root_through_the_earliest_replies() {
    let posts = vec![post("root", None, 0), post("a", Some("root"), 1), post("b", Some("a"), 2), post("late", Some("a"), 5), post("c", Some("b"), 3)];
    let line: Vec<&str> = thread_line(&posts, &PostId::new("a")).iter().map(|p| p.id.as_str()).collect();
    assert_eq!(line, vec!["root", "a", "b", "c"]);
    let sentences: Vec<String> = (0..10).map(|i| format!("s{i}")).collect();
    assert_eq!(sentence_window(&sentences, 2).len(), MAX_PIECES);
    assert!(sentence_window(&sentences[..2], 0).is_empty());
    assert_eq!(piece_text("a b c d", 3), "a b…");
}

#[test]
fn a_choice_run_grades_by_correctness_and_confidence() {
    let question = |id: &str| ChoiceQuestion { item_id: crate::domain::review::ItemId::new(id), options: vec!["a".into(), "b".into()], correct: 1 };
    let run = ChoiceRun::new(vec![question("one"), question("two")]).with_confidence(Confidence::Guess);
    let (run, answer) = run.pick(1);
    let answer = answer.unwrap();
    assert_eq!((answer.grade, answer.correct, answer.confidence), (Grade::Hard, Some(true), Some(Confidence::Guess)));
    assert!(run.clone().pick(0).1.is_none(), "only the first pick counts");
    let (run, wrong) = run.next().pick(0);
    assert_eq!(wrong.unwrap().grade, Grade::Again);
    let run = run.next();
    assert!(run.is_finished());
    assert_eq!(run.right, 1);
}

#[test]
fn a_recall_run_checks_then_takes_the_learners_grade() {
    let run = RecallRun::new(vec![crate::domain::review::ItemId::new("c")]);
    assert!(run.clone().grade(Grade::Good).1.is_none(), "grading waits for a check");
    let run = run.check("Spinosa", "Spinoza");
    assert_eq!(run.checked.as_ref().map(|(_, v)| *v), Some(RecallVerdict::Close));
    let (run, answer) = run.grade(Grade::Hard);
    assert_eq!(answer.unwrap().correct, Some(true));
    assert!(run.is_finished());
}

#[test]
fn prose_paragraphs_drop_markdown_residue() {
    let text = "# Part I: Of God\nBy substance I understand\nwhat is in itself.\n\n> The order of ideas.\n\n- A list item";
    assert_eq!(prose_paragraphs(text), vec!["By substance I understand what is in itself.", "The order of ideas.", "A list item"]);
}
