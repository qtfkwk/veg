#![doc = include_str!("../t/COLORED.md")]

use super::{HashSet, Result, anyhow, process_header};
use unicode_segmentation::UnicodeSegmentation;

pub use ::colored::{ColoredString, Colorize};

//--------------------------------------------------------------------------------------------------

/**
Trait that must be implemented for your custom type
*/
pub trait Table {
    fn row(&self) -> Vec<ColoredString>;
}

//--------------------------------------------------------------------------------------------------

/**
[`Vec`]-like struct that provides methods for generating tables

*See the [`colored` feature section] in the module documentation for more information.*

[`colored` feature section]: https://docs.rs/veg/latest/veg/index.html#colored
*/
pub struct Veg {
    header: String,
    rows: Vec<Box<dyn Table>>,
}

impl Veg {
    /**
    Create a [`Veg`]
    */
    #[must_use]
    pub fn table(header: &str) -> Veg {
        Veg {
            header: header.to_owned(),
            rows: vec![],
        }
    }

    /**
    Add a single item
    */
    pub fn push(&mut self, item: Box<dyn Table>) {
        self.rows.push(item);
    }

    /**
    Add multiple items
    */
    pub fn append(&mut self, other: &mut Vec<Box<dyn Table>>) {
        self.rows.append(other);
    }

    /**
    Return true if empty
    */
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /**
    Generate a markdown table
    */
    #[allow(clippy::missing_errors_doc)]
    pub fn markdown(&self) -> Result<ColoredString> {
        self.markdown_with(None, None)
    }

    /**
    Generate a markdown table with custom settings

    * Different header definition
    * Column indices

    # Errors

    Returns an error if the number of given columns do not match the columns in the table
    */
    #[allow(clippy::missing_panics_doc)]
    pub fn markdown_with(
        &self,
        header: Option<&String>,
        columns: Option<&[usize]>,
    ) -> Result<ColoredString> {
        // Convert self.{header,rows} into Vec<Vec<ColoredString>>
        let header = if let Some(header) = header {
            header
        } else {
            &self.header
        };
        let mut rows = process_header(header)
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| ColoredString::from(cell.clone()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        rows.append(&mut self.rows.iter().map(|x| x.row()).collect::<Vec<_>>());

        // Process columns subset, reordering, and/or duplication
        let rows = if let Some(columns) = columns {
            let valid = (0..rows[0].len()).collect::<HashSet<_>>();
            let cols = columns.iter().copied().collect::<HashSet<_>>();
            let mut invalid = cols
                .difference(&valid)
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            invalid.sort();
            if !invalid.is_empty() {
                return Err(anyhow!(format!(
                    "Invalid column indexes: {}",
                    invalid.join(", ")
                )));
            }
            rows.iter()
                .map(|row| {
                    columns
                        .iter()
                        .map(|col| row[*col].clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        } else {
            rows
        };

        // Get the columns with right alignment
        let right = rows[1]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if x.ends_with(':') { Some(i) } else { None })
            .collect::<Vec<_>>();

        // Get the maximum width of each column
        let mut width = rows[0].iter().map(|_| 0).collect::<Vec<_>>();
        for row in &rows {
            for (col, cell) in row.iter().enumerate() {
                width[col] = width[col].max(
                    strip_ansi_escapes::strip_str(cell.to_string())
                        .graphemes(true)
                        .count(),
                );
            }
        }

        // Generate the markdown table
        Ok(ColoredString::from(
            rows.iter()
                .enumerate()
                .map(|(i, x)| {
                    if i == 1 {
                        format!(
                            "|{}|\n",
                            x.iter()
                                .enumerate()
                                .map(|(i, x)| format!("{x:->0$}", width[i] + 2))
                                .collect::<Vec<_>>()
                                .join("|"),
                        )
                    } else {
                        format!(
                            "| {} |\n",
                            x.iter()
                                .enumerate()
                                .map(|(i, x)| {
                                    if right.contains(&i) {
                                        let mut s = format!("{x:>0$}", width[i]);
                                        let w = strip_ansi_escapes::strip_str(&s);
                                        s.insert_str(
                                            s.find(&w).unwrap(),
                                            &" ".repeat(width[i] - w.graphemes(true).count()),
                                        );
                                        s
                                    } else {
                                        let mut s = format!("{x:<0$}", width[i]);
                                        let w = strip_ansi_escapes::strip_str(&s);
                                        s.insert_str(
                                            s.find(&w).unwrap_or(0) + w.len(),
                                            &" ".repeat(width[i] - w.graphemes(true).count()),
                                        );
                                        s
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" | "),
                        )
                    }
                })
                .collect::<String>(),
        ))
    }
}
