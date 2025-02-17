mod renderer;

pub enum LatexComponent {
    Section(Section),
    Subsection(Subsection),
    Text(String),
}

pub struct Section {
    heading: String,
}

pub struct Subsection {
    heading: String,
}


