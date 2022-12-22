// https://docs.rs/latex/latest/latex/
extern crate failure;
use failure::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use latex::{DocumentClass, Element, Document, Section, Align};

pub fn write_latex() {
    println!("test1");
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Fancy Document");
    doc.preamble.author("Michael-F-Bryan");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");
    section_1.push("Here is some text which will be put in paragraph 1.")
            .push("And here is some more text for paragraph 2.");
    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    println!("test2");
    section_2.push("More text...")
            .push(Align::from("y &= mx + c"));

    doc.push(section_2);

    let rendered = latex::print(&doc).unwrap();
    let mut f = File::create("./report.txt").unwrap();
    write!(f, "{}", rendered).unwrap();

    // Then call latexmk on it
    //let exit_status = Command::new("latexmk").arg("./report.tex").status().unwrap();
    //println!("{}", exit_status);
    //assert!(exit_status.success());
}