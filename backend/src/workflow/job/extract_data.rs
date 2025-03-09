use async_trait::async_trait;
use log::error;
use markdown::mdast::Node;
use markdown::ParseOptions;

use crate::prelude::*;

use crate::workflow::WorkflowState;

use super::Job;

pub struct ExtractDataJob;

fn recurse_find_tables(node: Node) -> Vec<Node> {
    let mut tables = vec![];
    match node {
        Node::Table(_) => tables.push(node),
        Node::Root(root) => {
            for child in root.children {
                tables.extend(recurse_find_tables(child));
            }
        }
        _ => {}
    }
    tables
}

// Since a cell is made up of children, we need to recurse to find the actual content
fn find_cell_content(children: Vec<Node>) -> String {
    let mut content = String::new();
    for child in children {
        match child {
            Node::Text(text) => content.push_str(&text.value),
            Node::Emphasis(emphasis) => content.push_str(&find_cell_content(emphasis.children)),
            Node::Strong(strong) => content.push_str(&find_cell_content(strong.children)),
            _ => {}
        }
    }
    content
}

#[async_trait]
impl Job for ExtractDataJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        let mut csvs = vec![];
        for source in state.state.md_sources.clone().unwrap() {
            let ast = markdown::to_mdast(&source.content, &ParseOptions::default()).unwrap();
            let tables = recurse_find_tables(ast);
            for node in tables {
                let Node::Table(table) = node else {
                    error!("Invalid state: table is not a Table");
                    return Err(FinanalizeError::InternalServerError);
                };
                let mut csv = String::new();
                for node in table.children {
                    let Node::TableRow(row) = node else {
                        error!("Invalid state: table child is not a TableRow");
                        return Err(FinanalizeError::InternalServerError);
                    };
                    for node in row.children {
                        let Node::TableCell(cell) = node else {
                            error!("Invalid state: table row child is not a TableCell");
                            return Err(FinanalizeError::InternalServerError);
                        };
                        csv.push_str(find_cell_content(cell.children).as_str());
                        csv.push(',');
                    }
                    csv.pop();
                    csv.push('\n');
                }
                csvs.push(csv);
            }
        }
        state.state.csv_sources = Some(csvs);
        Ok(state)
    }
}
