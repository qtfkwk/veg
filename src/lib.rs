#![doc = include_str!("../README.md")]

pub trait Table {
    fn row(&self) -> Vec<String>;
}

pub struct Veg {
    header: String,
    rows: Vec<Box<dyn Table>>,
}

impl Veg {
    pub fn table(header: &str) -> Veg {
        Veg {
            header: header.into(),
            rows: vec![],
        }
    }

    pub fn push(&mut self, item: Box<dyn Table>) {
        self.rows.push(item);
    }

    pub fn append(&mut self, other: &mut Vec<Box<dyn Table>>) {
        self.rows.append(other);
    }

    pub fn markdown(&self) -> String {
        let mut r = header(&self.header);
        r.append(&mut self.rows.iter().map(|x| x.row()).collect::<Vec<_>>());

        let right = r[1]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if x.ends_with(':') { Some(i) } else { None })
            .collect::<Vec<_>>();

        let mut width = r[0].iter().map(|_| 0).collect::<Vec<_>>();
        for row in r.iter() {
            for (col, cell) in row.iter().enumerate() {
                width[col] = width[col].max(cell.chars().collect::<Vec<_>>().len() + 1);
            }
        }

        let mut r = r
            .iter()
            .enumerate()
            .map(|(i, x)| {
                if i == 1 {
                    x.iter()
                        .map(|x| format!("--{x}"))
                        .collect::<Vec<_>>()
                        .join("|")
                } else {
                    x.iter()
                        .enumerate()
                        .map(|(i, x)| {
                            if right.contains(&i) {
                                format!("{x:>0$}", width[i])
                            } else {
                                format!("{x:<0$}", width[i])
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" |")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        r.push('\n');

        r
    }
}

fn header(s: &str) -> Vec<Vec<String>> {
    s.split('\n')
        .map(|x| {
            x.split('|')
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
