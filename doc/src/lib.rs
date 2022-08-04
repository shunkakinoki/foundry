use clap::{Parser, ValueHint};
use config::DocConfig;
use context::Context;
// use ethers_solc::{Graph, Project};
use thiserror::Error;

use forge_fmt::{Formatter, FormatterConfig, Visitable, Visitor};
// use foundry_common::paths::ProjectPathsArgs;
use solang_parser::{
    doccomment::{parse_doccomments, DocComment},
    pt::{Comment, ContractTy, FunctionDefinition, SourceUnit, SourceUnitPart},
};

mod config;
mod context;

#[derive(Error, Debug)]
pub enum DocError {} // TODO:

type Result<T, E = DocError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct SolidityDoc {
    comments: Vec<Comment>,
    contracts: Vec<DocContract>,
}

#[derive(Debug)]
pub struct DocContract {
    ty: ContractTy,
    name: String,
    comments: Vec<DocComment>,
    // TODO: functions: Vec<DocFunction>,
    parts: Vec<DocContractPart>,
}

#[derive(Debug)]
pub enum DocContractPart {
    Function(DocFunction),
}

#[derive(Debug)]
pub struct DocFunction {
    comments: Vec<Comment>,
    name: String,
    // args: Vec<DocParam>,
    // returns: Vec<DocParam>,
}

// #[derive(Debug)]
// pub struct DocParam {
//     name: Option<String>,
//     type_: String, // TODO:
// }

impl SolidityDoc {
    pub fn new(comments: Vec<Comment>) -> Self {
        SolidityDoc { contracts: vec![], comments }
    }
}

impl Visitor for SolidityDoc {
    type Error = DocError;

    fn visit_source_unit(&mut self, source_unit: &mut SourceUnit) -> Result<()> {
        for source in source_unit.0.iter_mut() {
            match source {
                SourceUnitPart::ContractDefinition(def) => {
                    self.contracts.push(DocContract {
                        name: def.name.name.clone(),
                        ty: def.ty.clone(),
                        comments: parse_doccomments(&self.comments, 0, def.loc.start()),
                        parts: vec![],
                    });
                    for d in def.parts.iter_mut() {
                        d.visit(self)?;
                    }
                }
                SourceUnitPart::FunctionDefinition(def) => {}
                _ => {}
            };
        }

        Ok(())
    }

    fn visit_function(&mut self, func: &mut FunctionDefinition) -> Result<()> {
        Ok(())
    }
}

// #[derive(Debug)]
// pub enum Doc {
//     Build(DocBuildArgs),
//     // Serve,
// }

// #[derive(Debug, Clone, Parser)]
// pub struct DocBuildArgs {
//     #[clap(flatten, next_help_heading = "PROJECT OPTIONS")]
//     project_paths: ProjectPathsArgs,
// }

// impl DocBuildArgs {
//     fn run(self) {
//         //
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[allow(non_snake_case)]
    #[test]
    fn test_docs() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata").join("Governor.sol");
        let target = fs::read_to_string(path).unwrap();
        let (mut source_pt, source_comments) = solang_parser::parse(&target, 1).unwrap();
        // let source_comments = Comments::new(source_comments, source);
        let mut d = SolidityDoc::new(source_comments);
        source_pt.visit(&mut d).unwrap();
        // println!("doc {:?}", d);

        for contract in d.contracts {
            println!("contract >>> {:?}", contract);
            // for comment in contract.comments {
            //     for comment in comment.comments() {
            //         println!("comment >>> tag: {} val: {}", comment.tag, comment.value);
            //     }
            // }
            // let content = match comment {
            //     Comment::Block(_, comment) |
            //     Comment::Line(_, comment) |
            //     Comment::DocLine(_, comment) |
            //     Comment::DocBlock(_, comment) => comment,
            // };
        }
    }
}
