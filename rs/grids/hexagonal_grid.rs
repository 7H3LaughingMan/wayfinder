use pathfinding::{grid::Grid, undirected::kruskal};

use crate::{
    enums::TokenShapeType,
    exports::Walls,
    log,
    modules::geometry,
    nodes::HexagonalNode,
    traits::{AStar, BaseGrid, Node},
    types::{
        ElevatedPoint, GridMeasurePathResult, GridOffset2D, GridOffset3D, HexagonalGridCube2D, HexagonalGridCube3D,
        Point, Rectangle, TokenDocument, TokenHexagonalShapeData, TokenMovementWaypoint,
    },
};
use std::{collections::HashMap, ops::RangeInclusive, vec};

#[derive(Debug)]
pub struct HexagonalGrid {
    pub size: i32,
    pub distance: f64,
    pub size_x: f64,
    pub size_y: f64,
    pub columns: bool,
    pub even: bool,
}

impl HexagonalGrid {
    const SQRT1_3: f64 = 0.5773502691896257;
    const SQRT3: f64 = 1.7320508075688772;

    pub fn cube_round(q: f64, r: f64, s: f64, k: f64) -> HexagonalGridCube3D {
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

        HexagonalGridCube3D { q: iq as i32, r: ir as i32, s: is as i32, k: (k + 1e-8).floor() as i32 }
    }

    pub fn point_to_cube(&self, ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint) -> HexagonalGridCube3D {
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

        HexagonalGrid::cube_round(q, r, 0.0 - q - r, elevation / self.distance)
    }

