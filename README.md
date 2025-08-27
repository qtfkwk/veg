# Usage

1. Implement [`Table`] trait for your custom type by defining the [`row`] method
   which returns the type as a [`Vec`]`<`[`String`]`>`.
2. Create a [`Veg`] struct with a *header definition* based on a Markdown table
   header.
3. Use the [`Veg`] struct like a [`Vec`] to gather instances of your type.
4. Call one of the following methods to generate a table:

    * [`markdown`]: Markdown table using the initial header definition
    * [`markdown_with`]: Markdown table using a custom header definition and/or
      column indexes

# Example

```rust
// Import Veg
# use anyhow::anyhow;
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
        // Add `$` for inline LaTeX math spans
        [self.x, self.y].iter().map(|x| format!("${x}$")).collect()

        // Other ideas:
        //
        // - Add 3 decimal places:
        //
        // [self.x, self.y].iter().map(|x| format!("${x:.3}$")).collect()
        //
        // - Do something different for x and y:
        //
        // vec![
        //     format!("${:.1}$", self.x),
        //     format!("${:.4}$", self.y),
        // ]
        //
        // - Just convert to string:
        //
        // [self.x, self.y].iter().map(|x| x.to_string())).collect()
        //
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

# Features

## `colored`

The `colored` feature enables the [`veg::colored` module] which provides the
same API, but uses the [`colored`] crate to colorize [`Veg`] tables for printing
to the terminal.

*See [`tests/colored.rs`] and/or run
`cargo test --features colored --test colored -- --nocapture`.*

![](https://github.com/qtfkwk/veg/raw/HEAD/t/colored.png)

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
    * 0.4.1 (2023-12-16): Fix changelog
    * 0.4.2 (2023-12-17): Improve readme, doc, and tests
    * 0.4.3 (2023-12-17): Add doc for colored module; make the colored png
      smaller; improve doc
    * 0.4.4 (2023-12-17): Fix colored module doc
    * 0.4.5 (2023-12-17): Fix doc
    * 0.4.6 (2023-12-18): Fix veg::colored::Veg::table argument type
    * 0.4.7 (2023-12-18): Add `Veg::is_empty` methods
    * 0.4.8 (2024-01-06): Fix empty string find unwrap issue; update
      dependencies
* 0.5.0 (2024-03-10): Fix Unicode text width algorithm; update dependencies
    * 0.5.1 (2024-07-26): Fix makefile; update dependencies
    * 0.5.2 (2024-10-18): Update dependencies
    * 0.5.3 (2024-10-24): Update dependencies
    * 0.5.4 (2024-12-04): Update dependencies; add commit target to makefile
    * 0.5.5 (2025-02-20): Update dependencies
    * 0.5.6 (2025-04-16): Update dependencies
  * 0.6.0 (2025-08-27): Update dependencies; 2024 edition

[`colored`]: https://crates.io/crates/colored

[`tests/colored.rs`]: https://github.com/qtfkwk/veg/blob/main/tests/colored.rs

[`Table`]: https://docs.rs/veg/latest/veg/trait.Table.html
[`row`]: https://docs.rs/veg/latest/veg/colored/trait.Table.html#tymethod.row
[`Veg`]: https://docs.rs/veg/latest/veg/struct.Veg.html
[`markdown`]: https://docs.rs/veg/latest/veg/struct.Veg.html#method.markdown
[`markdown_with`]: https://docs.rs/veg/latest/veg/struct.Veg.html#method.markdown_with
[`veg::colored` module]: https://docs.rs/veg/latest/veg/colored/index.html

[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html

