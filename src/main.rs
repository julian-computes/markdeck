use anyhow::{Result, anyhow};
use markdown::{ParseOptions, mdast::Node, to_mdast};

fn main() -> Result<()> {
    let readme = std::fs::read_to_string("README.md")?;
    let mut mdast =
        to_mdast(readme.as_str(), &ParseOptions::default()).map_err(|e| anyhow!("{}", e))?;

    let mut current_slide_content = vec![];
    let mut slides = vec![];
    let children = mdast.children_mut().ok_or(anyhow!("None"))?;

    for node in children {
        if !current_slide_content.is_empty()
            && let Node::Heading(_) = node
        {
            // Move the current slide into the slides list
            slides.push(std::mem::take(&mut current_slide_content));
        }

        current_slide_content.push(node);
    }

    // Push the last slide
    slides.push(current_slide_content);
    dbg!(slides);

    Ok(())
}