    pub fn cube_to_point(&self, HexagonalGridCube3D { q, r, s: _, k }: HexagonalGridCube3D) -> ElevatedPoint {
        let x;
        let y;

        if self.columns {
            x = (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * q as f64) + 2.0) * self.size as f64);
            y = ((0.5 * (q as f64 + if self.even { 1.0 } else { 0.0 })) + r as f64) * self.size as f64;
        } else {
            y = (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * r as f64) + 2.0) * self.size as f64);
            x = ((0.5 * (r as f64 + if self.even { 1.0 } else { 0.0 })) + q as f64) * self.size as f64;
        }

        ElevatedPoint { x, y, elevation: (k as f64 + 0.5) * self.distance }
    }

    pub fn offset_to_cube(&self, GridOffset3D { i, j, k }: GridOffset3D) -> HexagonalGridCube3D {
        let q: i32;
        let r: i32;
        if self.columns {
            q = j;
            r = i - ((j + ((if self.even { 1 } else { -1 }) * (j & 1))) >> 1);
        } else {
            q = j - ((i + ((if self.even { 1 } else { -1 }) * (i & 1))) >> 1);
            r = i;
        }
        HexagonalGridCube3D { q: q, r: r, s: 0 - q - r, k }
    }

    pub fn cube_to_offset(&self, HexagonalGridCube3D { q, r, s: _, k }: HexagonalGridCube3D) -> GridOffset3D {
        if self.columns {
            GridOffset3D { i: q, j: r + ((q + ((if self.even { 1 } else { -1 }) * (q & 1))) >> 1), k }
        } else {
            GridOffset3D { i: r, j: q + ((r + ((if self.even { 1 } else { -1 }) * (r & 1))) >> 1), k }
        }
    }

    pub fn cube_distance(a: HexagonalGridCube2D, b: HexagonalGridCube2D) -> u32 {
        let dq = a.q - b.q;
        let dr = a.r - b.r;

        ((dq.abs() + dr.abs() + (dq + dr).abs()) / 2) as u32
    }

    pub fn get_hexagonal_shape(
        width: f64,
        height: f64,
        shape: TokenShapeType,
        columns: bool,
    ) -> Option<TokenHexagonalShapeData> {
        if (width * 2.0).fract() != 0.0 || (height * 2.0).fract() != 0.0 {
            return None;
        }

        if columns {
            if let Some(row_data) = HexagonalGrid::get_hexagonal_shape(height, width, shape, false) {
                let mut even_offsets = Vec::<GridOffset2D>::new();
                let mut odd_offsets = Vec::<GridOffset2D>::new();

                for GridOffset2D { i, j } in row_data.even_offsets {
                    even_offsets.push(GridOffset2D { i: j, j: i });
                }

                for GridOffset2D { i, j } in row_data.odd_offsets {
                    odd_offsets.push(GridOffset2D { i: j, j: i });
                }

                even_offsets.sort();
                odd_offsets.sort();

                let mut points = Vec::<Point>::new();

                for Point { x, y } in row_data.points.iter().rev() {
                    points.push(Point { x: *y, y: *x });
                }

                return Some(TokenHexagonalShapeData {
                    even_offsets,
                    odd_offsets,
                    points,
                    center: Point { x: row_data.center.y, y: row_data.center.x },
                    anchor: Point { x: row_data.anchor.y, y: row_data.anchor.x },
                    width: row_data.width,
                    height: row_data.height,
                });
            } else {
                return None;
            }
        } else if width == 0.5 && height == 0.5 {
            return Some(TokenHexagonalShapeData {
                even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                points: vec![
                    Point { x: 0.250, y: 0.000 },
                    Point { x: 0.500, y: 0.125 },
                    Point { x: 0.500, y: 0.375 },
                    Point { x: 0.250, y: 0.500 },
                    Point { x: 0.000, y: 0.375 },
                    Point { x: 0.000, y: 0.125 },
                ],
                center: Point { x: 0.25, y: 0.25 },
                anchor: Point { x: 0.25, y: 0.25 },
                width: width,
                height: height,
            });
        } else if width == 1.0 && height == 1.0 {
            return Some(TokenHexagonalShapeData {
                even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                points: vec![
                    Point { x: 0.50, y: 0.00 },
                    Point { x: 1.00, y: 0.25 },
                    Point { x: 1.00, y: 0.75 },
                    Point { x: 0.50, y: 1.00 },
                    Point { x: 0.00, y: 0.75 },
                    Point { x: 0.00, y: 0.25 },
                ],
                center: Point { x: 0.5, y: 0.5 },
                anchor: Point { x: 0.5, y: 0.5 },
                width: width,
                height: height,
            });
        } else if shape <= TokenShapeType::Trapezoid2 {
            return HexagonalGrid::create_hexagonal_ellipse_or_trapezoid(width, height, shape);
        } else if shape <= TokenShapeType::Rectangle2 {
            return HexagonalGrid::create_hexagonal_rectangle(width, height, shape);
        }

        return None;
    }

    fn create_hexagonal_ellipse_or_trapezoid(
        width: f64,
        height: f64,
        shape: TokenShapeType,
    ) -> Option<TokenHexagonalShapeData> {
        if width.fract() != 0.0 || height.fract() != 0.0 {
            return None;
        }

        let mut points = Vec::<Point>::new();
        let top;
        let bottom;

        match shape {
            TokenShapeType::Ellipse1 => {
                if height >= 2.0 * width {
                    return None;
                }
                top = (height / 2.0).floor() as i32;
                bottom = ((height - 1.0) / 2.0).floor() as i32;
            }
            TokenShapeType::Ellipse2 => {
                if height >= 2.0 * width {
                    return None;
                }
                top = ((height - 1.0) / 2.0).floor() as i32;
                bottom = (height / 2.0).floor() as i32;
            }
            TokenShapeType::Trapezoid1 => {
                if height > width {
                    return None;
                }
                top = (height - 1.0) as i32;
                bottom = 0;
            }
            TokenShapeType::Trapezoid2 => {
                if height > width {
                    return None;
                }
                top = 0;
                bottom = (height - 1.0) as i32;
            }
            _ => {
                return None;
            }
        }

        let mut even_offsets = Vec::<GridOffset2D>::new();
        let mut odd_offsets = Vec::<GridOffset2D>::new();

        for i in (1..=bottom).rev() {
            for j in 0..(width as i32 - i) {
                even_offsets.push(GridOffset2D { i: bottom - i, j: j + (((bottom & 1) + i + 1) >> 1) });
                odd_offsets.push(GridOffset2D { i: bottom - i, j: j + (((bottom & 1) + i) >> 1) });
            }
        }

        for i in 0..=top {
            for j in 0..(width as i32 - i) {
                even_offsets.push(GridOffset2D { i: bottom + i, j: j + (((bottom & 1) + i + 1) >> 1) });
                odd_offsets.push(GridOffset2D { i: bottom + i, j: j + (((bottom & 1) + i) >> 1) });
            }
        }

        let mut x = 0.5 * bottom as f64;
        let mut y = 0.25;

        for _k in 0..(width as i32 - bottom) {
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
        }

        points.push(Point { x, y });

        for _k in 0..bottom {
            y += 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        y += 0.5;

        for _k in 0..top {
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            y += 0.5;
        }

        for _k in 0..(width as i32 - top) {
            points.push(Point { x, y });
            x -= 0.5;
            y += 0.25;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
        }

        points.push(Point { x, y });

        for _k in 0..top {
            y -= 0.5;
            points.push(Point { x, y });
            x -= 0.5;
            y -= 0.25;
            points.push(Point { x, y });
        }

        y -= 0.5;

        for _k in 0..bottom {
            points.push(Point { x, y });
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            y -= 0.5;
        }

        let center = geometry::polygon_centroid(&points);
        return Some(TokenHexagonalShapeData {
            even_offsets,
            odd_offsets,
            points,
            center,
            anchor: if bottom % 2 != 0 { Point { x: 0.0, y: 0.5 } } else { Point { x: 0.5, y: 0.5 } },
            width,
            height,
        });
    }

    fn create_hexagonal_rectangle(width: f64, height: f64, shape: TokenShapeType) -> Option<TokenHexagonalShapeData> {
        if width < 1.0 || height.fract() != 0.0 {
            return None;
        }

        if width == 1.0 && height > 1.0 {
            return None;
        }

        if width.fract() != 0.0 && height == 1.0 {
            return None;
        }

        let even = (shape == TokenShapeType::Rectangle1) || (height == 1.0);
        let mut even_offsets = Vec::<GridOffset2D>::new();
        let mut odd_offsets = Vec::<GridOffset2D>::new();

        for i in 0..(height as i32) {
            let j0 = if even { 0 } else { (i + 1) & 1 };
            let j1 = (width + ((i & 1) as f64 * 0.5)).floor() as i32 - if even { i & 1 } else { 0 };
            for j in j0..j1 {
                even_offsets.push(GridOffset2D { i, j: j + (i & 1) });
                odd_offsets.push(GridOffset2D { i, j });
            }
        }

        let mut x = if even { 0.0 } else { 0.5 };
        let mut y = 0.25;
        let mut points = vec![Point { x, y }];

        while x + 1.0 <= width {
            x += 0.5;
            y -= 0.25;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        if x != width {
            y += 0.5;
            points.push(Point { x, y });
            x += 0.5;
            y += 0.25;
            points.push(Point { x, y });
        }

        while y + 1.5 <= 0.75 * height {
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

        if y + 0.75 < 0.75 * height {
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

        return Some(TokenHexagonalShapeData {
            even_offsets,
            odd_offsets,
            points,
            center: Point { x: width / 2.0, y: ((0.75 * height.floor()) + (0.5 * (height % 1.0)) + 0.25) / 2.0 },
            anchor: if even { Point { x: 0.5, y: 0.5 } } else { Point { x: 0.0, y: 0.5 } },
            width,
            height,
        });
    }
}

impl BaseGrid<HexagonalNode, TokenHexagonalShapeData> for HexagonalGrid {
    fn convert_node_to_offset(&self, HexagonalNode { q, r, s, k, d: _ }: HexagonalNode) -> GridOffset3D {
        self.cube_to_offset(HexagonalGridCube3D { q, r, s, k })
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> HexagonalNode {
        let HexagonalGridCube3D { q, r, s, k } = self.offset_to_cube(GridOffset3D { i, j, k });
        HexagonalNode { q, r, s, k, d: false }
    }

    fn get_adjacent_nodes(
        &self,
        node: &HexagonalNode,
        token_shape: &TokenHexagonalShapeData,
        elevation_range: &RangeInclusive<i32>,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<HashMap<GridOffset2D, bool>>,
    ) -> Vec<(HexagonalNode, u32)> {
        node.get_neighbors()
            .into_iter()
            .filter(|(neighbor, _cost)| elevation_range.contains(&neighbor.k))
            .filter(|(neighbor, _cost)| bounds.contains_point(self.get_node_center_point(neighbor).into()))
            .filter(|(neighbor, _cost)| {
                if let Some(hashmap) = explored {
                    if let Some(value) = hashmap.get(&self.convert_node_to_offset(*neighbor).into()) {
                        *value
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .filter(|(neighbor, _cost)| {
                if token_shape.width > 1.0 || token_shape.height > 1.0 {
                    if walls.check_point(
                        self.get_token_center_point(self.get_node_top_left_point(neighbor), token_shape).into(),
                    ) {
                        return false;
                    }
                }

                !walls.check_collisions(
                    self.get_occupied_grid_space_offsets(self.convert_node_to_offset(*node), token_shape)
                        .iter()
                        .map(|offset| self.get_offset_center_point(*offset).into())
                        .zip(
                            self.get_occupied_grid_space_offsets(self.convert_node_to_offset(*neighbor), token_shape)
                                .iter()
                                .map(|offset| self.get_offset_center_point(*offset).into()),
                        )
                        .collect(),
                )
            })
            .collect()
    }

    fn get_node(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShapeData { even_offsets: _, odd_offsets: _, points: _, center: _, anchor, width: _, height: _ }: &TokenHexagonalShapeData,
    ) -> HexagonalNode {
        self.convert_offset_to_node(self.cube_to_offset(self.point_to_cube(ElevatedPoint {
            x: x + (self.size_x * anchor.x),
            y: y + (self.size_y * anchor.y),
            elevation,
        })))
    }

    fn get_node_center_point(&self, HexagonalNode { q, r, s: _, k, d: _ }: &HexagonalNode) -> ElevatedPoint {
        if self.columns {
            ElevatedPoint {
                x: (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * *q as f64) + 2.0) * self.size as f64),
                y: ((0.5 * (*q as f64 + if self.even { 1.0 } else { 0.0 })) + *r as f64) * self.size as f64,
                elevation: (*k as f64 + 0.5) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * *r as f64) + 2.0) * self.size as f64),
                x: ((0.5 * (*r as f64 + if self.even { 1.0 } else { 0.0 })) + *q as f64) * self.size as f64,
                elevation: (*k as f64 + 0.5) * self.distance,
            }
        }
    }

    fn get_node_top_left_point(&self, HexagonalNode { q, r, s: _, k, d: _ }: &HexagonalNode) -> ElevatedPoint {
        if self.columns {
            ElevatedPoint {
                x: (HexagonalGrid::SQRT3 / 2.0) * (*q as f64 * self.size as f64),
                y: ((0.5 * (*q as f64 - if self.even { 0.0 } else { 1.0 })) + *r as f64) * self.size as f64,
                elevation: *k as f64 * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (HexagonalGrid::SQRT3 / 2.0) * (*r as f64 * self.size as f64),
                x: ((0.5 * (*r as f64 - if self.even { 0.0 } else { 1.0 })) + *q as f64) * self.size as f64,
                elevation: *k as f64 * self.distance,
            }
        }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        GridOffset3D { i, j, k }: GridOffset3D,
        TokenHexagonalShapeData { even_offsets, odd_offsets, points: _, center: _, anchor: _, width: _, height: _ }: &TokenHexagonalShapeData,
    ) -> Vec<GridOffset3D> {
        (if ((if self.columns { j } else { i }) % 2 == 0) == self.even { even_offsets } else { odd_offsets })
            .iter()
            .map(|offset| GridOffset3D { i: i + offset.i, j: j + offset.j, k })
            .collect()
    }

    fn get_offset(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShapeData { even_offsets: _, odd_offsets: _, points: _, center: _, anchor, width: _, height: _ }: &TokenHexagonalShapeData,
    ) -> GridOffset3D {
        self.cube_to_offset(self.point_to_cube(ElevatedPoint {
            x: x + (self.size_x * anchor.x),
            y: y + (self.size_y * anchor.y),
            elevation,
        }))
    }

    fn get_offset_center_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        let HexagonalGridCube3D { q, r, s: _, k } = self.offset_to_cube(offset);

        if self.columns {
            ElevatedPoint {
                x: (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * q as f64) + 2.0) * self.size as f64),
                y: ((0.5 * (q as f64 + if self.even { 1.0 } else { 0.0 })) + r as f64) * self.size as f64,
                elevation: (k as f64 + 0.5) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (0.5 * HexagonalGrid::SQRT1_3) * (((3.0 * r as f64) + 2.0) * self.size as f64),
                x: ((0.5 * (r as f64 + if self.even { 1.0 } else { 0.0 })) + q as f64) * self.size as f64,
                elevation: (k as f64 + 0.5) * self.distance,
            }
        }
    }

    fn get_offset_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        let HexagonalGridCube3D { q, r, s: _, k } = self.offset_to_cube(offset);

        if self.columns {
            ElevatedPoint {
                x: (HexagonalGrid::SQRT3 / 2.0) * (q as f64 * self.size as f64),
                y: ((0.5 * (q as f64 - if self.even { 0.0 } else { 1.0 })) + r as f64) * self.size as f64,
                elevation: k as f64 * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (HexagonalGrid::SQRT3 / 2.0) * (r as f64 * self.size as f64),
                x: ((0.5 * (r as f64 - if self.even { 0.0 } else { 1.0 })) + q as f64) * self.size as f64,
                elevation: k as f64 * self.distance,
            }
        }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShapeData { even_offsets: _, odd_offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenHexagonalShapeData,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + (center.x * self.size_x), y: y + (center.y * self.size_y), elevation }
    }

    fn get_token_shape(&self, mut width: f64, mut height: f64, shape: TokenShapeType) -> TokenHexagonalShapeData {
        width = (width * 2.0).round() / 2.0;
        height = (height * 2.0).round() / 2.0;

        if let Some(token_shape) = HexagonalGrid::get_hexagonal_shape(width, height, shape, self.columns) {
            token_shape
        } else {
            if self.columns {
                height += 0.5;
                width = width.round();
                if width == 1.0 {
                    height = height.floor();
                } else if height == 1.0 {
                    height += 0.5;
                }
            } else {
                width += 0.5;
                height = height.round();
                if height == 1.0 {
                    width = width.floor();
                } else if width == 1.0 {
                    width += 0.5;
                }
            }

            if let Some(TokenHexagonalShapeData { even_offsets, odd_offsets, points, center, anchor, width, height }) =
                HexagonalGrid::get_hexagonal_shape(width, height, TokenShapeType::Rectangle1, self.columns)
            {
                TokenHexagonalShapeData {
                    even_offsets,
                    odd_offsets,
                    points,
                    center,
                    anchor: Point { x: anchor.x - 0.25, y: anchor.y - 0.25 },
                    width,
                    height,
                }
            } else {
                TokenHexagonalShapeData {
                    even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                    odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                    points: vec![
                        Point { x: 0.50, y: 0.00 },
                        Point { x: 1.00, y: 0.25 },
                        Point { x: 1.00, y: 0.75 },
                        Point { x: 0.50, y: 1.00 },
                        Point { x: 0.00, y: 0.75 },
                        Point { x: 0.00, y: 0.25 },
                    ],
                    center: Point { x: 0.5, y: 0.5 },
                    anchor: Point { x: 0.5, y: 0.5 },
                    width: 1.0,
                    height: 1.0,
                }
            }
        }
    }
}

