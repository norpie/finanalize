use crate::latex::*;
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::fs::*;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
struct TemplateData {
    report_title: String,
    report_subtitle: String,
    components: Vec<LatexCommand>,
}

pub fn construct_report(
    commands: Vec<LatexCommand>,
    report_title: String,
    report_subtitle: String,
) -> Result<()> {
    let data = TemplateData {
        report_title: report_title.clone(),
        report_subtitle: report_subtitle.clone(),
        components: commands,
    };
    let backend_dir = env::current_dir()?;
    let project_root = backend_dir.parent().unwrap();
    // Remove tmp dir and create it.
    let tmp_dir = project_root.join("tmp");
    remove_dir_all(&tmp_dir).ok();
    if !tmp_dir.exists() {
        create_dir_all(&tmp_dir)?;
    }
    // Retrieve latex path and template path
    let latex_dir = project_root.join("latex");
    let template_path = latex_dir.join("report.tex.hbs");
    // Define path where the rendered LaTeX will be written to
    let output_path = &tmp_dir.join(report_title.replace(" ", "_").to_lowercase() + ".tex");
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_tex = handlebars.render_template(&template, &data)?;
    // Write the rendered LaTeX to a file on output_path
    write(output_path, rendered_tex)?;
    // Copy latex directory to tmp directory for compiling
    copy_latex_dir(&latex_dir, &tmp_dir)?;
    println!("Rendered LaTeX written to {}", output_path.display());
    // Compile the given report and other files into pdf using pdflatex
    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(&tmp_dir)
        .arg(output_path)
        .output()?;
    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(&tmp_dir)
        .arg(output_path)
        .output()?;
    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(&tmp_dir)
        .arg(output_path)
        .output()?;
    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(&tmp_dir)
        .arg(output_path)
        .output()?;
    Ok(())
}

fn copy_latex_dir(latex_dir: &Path, output_dir: &Path) -> Result<()> {
    // Recursively copy the latex directory to the output directory (tmp)
    for file in read_dir(latex_dir)? {
        let file = file?;
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let dest_path = output_dir.join(file_name);
        if file_path.is_dir() {
            create_dir_all(&dest_path)?;
            copy_latex_dir(&file_path, &dest_path)?;
        } else {
            copy(file_path, dest_path)?;
        }
    }
    // Cleanup tmp directory so that only needed files for compiling are present
    let template_file = output_dir.join("report.tex.hbs");
    let gitkeep_file = output_dir.join(".gitkeep");
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
            LatexComponent::Citation(Citation {
                inline_text: "Test Citation".to_string(),
                source: "Test Source".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::List(List {
                is_numbered: true,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Link(Link {
                is_mailto: false,
                text: "Test Link".to_string(),
                url: "https://example.com".to_string(),
            }),
            LatexComponent::Link(Link {
                is_mailto: true,
                text: "Test Email".to_string(),
                url: "test@test.com".to_string(),
            }),
            LatexComponent::Quotation(Quotation {
                text: "Test Quotation".to_string(),
                author: "Test Author".to_string(),
            }),
            LatexComponent::Figure(Figure {
                caption: "Test Caption".to_string(),
                path: "image".to_string(),
            }),
        ];
        let commands = get_commands(components).unwrap();
        construct_report(
            commands,
            "Test Report".to_string(),
            "This is a test report".to_string(),
        )
        .unwrap();
    }
}
