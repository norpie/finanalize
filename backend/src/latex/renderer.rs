use crate::latex::*;
use crate::prelude::*;
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::path::Path;
use std::fs::*;
#[derive(Serialize)]
struct TemplateData {
    report_title: String,
    report_subtitle: String,
    components: Vec<LatexCommand>,
}

#[derive(Serialize)]
struct LatexCommand {
    command: String,
    args: String,
}

pub fn construct_report(
    components: Vec<LatexComponent>,
    report_title: String,
    report_subtitle: String,
) -> Result<()> {
    let mut commands: Vec<LatexCommand> = Vec::new();
    for component in components.iter() {
        match component {
            LatexComponent::Section(section) => {
                commands.push(LatexCommand {
                    command: r"\section".to_string(),
                    args: section.heading.clone(),
                });
            }
            LatexComponent::Subsection(subsection) => {
                commands.push(LatexCommand {
                    command: r"\subsection".to_string(),
                    args: subsection.heading.clone(),
                });
            }
            LatexComponent::Text(text) => {
                commands.push(LatexCommand {
                    command: "".to_string(),
                    args: text.clone(),
                });
            }
            _ => {}
        }
    }
    let data = TemplateData {
        report_title: report_title.clone(),
        report_subtitle: report_subtitle.clone(),
        components: commands,
    };

    let backend_dir = env::current_dir()?;
    let project_root = backend_dir.parent().unwrap();
    let latex_dir = project_root.join("latex");
    dbg!(&latex_dir);
    let template_path = latex_dir.join("report.tex.hbs");
    dbg!(&template_path);
    let tmp_dir = project_root.join("tmp");
    dbg!(&tmp_dir);
    remove_dir_all(&tmp_dir).ok();
    if !tmp_dir.exists() {
        create_dir_all(&tmp_dir)?;
    }
    let output_path = tmp_dir.join(report_title.replace(" ", "_").to_lowercase() + ".tex");
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_tex = handlebars.render_template(&template, &data)?;
    write(&output_path, rendered_tex)?;
    copy_latex_dir(&latex_dir, &tmp_dir)?;
    println!("Rendered LaTeX written to {}", output_path.display());
    todo!("Implement LaTeX compilation, will use Docker image");
    Ok(())
}

fn copy_latex_dir(latex_dir: &Path, tmp_dir: &Path) -> Result<()> {
    for file in read_dir(latex_dir)? {
        let file = file?;
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let dest_path = tmp_dir.join(file_name);
        if file_path.is_dir() {
            create_dir_all(&dest_path)?;
            copy_latex_dir(&file_path, &dest_path)?;
        } else {
            copy(file_path, dest_path)?;
        }
    }
    let template_file = tmp_dir.join("report.tex.hbs");
    let gitkeep_file = tmp_dir.join(".gitkeep");
    if template_file.exists() {
        remove_file(template_file)?;
    }
    if gitkeep_file.exists() {
        remove_file(gitkeep_file)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_report() {
        let components = vec![
            LatexComponent::Section(Section {
                heading: "Test Section".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Test Subsection".to_string(),
            }),
            LatexComponent::Text("Test Text".to_string()),
        ];
        construct_report(
            components,
            "Test Report".to_string(),
            "This is a test report".to_string(),
        )
        .unwrap();
    }
}
