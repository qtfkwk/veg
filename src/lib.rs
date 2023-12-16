#![doc = include_str!("../README.md")]

//--------------------------------------------------------------------------------------------------

use anyhow::{anyhow, Result};
use std::collections::HashSet;

//--------------------------------------------------------------------------------------------------

#[cfg(feature = "colored")]
pub mod colored;

//--------------------------------------------------------------------------------------------------

pub trait Table {
    fn row(&self) -> Vec<String>;
}

//--------------------------------------------------------------------------------------------------

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

    pub fn markdown(&self) -> Result<String> {
        self.markdown_with(None, None)
    }

    pub fn markdown_with(&self, header: Option<&str>, columns: Option<&[usize]>) -> Result<String> {
        // Convert self.{header,rows} into Vec<Vec<String>>
        let header = if let Some(header) = header {
            header
        } else {
            &self.header
        };
        let mut rows = process_header(header);
        rows.append(&mut self.rows.iter().map(|x| x.row()).collect::<Vec<_>>());

        // Process columns subset, reordering, and/or duplication
        let rows = if let Some(columns) = columns {
            let valid = (0..rows[0].len()).collect::<HashSet<_>>();
            let cols = columns.iter().cloned().collect::<HashSet<_>>();
            let mut invalid = cols
                .difference(&valid)
                .map(|x| x.to_string())
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
        for row in rows.iter() {
            for (col, cell) in row.iter().enumerate() {
                width[col] = width[col].max(cell.chars().collect::<Vec<_>>().len());
            }
        }

        // Generate the markdown table
        Ok(rows
            .iter()
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
                                    format!("{x:>0$}", width[i])
                                } else {
                                    format!("{x:<0$}", width[i])
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" | "),
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(""))
    }
}

//--------------------------------------------------------------------------------------------------

fn process_header(s: &str) -> Vec<Vec<String>> {
    s.split('\n')
        .map(|x| {
            x.split('|')
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
