use crate::{
    enums::HexagonalShapes,
    exports::{Image, TokenShape, Walls},
    log,
    nodes::HexagonalNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper, Node},
    types::{Point, Rectangle},
};
use pathfinding::matrix::directions::N;
use rapier2d::prelude::Polyline;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct HexagonalGrid {
    pub size: i32,
    pub size_x: f32,
    pub size_y: f32,
    pub columns: bool,
    pub even: bool,
}

impl HexagonalGrid {
    const SQRT1_3: f32 = 0.577_350_26_f32;
    const SQRT3: f32 = 1.732_050_8_f32;

    fn cube_round(q: f32, r: f32, s: f32) -> HexagonalNode {
        let mut iq = q.round();
        let mut ir = r.round();
        let mut is = s.round();

        let dq = (iq - q).abs();
        let dr = (ir - r).abs();
        let ds = (is - s).abs();

        if (dq > dr) && (dq > ds) {
            iq = -ir - is;
        } else if dr > ds {
            ir = -iq - is;
        } else {
            is = -iq - ir;
        }

        HexagonalNode { q: iq as i32, r: ir as i32, s: is as i32 }
    }

    fn get_hexagonal_shape(columns: bool, r#type: i32, width: f32, height: f32) -> TokenShape {
        if columns {
            let row_shape = HexagonalGrid::get_hexagonal_shape(false, r#type, width, height);
            let points =
                row_shape.points.clone().into_iter().rev().map(|Point { x, y }| Point { x: y, y: x }).collect();
            let center = Point { x: row_shape.center.y, y: row_shape.center.x };

            return TokenShape::new(center, Point { x: 0.0, y: 0.0 }, points);
        } else if (width == 0.5) && (height == 0.5) {
            return TokenShape::new(
                Point { x: 0.25, y: 0.25 },
                Point { x: 0.0, y: 0.0 },
                vec![
                    Point { x: 0.250, y: 0.000 },
                    Point { x: 0.500, y: 0.125 },
                    Point { x: 0.500, y: 0.375 },
                    Point { x: 0.250, y: 0.500 },
                    Point { x: 0.000, y: 0.375 },
                    Point { x: 0.000, y: 0.125 },
                ],
            );
        } else if (width == 1.0) && (height == 1.0) {
            return TokenShape::new(
                Point { x: 0.5, y: 0.5 },
                Point { x: 0.0, y: 0.0 },
                vec![
                    Point { x: 0.50, y: 0.00 },
                    Point { x: 1.00, y: 0.25 },
                    Point { x: 1.00, y: 0.75 },
                    Point { x: 0.50, y: 1.00 },
                    Point { x: 0.00, y: 0.75 },
                    Point { x: 0.00, y: 0.25 },
                ],
            );
        } else if r#type <= HexagonalShapes::Trapezoid2 as i32 {
            return HexagonalGrid::create_hexagonal_ellipse_or_trapezoid(r#type, width, height);
        } else if r#type <= HexagonalShapes::Rectangle2 as i32 {
            return HexagonalGrid::create_hexagonal_rectangle(r#type, width, height);
        }

        TokenShape::new(
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            vec![
                Point { x: -(width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: -(height / 2.0) },
                Point { x: (width / 2.0), y: (height / 2.0) },
                Point { x: -(width / 2.0), y: (height / 2.0) },
            ],
        )
    }

    fn create_hexagonal_ellipse_or_trapezoid(r#type: i32, width: f32, height: f32) -> TokenShape {
        let center = Point { x: 0.0, y: 0.0 };
        let offset = Point { x: 0.0, y: 0.0 };
        let mut points = Vec::new();

        let width = width as i32;
        let height = height as i32;

        let top;
        let bottom;

        match r#type {
            0 => {
                if height >= 2 * width {
                    return TokenShape::new(center, offset, points);
                }

                top = height / 2;
                bottom = (height - 1) / 2;
            }
            1 => {
                if height >= 2 * width {
                    return TokenShape::new(center, offset, points);
                }

                top = (height - 1) / 2;
                bottom = height / 2;
            }
            2 => {
                if height > width {
                    return TokenShape::new(center, offset, points);
                }

                top = height - 1;
                bottom = 0;
            }
            3 => {
                if height > width {
                    return TokenShape::new(center, offset, points);
                }

                top = 0;
                bottom = height - 1;
            }
            _ => {
                return TokenShape::new(center, offset, points);
            }
        }

        let mut x = 0.5 * bottom as f32;
        let mut y = 0.25;

        for _ in (0..(width - bottom)).rev() {
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
        }

        points.push(Point { x, y });

        for _ in (0..(bottom)).rev() {
            y += 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        y += 0.5;

        for _ in (0..(top)).rev() {
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            y += 0.5;
        }

        for _ in (0..(width - top)).rev() {
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
        }

        points.push(Point { x, y });

        for _ in (0..(top)).rev() {
            y -= 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        y -= 0.5;

        for _ in (0..(bottom)).rev() {
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            y -= 0.5;
        }

        let center = TokenShape::centroid(&points);
        TokenShape::new(center, offset, points)
    }

    fn create_hexagonal_rectangle(r#type: i32, width: f32, height: f32) -> TokenShape {
        let center = Point { x: 0.0, y: 0.0 };
        let offset = Point { x: 0.0, y: 0.0 };
        let mut points = Vec::new();

        let width = width as i32;
        let height = height as i32;

        if width < 1 || height < 1 {
            return TokenShape::new(center, offset, points);
        }

        if (width == 1) && (height > 1) {
            return TokenShape::new(center, offset, points);
        }

        let even = (r#type == 4) || (height == 1);
        let mut x = if even { 0.0 } else { 0.5 };
        let mut y = 0.25;
        points.push(Point { x, y });

        while x + 1.0 <= width as f32 {
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        if x != width as f32 {
            y += 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        while y + 1.5 <= 0.75 * height as f32 {
            y += 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            y += 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        if y + 0.75 < 0.75 * height as f32 {
            y += 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        y += 0.5;
        points.push(Point { x, y });

        while x - 1.0 >= 0.0 {
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        if x != 0.0 {
            y -= 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        while y - 1.5 > 0.0 {
            y -= 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            y -= 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        if y - 0.75 > 0.0 {
            y -= 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        let center = Point {
            x: (width as f32) / 2.0,
            y: ((0.75 * f32::floor(height as f32)) + (0.5 * ((height as f32) % 1.0)) + 0.25) / 2.0,
        };
        TokenShape::new(center, offset, points)
    }
}

impl BaseGrid<HexagonalNode> for HexagonalGrid {
    fn get_adjacent_nodes(
        &self,
        node: &HexagonalNode,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        shape: &TokenShape,
    ) -> Vec<(HexagonalNode, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| {
                if let Some(bounds) = bounds {
                    bounds.contains(self.get_center_point(neighbor) + shape.offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if let Some(explored) = explored {
                    explored.check_pixel(self.get_center_point(neighbor) + shape.offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if let Some(walls) = walls {
                    !walls.check_collision(
                        self.get_center_point(node) + shape.offset,
                        self.get_center_point(neighbor) + shape.offset,
                        &shape.polyline,
                    )
                } else {
                    true
                }
            })
            .collect()
    }

    fn get_center_point(&self, node: &HexagonalNode) -> Point {
        let q = node.q as f32;
        let r = node.r as f32;

        let mut x;
        let mut y;

        if self.columns {
            x = (HexagonalGrid::SQRT3 / 2.0) * (q + (2.0 / 3.0));
            y = (0.5 * (q + (if self.even { 1.0 } else { 0.0 }))) + r;
        } else {
            y = (HexagonalGrid::SQRT3 / 2.0) * (r + (2.0 / 3.0));
            x = (0.5 * (r + (if self.even { 1.0 } else { 0.0 }))) + q;
        }

        x *= self.size as f32;
        y *= self.size as f32;

        Point { x, y }
    }

    fn get_node(&self, point: Point) -> HexagonalNode {
        let Point { mut x, mut y } = point;

        let q;
        let r;

        x /= self.size as f32;
        y /= self.size as f32;

        if self.columns {
            q = ((2.0 * HexagonalGrid::SQRT1_3) * x) - (2.0 / 3.0);
            r = (-0.5 * (q + (if self.even { 1.0 } else { 0.0 }))) + y;
        } else {
            r = ((2.0 * HexagonalGrid::SQRT1_3) * y) - (2.0 / 3.0);
            q = (-0.5 * (r + (if self.even { 1.0 } else { 0.0 }))) + x;
        }

        HexagonalGrid::cube_round(q, r, 0.0 - q - r)
    }

    fn get_token_shape(&self, token: JsValue) -> TokenShape {
        let width: f32;
        let height: f32;
        let hexagonal_shape: i32;

        if token.is_object() {
            width = JsDeserialize::from_value(token.get("document").get("width"));
            height = JsDeserialize::from_value(token.get("document").get("height"));
            hexagonal_shape = JsDeserialize::from_value(token.get("document").get("hexagonalShape"));
        } else {
            width = 1.0;
            height = 1.0;
            hexagonal_shape = 0;
        }

        let offset;

        if width % 2.0 != 1.0 {
            let center: Point = JsDeserialize::from_value(token.get("center"));

            if self.columns {
                let offset_by = Point { x: self.size_x / 4.0, y: self.size_y / 2.0 };
                let top_left = self.get_center_point(&self.get_node(center - offset_by));
                offset = center - top_left;
            } else {
                let offset_by = Point { x: self.size_x / 2.0, y: self.size_y / 4.0 };
                let top_left = self.get_center_point(&self.get_node(center - offset_by));
                offset = center - top_left;
            }
        } else {
            offset = Point { x: 0.0, y: 0.0 }
        };

        let token_shape = HexagonalGrid::get_hexagonal_shape(self.columns, hexagonal_shape, width, height);

        if !token_shape.points.is_empty() {
            let grid_size = Point { x: self.size_x, y: self.size_y };
            let center = token_shape.center * grid_size;
            let points = token_shape.points.into_iter().map(|point| (point * grid_size) - center).collect();

            return TokenShape::new(Point { x: 0.0, y: 0.0 }, offset, points);
        }

        TokenShape::new(
            Point { x: 0.0, y: 0.0 },
            offset,
            vec![
                Point { x: -((width * self.size_x) / 2.0), y: -((height * self.size_y) / 2.0) },
                Point { x: ((width * self.size_x) / 2.0), y: -((height * self.size_y) / 2.0) },
                Point { x: ((width * self.size_x) / 2.0), y: ((height * self.size_y) / 2.0) },
                Point { x: -((width * self.size_x) / 2.0), y: ((height * self.size_y) / 2.0) },
            ],
        )
    }
}

impl AStar for HexagonalGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        bounds: &Option<Rectangle>,
        walls: &Option<Walls>,
        explored: &Option<Image>,
        token_shape: &Option<TokenShape>,
    ) -> Vec<Point> {
        let shape = if let Some(token_shape) = token_shape {
            token_shape.shrink(Point { x: self.size as f32, y: self.size as f32 } * Point { x: 0.45, y: 0.45 })
        } else {
            self.get_token_shape(JsValue::NULL)
                .shrink(Point { x: self.size as f32, y: self.size as f32 } * Point { x: 0.45, y: 0.45 })
        };

        let path: Vec<HexagonalNode> = path.into_iter().map(|point| self.get_node(point - shape.offset)).collect();

        if path.is_empty() {
            return Vec::new();
        }

        let start_node = *path.last().unwrap();
        let end_node = self.get_node(goal - shape.offset);

        if start_node.at_node(&end_node) {
            return Vec::new();
        }

        let result = pathfinding::prelude::astar(
            &start_node,
            |node| self.get_adjacent_nodes(node, bounds, walls, explored, &shape),
            |node| node.get_distance(&end_node),
            |node| node.at_node(&end_node),
        );

        if let Some((nodes, _cost)) = result {
            return nodes.iter().map(|node| self.get_center_point(node) + shape.offset).collect();
        }

        Vec::new()
    }
}
