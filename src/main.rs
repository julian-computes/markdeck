use markdown::{ParseOptions, mdast::Node, to_mdast};

fn main() {
    let readme = std::fs::read_to_string("README.md").unwrap();
    let mut mdast = to_mdast(readme.as_str(), &ParseOptions::default()).unwrap();

    let mut current_slide_content = vec![];
    let mut slides = vec![];
    let children = mdast.children_mut().unwrap();

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
}
