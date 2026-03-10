use crate::{
    error::{AppError, AppResult},
    models::ai::AiTaskType,
};

const PROMPT_VERSION: &str = "2026-03-10.v1";

pub fn prompt_version() -> &'static str {
    PROMPT_VERSION
}

pub fn build_prompt(
    task_type: AiTaskType,
    title: &str,
    text: &str,
    target_language: Option<&str>,
) -> AppResult<String> {
    if text.trim().is_empty() {
        return Err(AppError::validation("entry text is empty"));
    }
    let prompt = match task_type {
        AiTaskType::EntrySummary => format!(
            "你是严谨的阅读助手。请对以下内容生成中文摘要，输出 4 条要点与 1 句结论。\n标题：{title}\n正文：{text}"
        ),
        AiTaskType::EntryTranslation => format!(
            "你是专业翻译助手。请将以下内容翻译为 {}，保持结构清晰，避免省略。\n标题：{title}\n正文：{text}",
            target_language.unwrap_or("中文")
        ),
        AiTaskType::EntryTopicTags => format!(
            "你是内容分类助手。请仅输出一个 JSON 数组，数组元素是 3 到 6 个中文主题标签字符串。\n标题：{title}\n正文：{text}"
        ),
        AiTaskType::CollectionDigest => format!(
            "你是编辑部值班主编。请基于以下信息流内容撰写一份中文合集摘要，包含今日重点、趋势、风险和下一步关注点。\n{title}\n{text}"
        ),
    };
    Ok(prompt)
}

#[cfg(test)]
mod tests {
    use crate::models::ai::AiTaskType;

    use super::{build_prompt, prompt_version};

    #[test]
    fn prompt_version_is_stable() {
        assert_eq!(prompt_version(), "2026-03-10.v1");
    }

    #[test]
    fn tags_prompt_requests_json_array() {
        let prompt = build_prompt(AiTaskType::EntryTopicTags, "A", "B", None).expect("prompt");
        assert!(prompt.contains("JSON 数组"));
    }
}

