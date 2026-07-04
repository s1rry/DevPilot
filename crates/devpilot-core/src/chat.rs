//! Repository-aware chat context building. Pure, deterministic functions that
//! turn a repository snapshot and a conversation into the messages sent to an
//! LLM, within an approximate token budget.
//!
//! Token counts are estimated as `chars / 4` — good enough to keep prompts
//! within a model's context window without pulling in a tokenizer.

use crate::entities::{ChatMessage, Language, Role};

/// Default prompt token budget when the caller does not specify one.
pub const DEFAULT_TOKEN_BUDGET: usize = 12_000;

/// Characters per token, for rough budgeting.
const CHARS_PER_TOKEN: usize = 4;

/// Hard cap on a single file's included content, in characters.
const MAX_FILE_CHARS: usize = 4_000;

/// A file included in the chat context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelevantFile {
    /// Path relative to the repository root.
    pub path: String,
    /// File content (already read).
    pub content: String,
}

/// A snapshot of the repository used to build chat context.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RepoContext {
    /// Project name.
    pub name: String,
    /// Current branch.
    pub branch: String,
    /// Per-language file counts, most first.
    pub languages: Vec<(Language, usize)>,
    /// Top-level directory names.
    pub top_level_dirs: Vec<String>,
    /// Files selected as relevant to the question.
    pub relevant_files: Vec<RelevantFile>,
}

/// Estimates the token count of a string.
fn estimate_tokens(text: &str) -> usize {
    text.len() / CHARS_PER_TOKEN + 1
}

/// Selects up to `max` repository files most relevant to `question`, by simple
/// word-overlap with the file path. Returns paths ordered by descending score.
pub fn select_relevant(paths: &[String], question: &str, max: usize) -> Vec<String> {
    let words: Vec<String> = question
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.len() >= 3)
        .map(|word| word.to_lowercase())
        .collect();

    let mut scored: Vec<(usize, &String)> = paths
        .iter()
        .map(|path| {
            let lower = path.to_lowercase();
            let score = words.iter().filter(|word| lower.contains(*word)).count();
            (score, path)
        })
        .filter(|(score, _)| *score > 0)
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
    scored
        .into_iter()
        .take(max)
        .map(|(_, path)| path.clone())
        .collect()
}

/// Builds the system prompt, including relevant files up to `file_budget`
/// tokens (each file also capped by [`MAX_FILE_CHARS`]).
fn system_prompt(context: &RepoContext, file_budget: usize) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are DevPilot, an assistant that answers questions about a specific code \
         repository using the provided context. Be concise, and cite file paths when \
         relevant.\n\n",
    );
    prompt.push_str(&format!(
        "Repository: {} (branch {})\n",
        context.name, context.branch
    ));

    if !context.languages.is_empty() {
        let languages: Vec<String> = context
            .languages
            .iter()
            .map(|(language, count)| format!("{} ({count})", language.name()))
            .collect();
        prompt.push_str(&format!("Languages: {}\n", languages.join(", ")));
    }
    if !context.top_level_dirs.is_empty() {
        prompt.push_str(&format!(
            "Top-level directories: {}\n",
            context.top_level_dirs.join(", ")
        ));
    }

    let mut spent = estimate_tokens(&prompt);
    if !context.relevant_files.is_empty() {
        prompt.push_str("\nRelevant files:\n");
        for file in &context.relevant_files {
            if spent >= file_budget {
                break;
            }
            let remaining_chars = (file_budget - spent).saturating_mul(CHARS_PER_TOKEN);
            let limit = remaining_chars.min(MAX_FILE_CHARS);
            let content: String = file.content.chars().take(limit).collect();
            let block = format!("\n--- {} ---\n```\n{}\n```\n", file.path, content);
            spent += estimate_tokens(&block);
            prompt.push_str(&block);
        }
    }

    prompt
}

/// Builds the messages to send to the model: a system prompt with repository
/// context, followed by as much recent history as fits the budget (the most
/// recent message is always kept).
pub fn build_messages(
    context: &RepoContext,
    history: &[ChatMessage],
    token_budget: usize,
) -> Vec<ChatMessage> {
    // Give at most half the budget to file context.
    let system = system_prompt(context, token_budget / 2);
    let mut remaining = token_budget.saturating_sub(estimate_tokens(&system));

    let mut kept: Vec<ChatMessage> = Vec::new();
    for message in history.iter().rev() {
        let cost = estimate_tokens(&message.content);
        if kept.is_empty() || cost <= remaining {
            remaining = remaining.saturating_sub(cost);
            kept.push(message.clone());
        } else {
            break;
        }
    }
    kept.reverse();

    let mut messages = Vec::with_capacity(kept.len() + 1);
    messages.push(ChatMessage::new(Role::System, system));
    messages.extend(kept);
    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> Vec<String> {
        vec![
            "src/parser.rs".to_string(),
            "src/lexer.rs".to_string(),
            "README.md".to_string(),
        ]
    }

    #[test]
    fn select_relevant_matches_question_words() {
        let selected = select_relevant(&paths(), "how does the parser work?", 5);
        assert_eq!(selected, vec!["src/parser.rs".to_string()]);
    }

    #[test]
    fn select_relevant_returns_empty_when_no_overlap() {
        assert!(select_relevant(&paths(), "unrelated deployment pipeline", 5).is_empty());
    }

    #[test]
    fn build_messages_starts_with_system_context() {
        let context = RepoContext {
            name: "demo".into(),
            branch: "main".into(),
            languages: vec![(Language::Rust, 3)],
            top_level_dirs: vec!["src".into()],
            relevant_files: vec![RelevantFile {
                path: "src/parser.rs".into(),
                content: "fn parse() {}".into(),
            }],
        };
        let history = vec![ChatMessage::new(Role::User, "what does parser.rs do?")];
        let messages = build_messages(&context, &history, DEFAULT_TOKEN_BUDGET);

        assert_eq!(messages[0].role, Role::System);
        assert!(messages[0].content.contains("Repository: demo"));
        assert!(messages[0].content.contains("src/parser.rs"));
        assert_eq!(messages.last().unwrap().role, Role::User);
    }

    #[test]
    fn build_messages_keeps_last_message_even_over_budget() {
        let context = RepoContext::default();
        let long = "x".repeat(10_000);
        let history = vec![ChatMessage::new(Role::User, long)];
        // Tiny budget: the last message must still be kept.
        let messages = build_messages(&context, &history, 10);
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[1].role, Role::User);
    }
}
