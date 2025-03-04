use crate::latex::*;
use handlebars::Handlebars;
use log::debug;
use serde::Serialize;
use std::env;
use std::fs::*;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

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

#[derive(Debug)]
pub struct PdfReport {
    pub uuid: String,
    pub report_title: String,
    pub report_path: String,
}

pub fn construct_report(
    sources: Vec<Source>,
    commands: Vec<LatexCommand>,
    report_title: String,
    report_subtitle: String,
) -> Result<PdfReport> {
    debug!("Constructing report: {}", report_title);
    let data = TemplateData {
        report_title: report_title.clone(),
        report_subtitle: report_subtitle.clone(),
        components: commands,
    };
    let backend_dir = env::current_dir()?;
    let project_root = backend_dir.parent().unwrap();
    let tmp_dir = env::temp_dir();
    let uuid = Uuid::new_v4().to_string();
    let destination_folder = &tmp_dir.join(&uuid);
    create_dir_all(destination_folder)?;
    debug!(
        "Destination folder created: {:?}",
        destination_folder.display()
    );
    let latex_dir = project_root.join("latex");
    let template_path = latex_dir.join("report.tex.hbs");
    let output_path =
        &destination_folder.join(report_title.replace(" ", "_").to_lowercase() + ".tex");
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_tex = handlebars.render_template(&template, &data)?;
    write(output_path, rendered_tex)?;
    debug!("Rendered LaTeX file: {:?}", output_path.display());
    debug!(
        "Copying LaTeX files to {:?} for compiling",
        destination_folder.display()
    );
    copy_latex_dir(&latex_dir, destination_folder)?;
    debug!("Constructing bibliography file based on references template");
    construct_bib_file(sources, destination_folder)?;
    debug!("Compiling LaTeX file for the first time");
    compile_latex(output_path, destination_folder, false)?;
    debug!("Compiling LaTeX file with bibliography");
    compile_latex(output_path, destination_folder, true)?;
    debug!("Compiling LaTeX file for the second time");
    compile_latex(output_path, destination_folder, false)?;
    debug!("Compiling LaTeX file for the third time");
    compile_latex(output_path, destination_folder, false)?;
    debug!("Compiling LaTeX file for the fourth & last time");
    compile_latex(output_path, destination_folder, false)?;
    let pdf_path = output_path.with_extension("pdf");
    debug!(
        "Cleaning up {:?}, removing all files except the report PDF",
        destination_folder.display()
    );
    // cleanup_destination_folder(destination_folder)?;
    debug!("Report compiled successfully: {:?}", pdf_path.display());
    Ok(PdfReport {
        uuid,
        report_title,
        report_path: pdf_path.to_str().unwrap().to_string(),
    })
}

fn copy_latex_dir(latex_dir: &Path, output_dir: &Path) -> Result<()> {
    for file in read_dir(latex_dir)? {
        let file = file?;
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let dest_path = output_dir.join(file_name);
        if file_path.is_dir() {
            debug!("Copying directory: {:?}", file_path.display());
            create_dir_all(&dest_path)?;
            copy_latex_dir(&file_path, &dest_path)?;
        } else {
            debug!("Copying file: {:?}", file_path.display());
            copy(file_path, dest_path)?;
        }
    }
    debug!("Copied all LaTeX files to {:?}", output_dir.display());
    Ok(())
}

fn construct_bib_file(sources: Vec<Source>, destination_folder: &Path) -> Result<()> {
    let data = BibData { sources };
    let bib_path = destination_folder.join("references.bib");
    let template_path = env::current_dir()?
        .parent()
        .unwrap()
        .join("latex/references.bib.hbs");
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path)?;
    let rendered_bib = handlebars.render_template(&template, &data)?;
    write(&bib_path, rendered_bib)?;
    debug!("Rendered bibliography file: {:?}", &bib_path.display());
    Ok(())
}

fn cleanup_destination_folder(destination_folder: &Path) -> Result<()> {
    for entry in read_dir(destination_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            debug!("Removing directory: {:?}", path);
            remove_dir_all(path)?;
        } else if let Some(ext) = path.extension() {
            if ext != "pdf" {
                debug!("Removing file: {:?}", path);
                remove_file(path)?;
            }
        }
    }
    debug!("Destination folder cleaned up successfully");
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
    let output = Command::new("pdflatex")
        .arg("-output-directory")
        .arg(output)
        .arg(input)
        .output()?;
    dbg!(output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Depends on external resources"]
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
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::List(List {
                is_numbered: true,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),
            LatexComponent::List(List {
                is_numbered: true,
                items: vec!["Test Item".to_string(), "Test Item 2".to_string()],
            }),          LatexComponent::Section(Section {
                heading: "Financial Report for Q4 2024".to_string(),
            }),
            LatexComponent::Subsection(Subsection {
                heading: "Executive Summary".to_string(),
            }),
            LatexComponent::Text("The financial performance for the fourth quarter of 2024 demonstrated steady growth across key revenue streams, alongside controlled operational expenses. The company achieved a revenue increase of 8% compared to the previous quarter, while net profit margins remained stable at 12%. Strategic investments in technology and operational efficiency contributed to the overall financial stability.".to_string()),
            LatexComponent::Subsection(Subsection {
                heading: "Revenue Performance".to_string(),
            }),
            LatexComponent::List(List {
                is_numbered: false,
                items: vec![
                    "Revenue increased by 8% compared to Q3 2024".to_string(),
                    "Growth was driven by strong performance in the software division".to_string(),
                    "Hardware sales remained stable, with a slight increase in demand for premium products".to_string(),
                ],
            }),
            LatexComponent::Citation(sources[0].citation_key.clone()),
            LatexComponent::Subsection(Subsection {
                heading: "Expense Analysis".to_string(),
            }),
            LatexComponent::Text("Total revenue for Q4 2024 reached $120 million, reflecting an 8% increase from Q3 2024. The primary drivers of revenue growth included increased customer acquisition, enhanced product offerings, and expansion into new markets. Subscription-based revenue grew by 10%, while one-time sales increased by 5%.".to_string()),
            LatexComponent::Section(Section {
                heading: "Other stuff".to_string(),
            }),
            LatexComponent::Text("Profitability
Net income for Q4 2024 was $14.4 million, compared to $13.5 million in Q3 2024. EBITDA margin stood at 18%, supported by efficient cost management and revenue diversification. Earnings per share (EPS) for Q4 were $1.20, marking a 7% increase.

Cash Flow & Liquidity
Operating cash flow remained strong at $20 million, enabling continued reinvestment in growth opportunities. The company maintains a healthy cash reserve of $50 million with a low debt-to-equity ratio of 0.35.

Outlook for Q1 2025
Continued investment in technology and expansion into new geographic markets is expected to drive further revenue growth. Cost optimization measures and automation initiatives aim to improve profitability. The company remains confident in sustaining its growth trajectory with an expected revenue increase of 5-7% in the next quarter.

Conclusion
Q4 2024 was a strong quarter for the company, with steady revenue growth, controlled expenses, and solid profitability. The company remains well-positioned for continued financial success in the coming quarters.".to_string()),
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
