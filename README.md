# Examples

## Main

```rust
# use anyhow::anyhow;

// Import Veg

use veg::Veg;

// Create a custom type

struct Point {
    x: f32,
    y: f32,
}

// Implement a method that creates a Box of the custom type

impl Point {
    fn new(x: f32, y: f32) -> Box<Point> {
        Box::new(Point { x, y })
    }
}

// Implement the veg::Table::row method to define how to print the custom type

impl veg::Table for Point {
    fn row(&self) -> Vec<String> {
        // Add `$`
        [self.x, self.y].iter().map(|x| format!("${x}$")).collect()

        // Other ideas:

        // - Add 3 decimal places:
        //
        // [self.x, self.y].iter().map(|x| format!("${x:.3}$")).collect()

        // - Do something different for x and y:
        //
        // vec![
        //     format!("${:.1}$", self.x),
        //     format!("${:.4}$", self.y),
        // ]

        // - Just convert to string:
        //
        // [self.x, self.y].iter().map(|x| x.to_string())).collect()

        // ...
    }
}

// Create a Veg via the table method with a header definition

let mut v = Veg::table("$x$|$y$\n-:|-:");

// Add a single point

v.push(Point::new(1.0, 1.0));

// Add a bunch of points

v.append(&mut vec![
    Point::new(2.0, 4.0),
    Point::new(3.0, 9.0),
    Point::new(4.0, 16.0),
]);

// Render as a markdown table

assert_eq!(
    v.markdown().unwrap(),
    "\
| $x$ |  $y$ |
|----:|-----:|
| $1$ |  $1$ |
| $2$ |  $4$ |
| $3$ |  $9$ |
| $4$ | $16$ |
\
    ",
);

// Render as a markdown table with a modified header definition

assert_eq!(
    v.markdown_with(Some("X|Y\n-|-"), None).unwrap(),
    "\
| X   | Y    |
|-----|------|
| $1$ | $1$  |
| $2$ | $4$  |
| $3$ | $9$  |
| $4$ | $16$ |
\
    ",
);

// Render as a markdown table with a modified header definition to increase the
// column widths

assert_eq!(
    v.markdown_with(Some("X|Y\n------|------"), None).unwrap(),
    "\
| X      | Y      |
|--------|--------|
| $1$    | $1$    |
| $2$    | $4$    |
| $3$    | $9$    |
| $4$    | $16$   |
\
    ",
);

// Just render the second column

assert_eq!(
    v.markdown_with(None, Some(&[1])).unwrap(),
    "\
|  $y$ |
|-----:|
|  $1$ |
|  $4$ |
|  $9$ |
| $16$ |
\
    ",
);

// Reorder the columns

assert_eq!(
    v.markdown_with(None, Some(&[1, 0])).unwrap(),
    "\
|  $y$ | $x$ |
|-----:|----:|
|  $1$ | $1$ |
|  $4$ | $2$ |
|  $9$ | $3$ |
| $16$ | $4$ |
\
    ",
);

// Duplicate column `y`

assert_eq!(
    v.markdown_with(None, Some(&[0, 1, 1])).unwrap(),
    "\
| $x$ |  $y$ |  $y$ |
|----:|-----:|-----:|
| $1$ |  $1$ |  $1$ |
| $2$ |  $4$ |  $4$ |
| $3$ |  $9$ |  $9$ |
| $4$ | $16$ | $16$ |
\
    ",
);

// Try to give invalid column indexes

assert_eq!(
    v.markdown_with(None, Some(&[3, 2, 0, 1])).unwrap_err().to_string(),
    "Invalid column indexes: 2, 3",
);
```

### Colored

*See [`tests/colored.rs`] and/or run `cargo test --features colored --test colored -- --nocapture`.*

[`tests/colored.rs`]: tests/colored.rs

# Changelog

* 0.1.0 (2023-12-11): Initial release
    * 0.1.1 (2023-12-11): Add makefile, changelog; fix readme, clippy
* 0.2.0 (2023-12-11): Convert the table function to a method
    * 0.2.1 (2023-12-11): Fix readme
* 0.3.0 (2023-12-12): Enable single column tables; add the `markdown_with`
  method to enable column subsets, reordering, duplicating, and temporary
  headers; add examples to the module doctest
* 0.4.0 (2023-12-16): Add `colored` feature/module enabling terminal colors via
  the [`colored`] crate

