use crate::prelude::*;
use serde::Serialize;

pub mod renderer;

pub enum LatexComponent {
    Section(Section),
    Subsection(Subsection),
    Text(String),
    Figure(Figure),
    Table(Table),
    Quotation(Quotation),
    Citation(String),
    List(List),
    Link(Link),
    // Equation(Equation),
}

pub struct Section {
    pub heading: String,
}

pub struct Subsection {
    pub heading: String,
}

pub struct Figure {
    pub caption: String,
    pub path: String,
}

pub struct Table {
    pub caption: String,
    pub rows: Vec<Vec<String>>,
    pub columns: Vec<String>,
}

pub struct Quotation {
    pub text: String,
    pub author: String,
}

pub struct Citation {
    pub source: String,
}

pub struct List {
    pub is_numbered: bool,
    pub items: Vec<String>,
}

pub struct Link {
    pub is_mailto: bool,
    pub text: String,
    pub url: String,
}

// pub struct Equation {
//     equation: String,
// }

#[derive(Serialize)]
pub struct Source {
    pub source_type: String,
    pub citation_key: String,
    pub author: String,
    pub title: String,
    pub year: i32,
    pub journal: String,
    pub url: String,
}

impl Source {
    pub fn new(
        source_type: String,
        citation_key: String,
        author: String,
        title: String,
        year: i32,
        journal: String,
        url: String,
    ) -> Self {
        Source {
            source_type,
            citation_key,
            author,
            title,
            year,
            journal,
            url,
        }
    }
}

#[derive(Serialize)]
pub struct LatexCommand {
    command: String,
    args: String,
}

pub fn get_commands(components: Vec<LatexComponent>) -> Result<Vec<LatexCommand>> {
    let mut commands: Vec<LatexCommand> = Vec::new();
    for component in components.iter() {
        match component {
            LatexComponent::Section(section) => {
                commands.push(LatexCommand {
                    command: format!(r"\section{{{}}}", section.heading.clone()),
                    args: "".to_string(),
                });
            }
            LatexComponent::Subsection(subsection) => {
                commands.push(LatexCommand {
                    command: format!(r"\subsection{{{}}}", subsection.heading.clone()),
                    args: "".to_string(),
                });
            }
            LatexComponent::Text(text) => {
                let escaped_text = escape_special_chars(text.clone());
                commands.push(LatexCommand {
                    command: "".to_string(),
                    args: escaped_text.clone(),
                });
            }
            LatexComponent::Citation(citation_key) => {
                commands.push(LatexCommand {
                    command: format!(r"\textcite{{{}}}", citation_key.clone()),
                    args: "".to_string(),
                });
            }
            LatexComponent::List(list) => {
                if list.is_numbered {
                    commands.push(LatexCommand {
                        command: r"\begin{enumerate}".to_string(),
                        args: "".to_string(),
                    });
                } else {
                    commands.push(LatexCommand {
                        command: r"\begin{itemize}".to_string(),
                        args: "".to_string(),
                    });
                }
                for item in list.items.iter() {
                    commands.push(LatexCommand {
                        command: r"\item".to_string(),
                        args: item.clone(),
                    });
                }
                if list.is_numbered {
                    commands.push(LatexCommand {
                        command: r"\end{enumerate}".to_string(),
                        args: "".to_string(),
                    });
                } else {
                    commands.push(LatexCommand {
                        command: r"\end{itemize}".to_string(),
                        args: "".to_string(),
                    });
                }
            }
            LatexComponent::Link(link) => {
                if link.is_mailto {
                    commands.push(LatexCommand {
                        command: format!(r"\href{{mailto:{}}}", link.url),
                        args: format!("{{{}}}", link.text.clone()),
                    });
                } else {
                    commands.push(LatexCommand {
                        command: format!(r"\href{{{}}}", link.url),
                        args: format!("{{{}}}", link.text.clone()),
                    });
                }
            }
            LatexComponent::Quotation(quotation) => {
                commands.push(LatexCommand {
                    command: r"\begin{quote}".to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\textbf{\LARGE}".to_string(),
                    args: format!(r"\lq {}", quotation.text.clone()) + r"\rq",
                });
                commands.push(LatexCommand {
                    command: r"\hfill---".to_string(),
                    args: quotation.author.clone(),
                });
                commands.push(LatexCommand {
                    command: r"\end{quote}".to_string(),
                    args: "".to_string(),
                });
            }
            LatexComponent::Figure(figure) => {
                commands.push(LatexCommand {
                    command: r"\begin{figure}[H]".to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\includegraphics[width=\linewidth]".to_string(),
                    args: format!("{{{}}}", figure.path.clone()),
                });
                commands.push(LatexCommand {
                    command: format!(r"\caption{{{}}}", figure.caption.clone()),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\end{figure}".to_string(),
                    args: "".to_string(),
                });
            }
            LatexComponent::Table(table) => {
                commands.push(LatexCommand {
                    command: r"\begin{table}[H]".to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: format!(r"\caption{{{}}}", table.caption.clone()),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command:
                        r"\begin{tabular}{L{0.35\linewidth} L{0.38\linewidth} L{0.16\linewidth}}"
                            .to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\toprule".to_string(),
                    args: "".to_string(),
                });
                commands.push(format_table_command(table, true));
                commands.push(LatexCommand {
                    command: r"\midrule".to_string(),
                    args: "".to_string(),
                });
                commands.push(format_table_command(table, false));
                commands.push(LatexCommand {
                    command: r"\bottomrule".to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\end{tabular}".to_string(),
                    args: "".to_string(),
                });
                commands.push(LatexCommand {
                    command: r"\end{table}".to_string(),
                    args: "".to_string(),
                });
            } // LatexComponent::Equation(equation) => {
              //     commands.push(LatexCommand {
              //         command: r"\begin{equation}".to_string(),
              //         args: "".to_string(),
              //     });
              //     commands.push(LatexCommand {
              //         command: equation.equation.clone(),
              //         args: "".to_string(),
              //     });
              //     commands.push(LatexCommand {
              //         command: r"\end{equation}".to_string(),
              //         args: "".to_string(),
              //     });
              // }
        }
    }
    Ok(commands)
}

fn format_table_command(table: &Table, is_column: bool) -> LatexCommand {
    if is_column {
        let formatted_data = table
            .columns
            .iter()
            .map(|item| format!(r"\textbf{{{}}}", item))
            .collect::<Vec<_>>()
            .join(" & ");
        LatexCommand {
            command: formatted_data,
            args: r" \\".to_string(),
        }
    } else {
        let formatted_data = table
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join(" & ")
            })
            .collect::<Vec<_>>()
            .join(r" \\");
        LatexCommand {
            command: formatted_data,
            args: r" \\".to_string(),
        }
    }
}

fn escape_special_chars(input: String) -> String {
    let special_chars: &[char] = &['&', '%', '$', '#', '_', '{', '}', '~', '^', '€', '\\'];
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        if special_chars.contains(&ch) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}
