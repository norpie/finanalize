use crate::prelude::*;
use serde::Serialize;
mod renderer;

pub enum LatexComponent {
    Section(Section),
    Subsection(Subsection),
    Text(String),
    Figure(Figure),
    Table(Table),
    Quotation(Quotation),
    Citation(Citation),
    List(List),
    Link(Link),
    // Equation(Equation),
}

pub struct Section {
    heading: String,
}

pub struct Subsection {
    heading: String,
}

pub struct Figure {
    caption: String,
    path: String,
}

pub struct Table {
    caption: String,
    rows: Vec<Vec<String>>,
    columns: Vec<String>,
}

pub struct Quotation {
    text: String,
    author: String,
}

pub struct Citation {
    inline_text: String,
    source: String,
}

pub struct List {
    is_numbered: bool,
    items: Vec<String>,
}

pub struct Link {
    is_mailto: bool,
    text: String,
    url: String,
}

// pub struct Equation {
//     equation: String,
// }

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
                commands.push(LatexCommand {
                    command: "".to_string(),
                    args: text.clone(),
                });
            }
            LatexComponent::Citation(citation) => {
                commands.push(LatexCommand {
                    command: format!(r"\textcite{{{}}}", citation.inline_text.clone()),
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
                commands.push(format_command(table, true));
                commands.push(LatexCommand {
                    command: r"\midrule".to_string(),
                    args: "".to_string(),
                });
                commands.push(format_command(table, false));
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

fn format_command(table: &Table, is_column: bool) -> LatexCommand {
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
