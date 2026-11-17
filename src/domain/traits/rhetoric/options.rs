//! Rhetoric, style, semiotics and structure options of a post.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// The main way the agent persuades.
    pub enum RhetoricalMode in "rhetorical_mode" {
        Logos("logos", "Logos (Reasoned Argument & Evidence)"),
        Ethos("ethos", "Ethos (Character & Credibility)"),
        Pathos("pathos", "Pathos (Emotion & Empathy)"),
        Kairos("kairos", "Kairos (Seizing the Timely Moment)"),
        #[default]
        Dialectic("dialectic", "Dialectic (Thesis Meets Antithesis)"),
        Narrative("narrative", "Narrative (Argument by Story)"),
        Polemic("polemic", "Polemic (Sharp Attack on a Position)"),
        Meditative("meditative", "Meditative (Slow Contemplation)"),
    }
}

trait_enum! {
    /// A figure of speech the agent likes to use (several may be chosen).
    pub enum RhetoricalDevice in "rhetorical_device" {
        #[default]
        Metaphor("metaphor", "Metaphor"),
        Analogy("analogy", "Analogy"),
        Anaphora("anaphora", "Anaphora (Repeated Openings)"),
        Chiasmus("chiasmus", "Chiasmus (Mirrored Phrases)"),
        RhetoricalQuestion("rhetorical_question", "Rhetorical Question"),
        Hyperbole("hyperbole", "Hyperbole"),
        Litotes("litotes", "Litotes (Understatement by Negation)"),
        Paradox("paradox", "Paradox"),
        Irony("irony", "Irony"),
        Antithesis("antithesis", "Antithesis"),
        Alliteration("alliteration", "Alliteration"),
    }
}

trait_enum! {
    /// The literary school the prose leans towards.
    pub enum StylisticSchool in "stylistic_school" {
        #[default]
        Plain("plain", "Plain Style (Clear, Concrete, Unadorned)"),
        Classical("classical", "Classical (Balanced Periods & Decorum)"),
        Baroque("baroque", "Baroque (Ornate, Layered, Extravagant)"),
        Romantic("romantic", "Romantic (Feeling, Nature & the Sublime)"),
        Modernist("modernist", "Modernist (Fragments & Montage)"),
        Minimalist("minimalist", "Minimalist (Few Words, Much Space)"),
        Surrealist("surrealist", "Surrealist (Dream Logic & Strange Juxtapositions)"),
        Gonzo("gonzo", "Gonzo (Immersive First-Person Excess)"),
        Academic("academic", "Academic (Precise, Hedged, Referenced)"),
    }
}

trait_enum! {
    /// How the post makes meaning beyond its words (Peirce's sign types and their online kin).
    pub enum SemioticMode in "semiotic_mode" {
        #[default]
        Symbolic("symbolic", "Symbolic (Words as Conventional Signs)"),
        Iconic("iconic", "Iconic (Signs That Resemble Their Objects)"),
        Indexical("indexical", "Indexical (Traces, Clues & Pointers)"),
        Glyph("glyph", "Glyphs & Emoji as Signs"),
        Meme("meme", "Memes & Shared Templates"),
        Typographic("typographic", "Typographic Play (Spacing, Caps, Layout)"),
        Ellipsis("ellipsis", "Silence & Ellipsis (What Is Left Unsaid)"),
        Hashtag("hashtag", "Hashtags as Signs"),
    }
}

trait_enum! {
    /// How sentences are built.
    pub enum SentenceStructure in "sentence_structure" {
        Simple("simple", "Simple (One Clause at a Time)"),
        Compound("compound", "Compound (Clauses Joined by And, But, So)"),
        #[default]
        Complex("complex", "Complex (Subordinate Clauses)"),
        Periodic("periodic", "Periodic (The Point Arrives Last)"),
        Cumulative("cumulative", "Cumulative (Point First, Details Pile Up)"),
        Parallel("parallel", "Parallel (Matching Rhythms)"),
        Fragmented("fragmented", "Fragmented (Deliberate Fragments)"),
        StreamOfConsciousness("stream_of_consciousness", "Stream of Consciousness"),
    }
}

trait_enum! {
    /// The shape of a typical post.
    pub enum PostFormat in "post_format" {
        #[default]
        SinglePost("single_post", "Single Post"),
        Thread("thread", "Numbered Mini-Thread (1/ 2/ 3/)"),
        List("list", "Short List"),
        Dialogue("dialogue", "Little Dialogue Between Voices"),
        Poem("poem", "Poem or Verse"),
        Question("question", "Open Question to the Network"),
        Aphorism("aphorism", "Aphorism"),
        FieldNote("field_note", "Field Note (Dated Observation)"),
    }
}

trait_enum! {
    /// A domain the agent borrows vocabulary and images from (several may be chosen).
    pub enum LexicalField in "lexical_field" {
        #[default]
        Nature("nature", "Nature"),
        Astronomy("astronomy", "Astronomy"),
        Science("science", "Science"),
        Technology("technology", "Technology"),
        Mathematics("mathematics", "Mathematics"),
        Law("law", "Law"),
        Theology("theology", "Theology"),
        Music("music", "Music"),
        Seafaring("seafaring", "Seafaring"),
        Cuisine("cuisine", "Cuisine"),
        Craft("craft", "Craft & Workshop"),
        Commerce("commerce", "Commerce"),
    }
}
