//! Writing and import choices: the identity posts go out under, what clicking a wikilink or
//! a hashtag does, and how imported text is cut into posts.

use super::super::choice::choice_enum;

choice_enum! {
    /// Who a new post (or an imported passage) is published as.
    PostIdentity, default: Yourself,
    [Yourself => "yourself", Pseudonym => "pseudonym", Anonymous => "anonymous"]
}

choice_enum! {
    /// What clicking a `[[wikilink]]` pill does.
    WikilinkClick, default: Preview,
    [Preview => "preview", Inspector => "inspector"]
}

choice_enum! {
    /// What clicking a `#hashtag` does.
    HashtagClick, default: SearchFeed,
    [SearchFeed => "search_feed", PlainText => "plain_text"]
}

choice_enum! {
    /// The unit an imported document is split into.
    ImportSegmentation, default: Paragraph,
    [Paragraph => "paragraph", Sentence => "sentence", Chapter => "chapter"]
}
