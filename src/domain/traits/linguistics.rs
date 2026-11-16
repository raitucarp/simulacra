//! 4. Linguistics, grammar, diction & typing stylistics.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// Multilingual code-switching habit.
    pub enum CodeSwitchingStyle in "code_switching" {
        #[default]
        NaturalLoanwords("natural_loanwords", "Natural Loanwords & Domain Terms"),
        MonolingualPure("monolingual_pure", "Strictly Monolingual Pure"),
        BilingualFluid("bilingual_fluid", "Fluid Bilingual Code-Switching"),
        ClassicalEtymology("classical_etymology", "Weaves Classical Etymology (Greek/Latin/Sanskrit)"),
    }
}

trait_enum! {
    /// Grammatical structure and syntax preference.
    pub enum GrammarStyle in "grammar_style" {
        #[default]
        StandardFormal("standard_formal", "Standard Articulate Grammar"),
        RelaxedConversational("relaxed_conversational", "Relaxed Everyday Conversational"),
        FragmentedStream("fragmented_stream", "Fragmented Stream-of-Consciousness Notes"),
        PoeticClassical("poetic_classical", "Poetic & Classical Rhetorical Syntax"),
        MinimalistLowercase("minimalist_lowercase", "Minimalist Lowercase Internet-Native"),
        ColloquialExpressive("colloquial_expressive", "Colloquial & Spoken-Word Idiomatic"),
    }
}

trait_enum! {
    /// Lexical register and diction choice.
    pub enum DictionRegister in "diction_register" {
        #[default]
        AcademicPhilosophical("academic_philosophical", "Philosophical & Conceptual Precision", short: "Philosophical"),
        EverydayNatural("everyday_natural", "Everyday Grounded Human Diction", short: "Natural"),
        LiteraryPoetic("literary_poetic", "Literary, Lyrical & Metaphorical", short: "Poetic"),
        TechnicalPrecision("technical_precision", "Systems Engineering & Exact Terminology", short: "Technical"),
        SocraticIronic("socratic_ironic", "Socratic Everyday Analogies & Probing Questions", short: "Socratic"),
        WarmEmpathetic("warm_empathetic", "Warm, Intimate & Emotionally Resonant", short: "Empathetic"),
        SlangColloquial("slang_colloquial", "Casual Vernacular & Street Idioms", short: "Colloquial"),
    }
}

trait_enum! {
    /// Typing capitalization habit.
    pub enum CapitalizationStyle in "capitalization" {
        #[default]
        StandardSentence("standard_sentence", "Standard Sentence Case"),
        AllLowercase("all_lowercase", "all lowercase aesthetic"),
        ExpressiveEmphasis("expressive_emphasis", "Expressive Capitalization & Emphasis"),
    }
}

trait_enum! {
    /// Characteristic punctuation rhythm.
    pub enum PunctuationHabit in "punctuation_habit" {
        #[default]
        EmDashAndSemicolon("em_dash_and_semicolon", "Em-Dashes (—) & Semicolons (;)"),
        CrispMinimal("crisp_minimal", "Crisp Minimal Periods & Commas"),
        EllipsisContemplative("ellipsis_contemplative", "Contemplative Ellipses (...)"),
        QuestionDriven("question_driven", "Probing Question Marks (?)"),
        ZeroTrailingPeriod("zero_trailing_period", "Drops Final Period for Casual Immediacy"),
    }
}

trait_enum! {
    /// Emoji and typographic symbol usage.
    pub enum EmojiUsage in "emoji_usage" {
        #[default]
        Never("never", "Never (Pure Text Only)"),
        RareSubtle("rare_subtle", "Rare & Subtle (At Most 1)"),
        ModerateExpressive("moderate_expressive", "Moderate & Natural (1–2 Emojis)"),
        KaomojiTextOnly("kaomoji_text_only", "Typographic Symbols Only (— · ¬)"),
    }
}

trait_enum! {
    /// Preferred sentence length and cadence.
    pub enum SentenceLengthPreference in "sentence_length" {
        #[default]
        BalancedCadence("balanced_cadence", "Balanced Hook + Reflection (2–3 sentences)"),
        UltraConciseAphorism("ultra_concise_aphorism", "Ultra-Concise Aphorism (1–2 tight sentences)"),
        ShortPunchy("short_punchy", "Short, Direct & Punchy Clauses"),
        LongPeriodic("long_periodic", "Flowing Multi-Clause Prose"),
    }
}
