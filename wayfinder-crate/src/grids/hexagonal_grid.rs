use crate::{
    enums::HexagonalShapes,
    exports::{Image, Walls},
    log,
    nodes::HexagonalNode,
    traits::{AStar, BaseGrid, JsDeserialize, JsHelper, Node},
    types::{Point, Rectangle},
};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct HexagonalGrid {
    pub size: i32,
    pub size_x: f64,
    pub size_y: f64,
    pub columns: bool,
    pub even: bool,
}

impl HexagonalGrid {
    const SQRT1_3: f64 = 0.5773502691896257;
    const SQRT3: f64 = 1.7320508075688772;

    fn cube_round(q: f64, r: f64, s: f64) -> HexagonalNode {
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
}

impl BaseGrid<HexagonalNode> for HexagonalGrid {
    fn get_adjacent_nodes(
        &self,
        node: &HexagonalNode,
        offset: &Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<(HexagonalNode, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| bounds.contains_point(self.get_center_point(neighbor) + *offset))
            .filter(|(neighbor, _cost)| {
                if let Some(explored) = explored {
                    explored.check_pixel(self.get_center_point(neighbor) + *offset)
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                !walls.check_collision(self.get_center_point(node) + *offset, self.get_center_point(neighbor) + *offset)
            })
            .collect()
    }

    fn get_center_point(&self, node: &HexagonalNode) -> Point {
        let q = node.q as f64;
        let r = node.r as f64;

        let mut x;
        let mut y;

        if self.columns {
            x = (HexagonalGrid::SQRT3 / 2.0) * (q + (2.0 / 3.0));
            y = (0.5 * (q + (if self.even { 1.0 } else { 0.0 }))) + r;
        } else {
            y = (HexagonalGrid::SQRT3 / 2.0) * (r + (2.0 / 3.0));
            x = (0.5 * (r + (if self.even { 1.0 } else { 0.0 }))) + q;
        }

        x *= self.size as f64;
        y *= self.size as f64;

        Point { x, y }
    }

    fn get_node(&self, point: Point) -> HexagonalNode {
        let Point { mut x, mut y } = point;

        let q;
        let r;

        x /= self.size as f64;
        y /= self.size as f64;

        if self.columns {
            q = ((2.0 * HexagonalGrid::SQRT1_3) * x) - (2.0 / 3.0);
            r = (-0.5 * (q + (if self.even { 1.0 } else { 0.0 }))) + y;
        } else {
            r = ((2.0 * HexagonalGrid::SQRT1_3) * y) - (2.0 / 3.0);
            q = (-0.5 * (r + (if self.even { 1.0 } else { 0.0 }))) + x;
        }

        HexagonalGrid::cube_round(q, r, 0.0 - q - r)
    }
}

impl AStar for HexagonalGrid {
    fn find_path(
        &self,
        path: Vec<Point>,
        goal: Point,
        offset: Point,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<Image>,
    ) -> Vec<Point> {
        let path: Vec<HexagonalNode> = path.into_iter().map(|point| self.get_node(point - offset)).collect();

        if path.is_empty() {
            return Vec::new();
        }

        let start_node = *path.last().unwrap();
        let end_node = self.get_node(goal - offset);

        if start_node.at_node(&end_node) {
            return Vec::new();
        }

        let result = pathfinding::prelude::astar(
            &start_node,
            |node| self.get_adjacent_nodes(node, &offset, bounds, walls, explored),
            |node| node.get_distance(&end_node),
            |node| node.at_node(&end_node),
        );

        if let Some((nodes, _cost)) = result {
            return nodes.iter().map(|node| self.get_center_point(node) + offset).collect();
        }

        Vec::new()
    }
}