impl AStar for HexagonalGrid {
    fn find_path(
        &self,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<HashMap<GridOffset2D, bool>>,
        grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint> {
        if waypoints.len() <= 1 {
            return waypoints;
        }

        let mut token_shape = self.get_token_shape(token.width, token.height, token.shape);
        let mut start_waypoint = &waypoints[0];
        let mut start_node = self.get_node(start_waypoint.create_elevated_point(), &token_shape);
        let mut path = vec![start_waypoint.clone()];

        start_node.d = grid_measure_path_result.diagonals % 2 != 0;

        for end_waypoint in &waypoints[1..] {
            let end_node = self.get_node(end_waypoint.create_elevated_point(), &token_shape);
            let elevation_range = i32::min(start_node.k, end_node.k)..=(i32::max(start_node.k, end_node.k));

            if let Some((neighbor, _cost)) = start_node
                .get_neighbors()
                .iter()
                .filter(|(neighbor, _cost)| {
                    if token_shape.width > 1.0 || token_shape.height > 1.0 {
                        if walls.check_point(
                            self.get_token_center_point(self.get_node_top_left_point(neighbor), &token_shape).into(),
                        ) {
                            return false;
                        }
                    }

                    !walls.check_collisions(
                        self.get_occupied_grid_space_offsets(self.convert_node_to_offset(start_node), &token_shape)
                            .iter()
                            .map(|offset| self.get_offset_center_point(*offset).into())
                            .zip(
                                self.get_occupied_grid_space_offsets(
                                    self.convert_node_to_offset(*neighbor),
                                    &token_shape,
                                )
                                .iter()
                                .map(|offset| self.get_offset_center_point(*offset).into()),
                            )
                            .collect(),
                    )
                })
                .find(|(neighbor, _cost)| neighbor.at_node(&end_node))
            {
                path.push(end_waypoint.clone());

                token_shape = self.get_token_shape(end_waypoint.width, end_waypoint.height, end_waypoint.shape);
                start_waypoint = end_waypoint;
                start_node = *neighbor;
                continue;
            }

            if let Some((nodes, _cost)) = pathfinding::prelude::astar(
                &start_node,
                |node| self.get_adjacent_nodes(node, &token_shape, &elevation_range, bounds, walls, explored),
                |node| node.get_distance(&end_node),
                |node| node.at_node(&end_node),
            ) {
                for node in &nodes[1..nodes.len() - 1] {
                    path.push(start_waypoint.from_elevated_point(
                        self.get_node_top_left_point(node).round(),
                        true,
                        false,
                        true,
                    ));
                }

                path.push(end_waypoint.clone());

                token_shape = self.get_token_shape(end_waypoint.width, end_waypoint.height, end_waypoint.shape);
                start_waypoint = end_waypoint;
                start_node = *nodes.last().unwrap();
            } else {
                break;
            }
        }

        path
    }
}
