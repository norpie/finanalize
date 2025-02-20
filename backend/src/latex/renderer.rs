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

#[derive(Serialize)]
struct BibData {
    sources: Vec<Source>,
}
pub fn construct_report(
    sources: Vec<Source>,
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
    construct_bib_file(sources)?;
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_tex = handlebars.render_template(&template, &data)?;
    // Write the rendered LaTeX to a file on output_path
    write(output_path, rendered_tex)?;
    // Copy latex directory to tmp directory for compiling
    copy_latex_dir(&latex_dir, &tmp_dir)?;
    println!("Rendered LaTeX written to {}", output_path.display());
    compile_latex(output_path, &tmp_dir, false)?;
    compile_latex(output_path, &tmp_dir, true)?;
    compile_latex(output_path, &tmp_dir, false)?;
    compile_latex(output_path, &tmp_dir, false)?;
    cleanup_tmp_dir(&tmp_dir)?;
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
    let tex_template = output_dir.join("report.tex.hbs");
    let bib_template = output_dir.join("references.bib.hbs");
    let gitkeep_file = output_dir.join(".gitkeep");
    if tex_template.exists() {
        remove_file(tex_template)?;
    }
    if bib_template.exists() {
        remove_file(bib_template)?;
    }
    if gitkeep_file.exists() {
        remove_file(gitkeep_file)?;
    }
    Ok(())
}

fn construct_bib_file(sources: Vec<Source>) -> Result<()> {
    let data = BibData { sources };
    let tmp_dir = env::current_dir()?.parent().unwrap().join("tmp");
    let bib_path = tmp_dir.join("references.bib");
    let template_path = env::current_dir()?
        .parent()
        .unwrap()
        .join("latex/references.bib.hbs");
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_bib = handlebars.render_template(&template, &data)?;
    write(bib_path, rendered_bib)?;
    Ok(())
}

fn cleanup_tmp_dir(tmp_dir: &Path) -> Result<()> {
    for entry in read_dir(tmp_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_dir_all(path)?;
        } else if let Some(ext) = path.extension() {
            if ext != "pdf" {
                remove_file(path)?;
            }
        } else {
            remove_file(path)?;
        }
    }
    Ok(())
}

fn compile_latex(input: &Path, output: &Path, is_bib: bool) -> Result<()> {
    if is_bib {
        Command::new("biber")
            .arg("-output-directory")
            .arg(output)
            .arg(input.file_stem().unwrap())
            .output()?;
        return Ok(());
    }
    Command::new("pdflatex")
        .arg("-output-directory")
        .arg(output)
        .arg(input)
        .output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_report() {
        let sources = vec![
            Source {
                source_type: "MISC".to_string(),
                citation_key: "smith2024".to_string(),
                author: "John Smith".to_string(),
                title: "The Future of AI".to_string(),
                year: 2024,
                journal: "Tech Insights".to_string(),
                url: "https://example.com/future-ai".to_string(),
            },
            Source {
                source_type: "MISC".to_string(),
                citation_key: "doe2023".to_string(),
                author: "Jane Doe".to_string(),
                title: "Blockchain and Data Security".to_string(),
                year: 2023,
                journal: "Cyber Journal".to_string(),
                url: "https://example.com/blockchain-security".to_string(),
            },
        ];
        let components = vec![
            LatexComponent::Section(Section {
                heading: "Test Section".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Test Subsection".to_string(),
            }),
            LatexComponent::Text("Test Text".to_string()),
            LatexComponent::Citation(sources[0].citation_key.clone()),
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
            LatexComponent::Table(Table {
                caption: "Table caption".to_string(),
                rows: vec![
                    vec![
                        "Row1Col1".to_string(),
                        "Row1Col2".to_string(),
                        "Row1Col3".to_string(),
                    ],
                    vec![
                        "Row2Col1".to_string(),
                        "Row2Col2".to_string(),
                        "Row2Col3".to_string(),
                    ],
                ],
                columns: vec![
                    "Header1".to_string(),
                    "Header2".to_string(),
                    "Header3".to_string(),
                ],
            }),
        ];
        let commands = get_commands(components).unwrap();
        construct_report(
            sources,
            commands,
            "Test Report".to_string(),
            "This is a test report".to_string(),
        )
        .unwrap();
    }
}
