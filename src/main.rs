use rand::prelude::*;

#[derive(Debug)]
struct Maze {
  width: usize,
  height: usize,
  east_walls: Vec<bool>,
  south_walls: Vec<bool>,
}

struct MazeIterator<'a> { maze: &'a Maze, n: usize }

#[derive(Debug)]
struct BoundsError;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point { pub x: usize, pub y: usize }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell { pub north: bool, pub east: bool, pub south: bool, pub west: bool }

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir { North, South, East, West }

impl Maze {
  fn new(width: usize, height: usize) -> Result<Maze, BoundsError> {
    if width > 0 && height > 0 {
      Ok(Maze {
        width, height,
        east_walls: vec![true; height * (width - 1)],
        south_walls: vec![true; width * (height - 1)]
      })
    } else {
      Err(BoundsError)
    }
  }

  fn valid(&self, point: Point) -> bool {
    point.x < self.width && point.y < self.height
  }

  fn edge(&self, point: Point) -> bool {
    self.valid(point) && (
      point.x == 0 || point.y == 0 ||
        point.x == self.width - 1 || point.y == self.height - 1)
  }

  fn corner(&self, point: Point) -> bool {
    (point.x == 0 || point.x == self.width - 1) &&
      (point.y == 0 || point.y == self.height - 1)
  }

  fn neighbor(&self, point: Point, dir: Dir) -> Option<Point> {
    let n = point.translate(dir)?;
    if self.valid(n) {
      Some(n)
    } else {
      None
    }
  }

  fn nth_point(&self, n: usize) -> Option<Point> {
    let pt = Point { x: n % self.width, y: n / self.width };
    if self.valid(pt) {
      Some(pt)
    } else {
      None
    }
  }

  fn iter(&self) -> MazeIterator {
    MazeIterator { maze: &self, n: 0 }
  }

  fn passage(&self, point: Point, dir: Dir) -> bool {
    if let Some(_) = self.neighbor(point, dir) {
      match dir {
        Dir::North => !self.south_walls[point.x + self.width * (point.y - 1)],
        Dir::South => !self.south_walls[point.x + self.width * point.y],
        Dir::East => !self.east_walls[point.x + (self.width - 1) * point.y],
        Dir::West => !self.east_walls[point.x - 1 + (self.width - 1) * point.y],
      }
    } else {
      false
    }
  }

  fn cell(&self, point: Point) -> Cell {
    Cell {
      north: self.passage(point, Dir::North),
      south: self.passage(point, Dir::South),
      east: self.passage(point, Dir::East),
      west: self.passage(point, Dir::West)
    }
  }

  fn carve(&mut self, point: Point, dir: Dir) -> Result<(), BoundsError> {
    if let Some(_) = self.neighbor(point, dir) {
      Ok(match dir {
        Dir::North => self.south_walls[point.x + self.width * (point.y - 1)] = false,
        Dir::South => self.south_walls[point.x + self.width * point.y] = false,
        Dir::East => self.east_walls[point.x + (self.width - 1) * point.y] = false,
        Dir::West => self.east_walls[point.x - 1 + (self.width - 1) * point.y] = false
      })
    } else {
      Err(BoundsError)
    }
  }

  fn char(&self, point: Point, dir: Dir) -> &str {
    if self.passage(point, dir) {
      " "
    } else {
      match dir {
        Dir::North | Dir::South => "-",
        Dir::East | Dir::West => "|"
      }
    }
  }

  fn print(&self) {
    // First print a line of norths
    for x in 0..(self.width) {
      print!("+");
      print!("{}", self.char(Point{x, y: 0}, Dir::North))
    }
    println!("+");

    // Then a loop for each row...
    for y in 0..(self.height) {
      // printing the first west, then all easts
      print!("{}", self.char(Point{x: 0, y}, Dir::West));
      for x in 0..(self.width) {
        print!(" ");
        print!("{}", self.char(Point{x, y}, Dir::East));
      }
      println!("");
      // Then all souths
      for x in 0..(self.width) {
        print!("+");
        print!("{}", self.char(Point{x, y}, Dir::South));
      }
      println!("+");
    }
  }

  fn binary_tree(&mut self) {
    for i in (0..(self.width * self.height)).into_iter() {
      if let Some(pt) = self.nth_point(i) {
        let n = self.neighbor(pt, Dir::North).is_some();
        let e = self.neighbor(pt, Dir::East).is_some();

        if n && !e {
          self.carve(pt, Dir::North).expect("");
        } else if e && !n {
          self.carve(pt, Dir::East).expect("");
        } else if n && e {
          if rand::random() {
            self.carve(pt, Dir::North).expect("");
          } else {
            self.carve(pt, Dir::East).expect("");
          }
        }
      }
    }
  }
}

