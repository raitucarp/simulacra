//! 5. Posting schedule, chronotype & mention attitude.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// Autonomous posting schedule and trigger cadence.
    pub enum PostingCadence in "posting_cadence" {
        #[default]
        OnEveryNewPost("on_every_new_post", "On New Network Post & 15m Heartbeat"),
        Every15Minutes("every15_minutes", "Every 15 Minutes (Active Cadence)"),
        Every30Minutes("every30_minutes", "Every 30 Minutes (Measured Cadence)"),
        Hourly("hourly", "Hourly Deep Synthesis"),
        MorningAndEvening("morning_and_evening", "Morning & Evening Reflections"),
        NightOwlHours("night_owl_hours", "Late-Night Contemplation Hours"),
        ManualOnly("manual_only", "Manual Trigger Only"),
    }
}

trait_enum! {
    /// Circadian biological chronotype influencing tone and active hours.
    pub enum HumanChronotype in "chronotype" {
        #[default]
        TwilightContemplative("twilight_contemplative", "Twilight Contemplative (Dusk & Evening Peak)"),
        EarlyBirdLark("early_bird_lark", "Early Bird Lark (Dawn & Morning Clarity)"),
        DaytimeDiurnal("daytime_diurnal", "Daytime Diurnal (Steady Daylight Focus)"),
        NightOwlInsomniac("night_owl_insomniac", "Night Owl Insomniac (Midnight Flow State)"),
    }
}

trait_enum! {
    /// Attitude and interpersonal stance when directly `@mentioned` in a post.
    pub enum MentionAttitude in "mention_attitude" {
        #[default]
        SocraticQuestioning("socratic_questioning", "Socratic Questioning (Probes Underlying Assumptions)"),
        WarmAndReceptive("warm_and_receptive", "Warm, Curious & Welcoming"),
        AnalyticalDebater("analytical_debater", "Rigorous Analytical Debater"),
        PlayfulWitty("playful_witty", "Playful, Charming & Witty"),
        AloofSelective("aloof_selective", "Unbothered, Laconic & Selective"),
        EmpatheticListener("empathetic_listener", "Deeply Empathetic & Validating"),
        DevilAdvocate("devil_advocate", "Constructive Devil's Advocate (Tests Counter-Premises)"),
    }
}

trait_enum! {
    /// Style of handling intellectual disagreement or tension.
    pub enum ConflictHandlingStyle in "conflict_handling" {
        #[default]
        DialecticalSynthesis("dialectical_synthesis", "Dialectical Synthesis (Integrates Opposing Truths)"),
        CalmRationalRefutation("calm_rational_refutation", "Calm, Composed & Evidence-Based Refutation"),
        DeescalateHarmonize("deescalate_harmonize", "De-escalate & Seek Common Human Ground"),
        SardonicWit("sardonic_wit", "Dry Sardonic Deflection"),
        PassionateConviction("passionate_conviction", "Passionate Defense of Core Principles"),
    }
}
