// Import Veg, ColoredString, and Colorize from veg::colored

use veg::colored::{ColoredString, Colorize, Veg};

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

impl veg::colored::Table for Point {
    fn row(&self) -> Vec<ColoredString> {
        /*
        // Just color both red
        [self.x, self.y]
            .iter()
            .map(|x| x.to_string().red())
            .collect()
        */

        /*
        // Color x green and y red
        vec![self.x.to_string().green(), self.y.to_string().red()]
        */

        // Color negative red, 0 plain, and positive green
        [self.x, self.y]
            .iter()
            .map(|&x| {
                let s = format!("{x:.1}");
                if x < 0.0 {
                    s.red()
                } else if x > 0.0 {
                    s.green()
                } else {
                    ColoredString::from(s)
                }
            })
            .collect()

        // ...
    }
}

#[test]
fn it_works() {
    // Create a Veg via the table method with a header definition

    let mut v = Veg::table(&format!("{}|{}\n-:|-:", "x".cyan(), "y".magenta()));

    // Add a single point

    v.push(Point::new(-2.0, 4.0));

    // Add a bunch of points

    v.append(&mut vec![
        Point::new(-1.5, 3.0),
        Point::new(-1.0, 2.0),
        Point::new(-0.5, 1.0),
        Point::new(0.0, 0.0),
        Point::new(0.5, -1.0),
        Point::new(1.0, -2.0),
        Point::new(1.5, -3.0),
        Point::new(2.0, -4.0),
    ]);

    // Render as a markdown table

    let m = v.markdown().unwrap();
    println!("{m}");
}