impl<'a> Iterator for MazeIterator<'a> {
  type Item = Point;
  fn next(&mut self) -> Option<Point> {
    let pt = self.maze.nth_point(self.n);
    self.n += 1;
    pt
  }
}

impl Point {
  fn translate(&self, dir: Dir) -> Option<Point> {
    match dir {
      Dir::North => Some(Point { x: self.x, y: self.y.checked_sub(1)? }),
      Dir::South => Some(Point { x: self.x, y: self.y + 1 }),
      Dir::East => Some(Point { x: self.x + 1, y: self.y }),
      Dir::West => Some(Point { x: self.x.checked_sub(1)?, y: self.y })
    }
  }
}

#[test]
fn maze_point_tests() {
  let m = Maze::new(5,5).expect("");
  assert!(m.valid(Point{x: 2, y: 3}));
  assert!(! m.valid(Point{x: 2, y: 20}));
  assert!(m.valid(Point{x: 2, y: 4}));
  assert!(! m.valid(Point{x: 2, y: 5}));

  assert!(m.edge(Point{x: 0, y: 3}));
  assert!(m.edge(Point{x: 2, y: 0}));
  assert!(m.edge(Point{x: 4, y: 2}));
  assert!(m.edge(Point{x: 3, y: 4}));
  assert!(! m.edge(Point{x: 3, y: 2}));

  assert!(m.corner(Point{x: 0, y: 0}));
  assert!(! m.corner(Point{x: 3, y: 2}));
  assert!(m.corner(Point{x: 4, y: 0}));
  assert!(m.corner(Point{x: 0, y: 4}));
}

#[test]
fn point_translate_test() {
  let p = Point { x: 1, y: 1 };

  assert_eq!(p.translate(Dir::North), Some(Point { x: 1, y: 0 }));
  assert_eq!(p.translate(Dir::South), Some(Point { x: 1, y: 2 }));
  assert_eq!(p.translate(Dir::East), Some(Point { x: 2, y: 1 }));
  assert_eq!(p.translate(Dir::West), Some(Point { x: 0, y: 1 }));

  let p2 = Point { x: 0, y: 0 };
  assert_eq!(p2.translate(Dir::North), None);
}

#[test]
fn maze_neighbor_test() {
  let m = Maze::new(5,5).expect("");
  let p = Point { x: 0, y: 0 };

  assert_eq!(m.neighbor(p, Dir::North), None);
  assert_eq!(m.neighbor(p, Dir::East), Some(Point{ x: 1, y: 0 }));
}

#[test]
fn maze_iterator_test() {
  let m = Maze::new(5, 3).expect("");
  assert_eq!(m.iter().count(), 15);
  assert_eq!(m.iter().filter(|p| m.corner(*p)).count(), 4);
  assert_eq!(m.iter().filter(|p| m.edge(*p)).count(), 12)
}

#[test]
fn maze_carve_passage_test() {
  let mut m = Maze::new(2,2).expect("");
  m.carve(Point { x: 0, y: 0 }, Dir::South);
  m.carve(Point { x: 0, y: 1 }, Dir::East);
  assert!(m.passage(Point { x: 0, y: 0 }, Dir::South));
  assert!(m.passage(Point { x: 0, y: 1 }, Dir::North));
  assert!(m.passage(Point { x: 0, y: 1 }, Dir::East));
  assert!(!m.passage(Point { x: 0, y: 0 }, Dir::East));
  assert!(!m.passage(Point { x: 0, y: 0 }, Dir::West));

  let mut m = Maze::new(2,2).expect("");
  m.carve(Point { x: 0, y: 1 }, Dir::North);
  m.carve(Point { x: 1, y: 0 }, Dir::West);
  assert!(m.passage(Point { x: 0, y: 0 }, Dir::South));
  assert!(m.passage(Point { x: 0, y: 1 }, Dir::North));
  assert!(m.passage(Point { x: 0, y: 0 }, Dir::East));
  assert!(m.passage(Point { x: 1, y: 0 }, Dir::West));
}

#[test]
fn maze_cell_test() {
  let mut m = Maze::new(2,2).expect("");
  m.carve(Point { x: 0, y: 1 }, Dir::North);
  m.carve(Point { x: 1, y: 0 }, Dir::West);
  assert_eq!(m.cell(Point { x: 0, y: 0 }),
             Cell { north: false, east: true, south: true, west: false });
  assert_eq!(m.cell(Point { x: 0, y: 1 }),
             Cell { north: true, east: false, south: false, west: false });
  assert_eq!(m.cell(Point { x: 1, y: 1 }),
             Cell { north: false, east: false, south: false, west: false });
}

fn main() {
  let mut m = Maze::new(8,8).expect("");
  m.binary_tree();
  m.print();
}
