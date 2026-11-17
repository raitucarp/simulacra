//! Fields of the AI & Models, Embeddings and Images categories.

use super::macros::category_fields;
use super::value::{choice, choice_value, choices, count, limit, number, number_value, text, toggle, FieldKind, NumberUnit, SettingValue};
use crate::domain::preferences::{AiPrefs, DAILY_REQUESTS, DAILY_TOKENS, MAX_RETRIES, REQUEST_TIMEOUT_SECONDS};
use crate::domain::preferences::{EmbeddingPrefs, EmbeddingProvider, CHUNK_OVERLAP, CHUNK_SIZE};
use crate::domain::preferences::{ImageAspect, ImageEngineChoice, ImagePrefs};

category_fields! {
    /// An AI & Models preference (provider, key and model keep their own rows).
    AiField {
        RequestTimeout => "request_timeout_seconds", MaxRetries => "max_retries",
        DailyRequestBudget => "daily_request_budget", DailyTokenCap => "daily_token_cap",
        AskBeforeSpending => "ask_before_spending",
    }
}

impl AiField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::RequestTimeout => number(REQUEST_TIMEOUT_SECONDS, NumberUnit::Seconds),
            Self::MaxRetries => number(MAX_RETRIES, NumberUnit::Attempts),
            Self::DailyRequestBudget => limit(DAILY_REQUESTS, NumberUnit::Requests),
            Self::DailyTokenCap => limit(DAILY_TOKENS, NumberUnit::TokensPerDay),
            Self::AskBeforeSpending => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &AiPrefs) -> SettingValue {
        match self {
            Self::RequestTimeout => number_value(prefs.request_timeout_seconds),
            Self::MaxRetries => number_value(prefs.max_retries),
            Self::DailyRequestBudget => number_value(prefs.daily_request_budget),
            Self::DailyTokenCap => number_value(prefs.daily_token_cap),
            Self::AskBeforeSpending => SettingValue::Toggle(prefs.ask_before_spending),
        }
    }

    pub fn set(self, prefs: AiPrefs, value: &SettingValue) -> AiPrefs {
        match self {
            Self::RequestTimeout => AiPrefs { request_timeout_seconds: count(value, prefs.request_timeout_seconds), ..prefs },
            Self::MaxRetries => AiPrefs { max_retries: count(value, prefs.max_retries), ..prefs },
            Self::DailyRequestBudget => AiPrefs { daily_request_budget: count(value, prefs.daily_request_budget), ..prefs },
            Self::DailyTokenCap => AiPrefs { daily_token_cap: count(value, prefs.daily_token_cap), ..prefs },
            Self::AskBeforeSpending => AiPrefs { ask_before_spending: toggle(value, prefs.ask_before_spending), ..prefs },
        }
    }
}

category_fields! {
    /// An Embeddings preference.
    EmbeddingField {
        Provider => "provider", Model => "model", ChunkSize => "chunk_size", ChunkOverlap => "chunk_overlap",
        SemanticSearch => "semantic_search",
    }
}

impl EmbeddingField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::Provider => choices::<EmbeddingProvider>(),
            Self::Model => FieldKind::Text,
            Self::ChunkSize => number(CHUNK_SIZE, NumberUnit::Tokens),
            Self::ChunkOverlap => number(CHUNK_OVERLAP, NumberUnit::Tokens),
            Self::SemanticSearch => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &EmbeddingPrefs) -> SettingValue {
        match self {
            Self::Provider => choice_value(prefs.provider),
            Self::Model => SettingValue::Text(prefs.model.clone()),
            Self::ChunkSize => number_value(prefs.chunk_size),
            Self::ChunkOverlap => number_value(prefs.chunk_overlap),
            Self::SemanticSearch => SettingValue::Toggle(prefs.semantic_search),
        }
    }

    pub fn set(self, prefs: EmbeddingPrefs, value: &SettingValue) -> EmbeddingPrefs {
        match self {
            Self::Provider => EmbeddingPrefs { provider: choice(value, prefs.provider), ..prefs },
            Self::Model => EmbeddingPrefs { model: text(value, prefs.model.clone()), ..prefs },
            Self::ChunkSize => EmbeddingPrefs { chunk_size: count(value, prefs.chunk_size), ..prefs },
            Self::ChunkOverlap => EmbeddingPrefs { chunk_overlap: count(value, prefs.chunk_overlap), ..prefs },
            Self::SemanticSearch => EmbeddingPrefs { semantic_search: toggle(value, prefs.semantic_search), ..prefs },
        }
    }
}

category_fields! {
    /// An Images preference.
    ImageField { DefaultEngine => "default_engine", Size => "size", StylePrefix => "style_prefix", AutoAvatars => "auto_avatars" }
}

impl ImageField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::DefaultEngine => choices::<ImageEngineChoice>(),
            Self::Size => choices::<ImageAspect>(),
            Self::StylePrefix => FieldKind::Text,
            Self::AutoAvatars => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &ImagePrefs) -> SettingValue {
        match self {
            Self::DefaultEngine => choice_value(prefs.default_engine),
            Self::Size => choice_value(prefs.size),
            Self::StylePrefix => SettingValue::Text(prefs.style_prefix.clone()),
            Self::AutoAvatars => SettingValue::Toggle(prefs.auto_avatars),
        }
    }

    pub fn set(self, prefs: ImagePrefs, value: &SettingValue) -> ImagePrefs {
        match self {
            Self::DefaultEngine => ImagePrefs { default_engine: choice(value, prefs.default_engine), ..prefs },
            Self::Size => ImagePrefs { size: choice(value, prefs.size), ..prefs },
            Self::StylePrefix => ImagePrefs { style_prefix: text(value, prefs.style_prefix.clone()), ..prefs },
            Self::AutoAvatars => ImagePrefs { auto_avatars: toggle(value, prefs.auto_avatars), ..prefs },
        }
    }
}
