use include_dir::{include_dir, Dir};

use crate::prelude::*;

static PROMPTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/prompts");

pub fn get_prompt(id: String) -> Result<String> {
    let prompt_path = format!("{}/{}.prompt.hbs", &id, &id);
    let prompt = PROMPTS_DIR
        .get_file(prompt_path.clone())
        .ok_or_else(|| FinanalizeError::MissingPromptFile(prompt_path.clone()))?
        .contents_utf8()
        .ok_or_else(|| FinanalizeError::MissingPromptUTF8(prompt_path))?
        .to_string();

    Ok(prompt)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_prompt() {
        let prompt = super::get_prompt("title".to_string()).unwrap();
        dbg!(&prompt);
        assert!(prompt.contains("title"));
    }
}
