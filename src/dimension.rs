pub struct Dimension {
    pub name: &'static str,
    pub directory: &'static str,
}

pub const DIMENSIONS: [Dimension; 3] = [
    Dimension {
        name: "Overworld",
        directory: "",
    },
    Dimension {
        name: "The Nether",
        directory: "DIM-1",
    },
    Dimension {
        name: "The End",
        directory: "DIM1",
    },
];
