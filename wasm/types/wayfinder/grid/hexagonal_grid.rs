use crate::{
    EPSILON, GRID_DIAGONAL,
    exports::CancellationToken,
    modules::{geometry, math},
    traits::{AStar, BaseGrid},
    types::{
        foundry::{GridDiagonalRule, TokenShapeType},
        wayfinder::{
            ElevatedPoint, ElevationRange, FogManager, GridMeasurePathResult, GridOffset2D, GridOffset3D,
            HexagonalGridCube2D, HexagonalGridCube3D, Point, RegionManager, TokenDocument, TokenHexagonalShape,
            TokenMovementWaypoint, WallManager, node::HexagonalNode,
        },
    },
};
use geo::{Contains, Coord, Rect};
use itertools::Itertools;
use pathfinding::prelude::astar;
use rust_decimal::{Decimal, dec};
use tap::Tap;

#[derive(Clone, Debug)]
pub struct HexagonalGrid {
    pub size: i32,
    pub distance: f64,
    pub size_x: f64,
    pub size_y: f64,
    pub columns: bool,
    pub even: bool,
}

impl HexagonalGrid {
    pub fn cube_round(
        q: impl Into<f64>,
        r: impl Into<f64>,
        s: impl Into<f64>,
        k: impl Into<f64>,
    ) -> HexagonalGridCube3D {
        let q: f64 = q.into();
        let r: f64 = r.into();
        let s: f64 = s.into();
        let k: f64 = k.into();

        let mut iq = q.round();
        let mut ir = r.round();
        let mut is = s.round();

        let dq = (iq - q).abs();
        let dr = (ir - r).abs();
        let ds = (is - s).abs();

        if dq > dr && dq > ds {
            iq = -ir - is;
        } else if dr > ds {
            ir = -iq - is;
        } else {
            is = -iq - ir;
        }

        HexagonalGridCube3D { q: iq as i32, r: ir as i32, s: is as i32, k: (k + EPSILON).floor() as i32 }
    }

    pub fn point_to_cube(&self, ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint) -> HexagonalGridCube3D {
        let q;
        let r;

        x /= self.size as f64;
        y /= self.size as f64;

        if self.columns {
            q = 2.0 * crate::SQRT1_3 * x - 2.0 / 3.0;
            r = -0.5 * (q + (if self.even { 1.0 } else { 0.0 })) + y;
        } else {
            r = 2.0 * crate::SQRT1_3 * y - 2.0 / 3.0;
            q = -0.5 * (r + (if self.even { 1.0 } else { 0.0 })) + x;
        }

        HexagonalGrid::cube_round(q, r, 0.0 - q - r, elevation / self.distance)
    }

    pub fn cube_to_point(&self, HexagonalGridCube3D { q, r, s: _, k }: HexagonalGridCube3D) -> ElevatedPoint {
        let x;
        let y;

        if self.columns {
            x = 0.5 * crate::SQRT1_3 * ((3.0 * (q as f64) + 2.0) * (self.size as f64));
            y = (0.5 * ((q as f64) + (if self.even { 1.0 } else { 0.0 })) + (r as f64)) * (self.size as f64);
        } else {
            y = 0.5 * crate::SQRT1_3 * ((3.0 * (r as f64) + 2.0) * (self.size as f64));
            x = (0.5 * ((r as f64) + (if self.even { 1.0 } else { 0.0 })) + (q as f64)) * (self.size as f64);
        }

        ElevatedPoint { x, y, elevation: ((k as f64) + 0.5) * self.distance }
    }

    pub fn offset_to_cube(&self, GridOffset3D { i, j, k }: GridOffset3D) -> HexagonalGridCube3D {
        let q: i32;
        let r: i32;
        if self.columns {
            q = j;
            r = i - ((j + (if self.even { 1 } else { -1 }) * (j & 1)) >> 1);
        } else {
            q = j - ((i + (if self.even { 1 } else { -1 }) * (i & 1)) >> 1);
            r = i;
        }
        HexagonalGridCube3D { q: q, r: r, s: 0 - q - r, k }
    }

    pub fn cube_to_offset(&self, HexagonalGridCube3D { q, r, s: _, k }: HexagonalGridCube3D) -> GridOffset3D {
        if self.columns {
            GridOffset3D { i: q, j: r + ((q + (if self.even { 1 } else { -1 }) * (q & 1)) >> 1), k }
        } else {
            GridOffset3D { i: r, j: q + ((r + (if self.even { 1 } else { -1 }) * (r & 1)) >> 1), k }
        }
    }

    pub fn cube_distance(a: impl Into<HexagonalGridCube2D>, b: impl Into<HexagonalGridCube2D>) -> i32 {
        let HexagonalGridCube2D { q: q1, r: r1, s: _ } = a.into();
        let HexagonalGridCube2D { q: q2, r: r2, s: _ } = b.into();

        let dq = q1 - q2;
        let dr = r1 - r2;

        (dq.abs() + dr.abs() + (dq + dr).abs()) / 2
    }

    pub fn get_hexagonal_shape(
        width: f64,
        height: f64,
        shape: TokenShapeType,
        columns: bool,
    ) -> Option<TokenHexagonalShape> {
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

                return Some(TokenHexagonalShape {
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
            return Some(TokenHexagonalShape {
                even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                points: vec![
                    Point { x: 0.25, y: 0.0 },
                    Point { x: 0.5, y: 0.125 },
                    Point { x: 0.5, y: 0.375 },
                    Point { x: 0.25, y: 0.5 },
                    Point { x: 0.0, y: 0.375 },
                    Point { x: 0.0, y: 0.125 },
                ],
                center: Point { x: 0.25, y: 0.25 },
                anchor: Point { x: 0.25, y: 0.25 },
                width: width,
                height: height,
            });
        } else if width == 1.0 && height == 1.0 {
            return Some(TokenHexagonalShape {
                even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                points: vec![
                    Point { x: 0.5, y: 0.0 },
                    Point { x: 1.0, y: 0.25 },
                    Point { x: 1.0, y: 0.75 },
                    Point { x: 0.5, y: 1.0 },
                    Point { x: 0.0, y: 0.75 },
                    Point { x: 0.0, y: 0.25 },
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
    ) -> Option<TokenHexagonalShape> {
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
            for j in 0..(width as i32) - i {
                even_offsets.push(GridOffset2D { i: bottom - i, j: j + (((bottom & 1) + i + 1) >> 1) });
                odd_offsets.push(GridOffset2D { i: bottom - i, j: j + (((bottom & 1) + i) >> 1) });
            }
        }

        for i in 0..=top {
            for j in 0..(width as i32) - i {
                even_offsets.push(GridOffset2D { i: bottom + i, j: j + (((bottom & 1) + i + 1) >> 1) });
                odd_offsets.push(GridOffset2D { i: bottom + i, j: j + (((bottom & 1) + i) >> 1) });
            }
        }

        let mut x = 0.5 * (bottom as f64);
        let mut y = 0.25;

        for _k in 0..(width as i32) - bottom {
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

        for _k in 0..(width as i32) - top {
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
        return Some(TokenHexagonalShape {
            even_offsets,
            odd_offsets,
            points,
            center,
            anchor: if bottom % 2 != 0 { Point { x: 0.0, y: 0.5 } } else { Point { x: 0.5, y: 0.5 } },
            width,
            height,
        });
    }

    fn create_hexagonal_rectangle(width: f64, height: f64, shape: TokenShapeType) -> Option<TokenHexagonalShape> {
        if width < 1.0 || height.fract() != 0.0 {
            return None;
        }

        if width == 1.0 && height > 1.0 {
            return None;
        }

        if width.fract() != 0.0 && height == 1.0 {
            return None;
        }

        let even = shape == TokenShapeType::Rectangle1 || height == 1.0;
        let mut even_offsets = Vec::<GridOffset2D>::new();
        let mut odd_offsets = Vec::<GridOffset2D>::new();

        for i in 0..height as i32 {
            let j0 = if even { 0 } else { (i + 1) & 1 };
            let j1 = ((width + ((i & 1) as f64) * 0.5).floor() as i32) - (if even { i & 1 } else { 0 });
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

        return Some(TokenHexagonalShape {
            even_offsets,
            odd_offsets,
            points,
            center: Point { x: width / 2.0, y: (0.75 * height.floor() + 0.5 * (height % 1.0) + 0.25) / 2.0 },
            anchor: if even { Point { x: 0.5, y: 0.5 } } else { Point { x: 0.0, y: 0.5 } },
            width,
            height,
        });
    }
}

impl BaseGrid<HexagonalNode, TokenHexagonalShape> for HexagonalGrid {
    fn calculate_cost(
        &self,
        from: HexagonalNode,
        to: HexagonalNode,
        token_shape: &TokenHexagonalShape,
        fog_manager: Option<&FogManager>,
        _region_manager: &RegionManager,
        wall_manager: &WallManager,
    ) -> Option<(HexagonalNode, Decimal)> {
        let path = self.get_direct_path(vec![from, to]);
        if path.len() <= 1 {
            return None;
        }

        let mut n0 = path[0];
        let mut c = dec!(0);

        for n1 in path.into_iter().dropping(1) {
            if let Some(fog_manager) = fog_manager {
                if !fog_manager.is_point_explored(self.get_node_center_point(n1).into()) {
                    return None;
                }
            }

            if wall_manager.check_collisions(
                self.get_occupied_grid_space_offsets(self.convert_node_to_offset(n0), token_shape)
                    .into_iter()
                    .map(|offset| self.get_offset_center_point(offset).into())
                    .zip(
                        self.get_occupied_grid_space_offsets(self.convert_node_to_offset(n1), token_shape)
                            .into_iter()
                            .map(|offset| self.get_offset_center_point(offset).into()),
                    )
                    .collect(),
            ) {
                return None;
            }

            let d = if (n0.k == n1.k) || ((n0.q == n1.q) && (n0.r == n1.r) && (n0.s == n1.s)) {
                dec!(1)
            } else {
                match *GRID_DIAGONAL {
                    GridDiagonalRule::Equidistant => dec!(1),
                    GridDiagonalRule::Exact | GridDiagonalRule::Approximate => dec!(1.5),
                    GridDiagonalRule::Rectilinear => dec!(2),
                    GridDiagonalRule::Alternating1 => match n0.d {
                        true => dec!(2),
                        false => dec!(1),
                    },
                    GridDiagonalRule::Alternating2 => match n0.d {
                        true => dec!(1),
                        false => dec!(2),
                    },
                    GridDiagonalRule::Illegal => dec!(0),
                }
            };

            c += d;
            n0 = n1;
        }

        Some((to, c))
    }

    fn convert_node_to_offset(&self, HexagonalNode { q, r, s, k, d: _ }: HexagonalNode) -> GridOffset3D {
        self.cube_to_offset(HexagonalGridCube3D { q, r, s, k })
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> HexagonalNode {
        let HexagonalGridCube3D { q, r, s, k } = self.offset_to_cube(GridOffset3D { i, j, k });
        HexagonalNode { q, r, s, k, d: false }
    }

    fn get_adjacent_nodes(&self, HexagonalNode { q, r, s, k, d }: HexagonalNode) -> Vec<(HexagonalNode, Decimal)> {
        match *GRID_DIAGONAL {
            GridDiagonalRule::Equidistant => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), dec!(1)),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), dec!(1)),
            ],
            GridDiagonalRule::Exact | GridDiagonalRule::Approximate => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), dec!(1.5)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), dec!(1.5)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), dec!(1.5)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), dec!(1.5)),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), dec!(1.5)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), dec!(1.5)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), dec!(1.5)),
            ],
            GridDiagonalRule::Rectilinear => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), dec!(2)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), dec!(2)),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), dec!(2)),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), dec!(2)),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), dec!(2)),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), dec!(2)),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), dec!(2)),
            ],
            GridDiagonalRule::Alternating1 => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
            ],
            GridDiagonalRule::Alternating2 => vec![
                (HexagonalNode::new(q - 1, r, s + 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q - 1, r + 1, s, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q, r - 1, s + 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q + 1, r - 1, s, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q + 1, r, s - 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q - 1, r + 1, s, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q, r - 1, s + 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q + 1, r - 1, s, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (HexagonalNode::new(q + 1, r, s - 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
            ],
            GridDiagonalRule::Illegal => vec![
                (HexagonalNode::new(q, r, s, k - 1, d), dec!(1)),
                (HexagonalNode::new(q - 1, r, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q - 1, r + 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q, r - 1, s + 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r + 1, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r - 1, s, k, d), dec!(1)),
                (HexagonalNode::new(q + 1, r, s - 1, k, d), dec!(1)),
                (HexagonalNode::new(q, r, s, k + 1, d), dec!(1)),
            ],
        }
    }

    fn get_direct_path(&self, waypoints: Vec<HexagonalNode>) -> Vec<HexagonalNode> {
        if waypoints.len() <= 1 {
            return waypoints;
        }

        let HexagonalNode { q: mut q0, r: mut r0, s: mut s0, k: mut k0, d: d0 } = waypoints[0];
        let mut path = vec![HexagonalNode::new(q0, r0, s0, k0, d0)];
        let diagonals = *GRID_DIAGONAL != GridDiagonalRule::Illegal;

        for HexagonalNode { q: q1, r: r1, s: s1, k: k1, d: _ } in waypoints.into_iter().dropping(1) {
            if (q0 == q1) && (r0 == r1) && (k0 == k1) {
                continue;
            }

            let dq = q0 - q1;
            let dr = r0 - r1;
            let mut eq = 0.0;
            let mut er = 0.0;

            if self.columns {
                if dq == dr {
                    er = if !(((q0 + r0) & 1) != 0) == self.even { EPSILON } else { -EPSILON };
                    eq = -er;
                } else if -2 * dq == dr {
                    eq = if !((r0 & 1) != 0) == self.even { EPSILON } else { -EPSILON };
                } else if dq == -2 * dr {
                    er = if !((q0 & 1) != 0) == self.even { -EPSILON } else { EPSILON };
                }
            } else {
                if dq == dr {
                    eq = if !(((q0 + r0) & 1) != 0) == self.even { EPSILON } else { -EPSILON };
                    er = -eq;
                } else if dq == -2 * dr {
                    er = if !((q0 & 1) != 0) == self.even { EPSILON } else { -EPSILON };
                } else if -2 * dq == dr {
                    eq = if !((r0 & 1) != 0) == self.even { -EPSILON } else { EPSILON };
                }
            }

            let n = HexagonalGrid::cube_distance(
                HexagonalGridCube2D::new(q0, r0, s0),
                HexagonalGridCube2D::new(q1, r1, s1),
            );

            if n != 0 {
                let mut q = q0 as f64;
                let mut r = r0 as f64;
                let mut s = s0 as f64;
                let mut k = k0;
                let mut j = 0;
                let sk = if k0 < k1 { 1 } else { -1 };

                if diagonals {
                    let dk = 0 - (k0 - k1).abs();
                    let mut e = n + dk;

                    loop {
                        let e2 = e * 2;

                        if e2 >= dk {
                            e += dk;
                            j += 1;

                            let t = (j as f64 + EPSILON) / n as f64;
                            q = math::mix(q0, q1, t) + eq;
                            r = math::mix(r0, r1, t) + er;
                            s = 0.0 - q - r;
                        }

                        if e2 <= n {
                            e += n;
                            k += sk;
                        }

                        if (j == n) && (k == k1) {
                            break;
                        }

                        path.push(HexagonalGrid::cube_round(q, r, s, k).into());
                    }
                } else {
                    let dk1 = (k0 - k1).abs().min(1);
                    let mut tc = dk1;
                    let mut tk = n;

                    loop {
                        if tc <= tk {
                            tc += dk1;
                            j += 1;

                            let t = (j as f64 + EPSILON) / n as f64;
                            q = math::mix(q0, q1, t) + eq;
                            r = math::mix(r0, r1, t) + er;
                            s = 0.0 - q - r;
                        } else {
                            tk += n;
                            k += sk;
                        }

                        if (j == n) && (k == k1) {
                            break;
                        }

                        path.push(HexagonalGrid::cube_round(q, r, s, k).into());
                    }
                }

                path.push(HexagonalNode::new(q1, r1, s1, k1, false));
            } else {
                let mut k = k0;
                let sk = if k0 < k1 { 1 } else { -1 };
                while k != k1 {
                    k += sk;
                    path.push(HexagonalNode::new(q0, r0, s0, k, false));
                }
            }

            q0 = q1;
            r0 = r1;
            s0 = s1;
            k0 = k1;
        }

        if diagonals {
            for i in 1..path.len() {
                if self.test_diagonal(path[i - 1], path[i]) {
                    path[i].d = !path[i - 1].d;
                }
            }
        }

        path
    }

    fn get_node(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShape {
            even_offsets: _,
            odd_offsets: _,
            points: _,
            center: _,
            anchor,
            width: _,
            height: _,
        }: &TokenHexagonalShape,
    ) -> HexagonalNode {
        self.convert_offset_to_node(self.cube_to_offset(self.point_to_cube(ElevatedPoint {
            x: x + self.size_x * anchor.x,
            y: y + self.size_y * anchor.y,
            elevation,
        })))
    }

    fn get_node_center_point(&self, HexagonalNode { q, r, s: _, k, d: _ }: HexagonalNode) -> ElevatedPoint {
        if self.columns {
            ElevatedPoint {
                x: 0.5 * crate::SQRT1_3 * ((3.0 * (q as f64) + 2.0) * (self.size as f64)),
                y: (0.5 * ((q as f64) + (if self.even { 1.0 } else { 0.0 })) + (r as f64)) * (self.size as f64),
                elevation: ((k as f64) + 0.5) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: 0.5 * crate::SQRT1_3 * ((3.0 * (r as f64) + 2.0) * (self.size as f64)),
                x: (0.5 * ((r as f64) + (if self.even { 1.0 } else { 0.0 })) + (q as f64)) * (self.size as f64),
                elevation: ((k as f64) + 0.5) * self.distance,
            }
        }
    }

    fn get_node_top_left_point(&self, HexagonalNode { q, r, s: _, k, d: _diagonal }: HexagonalNode) -> ElevatedPoint {
        if self.columns {
            ElevatedPoint {
                x: (crate::SQRT3 / 2.0) * ((q as f64) * (self.size as f64)),
                y: (0.5 * ((q as f64) - (if self.even { 0.0 } else { 1.0 })) + (r as f64)) * (self.size as f64),
                elevation: (k as f64) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (crate::SQRT3 / 2.0) * ((r as f64) * (self.size as f64)),
                x: (0.5 * ((r as f64) - (if self.even { 0.0 } else { 1.0 })) + (q as f64)) * (self.size as f64),
                elevation: (k as f64) * self.distance,
            }
        }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        GridOffset3D { i, j, k }: GridOffset3D,
        TokenHexagonalShape {
            even_offsets,
            odd_offsets,
            points: _,
            center: _,
            anchor: _,
            width: _,
            height: _,
        }: &TokenHexagonalShape,
    ) -> Vec<GridOffset3D> {
        (if ((if self.columns { j } else { i }) % 2 == 0) == self.even { even_offsets } else { odd_offsets })
            .iter()
            .map(|offset| GridOffset3D { i: i + offset.i, j: j + offset.j, k })
            .collect()
    }

    fn get_offset(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShape {
            even_offsets: _,
            odd_offsets: _,
            points: _,
            center: _,
            anchor,
            width: _,
            height: _,
        }: &TokenHexagonalShape,
    ) -> GridOffset3D {
        self.cube_to_offset(self.point_to_cube(ElevatedPoint {
            x: x + self.size_x * anchor.x,
            y: y + self.size_y * anchor.y,
            elevation,
        }))
    }

    fn get_offset_center_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        let HexagonalGridCube3D { q, r, s: _, k } = self.offset_to_cube(offset);

        if self.columns {
            ElevatedPoint {
                x: 0.5 * crate::SQRT1_3 * ((3.0 * (q as f64) + 2.0) * (self.size as f64)),
                y: (0.5 * ((q as f64) + (if self.even { 1.0 } else { 0.0 })) + (r as f64)) * (self.size as f64),
                elevation: ((k as f64) + 0.5) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: 0.5 * crate::SQRT1_3 * ((3.0 * (r as f64) + 2.0) * (self.size as f64)),
                x: (0.5 * ((r as f64) + (if self.even { 1.0 } else { 0.0 })) + (q as f64)) * (self.size as f64),
                elevation: ((k as f64) + 0.5) * self.distance,
            }
        }
    }

    fn get_offset_top_left_point(&self, offset: GridOffset3D) -> ElevatedPoint {
        let HexagonalGridCube3D { q, r, s: _, k } = self.offset_to_cube(offset);

        if self.columns {
            ElevatedPoint {
                x: (crate::SQRT3 / 2.0) * ((q as f64) * (self.size as f64)),
                y: (0.5 * ((q as f64) - (if self.even { 0.0 } else { 1.0 })) + (r as f64)) * (self.size as f64),
                elevation: (k as f64) * self.distance,
            }
        } else {
            ElevatedPoint {
                y: (crate::SQRT3 / 2.0) * ((r as f64) * (self.size as f64)),
                x: (0.5 * ((r as f64) - (if self.even { 0.0 } else { 1.0 })) + (q as f64)) * (self.size as f64),
                elevation: (k as f64) * self.distance,
            }
        }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenHexagonalShape {
            even_offsets: _,
            odd_offsets: _,
            points: _,
            center,
            anchor: _,
            width: _,
            height: _,
        }: &TokenHexagonalShape,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + center.x * self.size_x, y: y + center.y * self.size_y, elevation }
    }

    fn get_token_shape(&self, mut width: f64, mut height: f64, shape: TokenShapeType) -> TokenHexagonalShape {
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

            if let Some(TokenHexagonalShape { even_offsets, odd_offsets, points, center, anchor, width, height }) =
                HexagonalGrid::get_hexagonal_shape(width, height, TokenShapeType::Rectangle1, self.columns)
            {
                TokenHexagonalShape {
                    even_offsets,
                    odd_offsets,
                    points,
                    center,
                    anchor: Point { x: anchor.x - 0.25, y: anchor.y - 0.25 },
                    width,
                    height,
                }
            } else {
                TokenHexagonalShape {
                    even_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                    odd_offsets: vec![GridOffset2D { i: 0, j: 0 }],
                    points: vec![
                        Point { x: 0.5, y: 0.0 },
                        Point { x: 1.0, y: 0.25 },
                        Point { x: 1.0, y: 0.75 },
                        Point { x: 0.5, y: 1.0 },
                        Point { x: 0.0, y: 0.75 },
                        Point { x: 0.0, y: 0.25 },
                    ],
                    center: Point { x: 0.5, y: 0.5 },
                    anchor: Point { x: 0.5, y: 0.5 },
                    width: 1.0,
                    height: 1.0,
                }
            }
        }
    }

    fn measure_path(&self, waypoints: Vec<HexagonalNode>) -> Decimal {
        if waypoints.len() <= 1 {
            return dec!(0);
        }

        let mut c = dec!(0);
        let mut n0 = waypoints[0];
        let mut nd = match *GRID_DIAGONAL {
            GridDiagonalRule::Alternating2 => match n0.d {
                true => dec!(0),
                false => dec!(1),
            },
            _ => match n0.d {
                true => dec!(1),
                false => dec!(0),
            },
        };

        for n1 in waypoints.into_iter().dropping(1) {
            let [n, d] = [Decimal::from(HexagonalGrid::cube_distance(n0, n1)), Decimal::from((n0.k - n1.k).abs())]
                .tap_mut(|values| {
                    values.sort();
                    values.reverse();
                });
            let nd0 = nd;

            match *GRID_DIAGONAL {
                GridDiagonalRule::Equidistant => c += n,
                GridDiagonalRule::Exact | GridDiagonalRule::Approximate => c += n + (dec!(0.5) * d),
                GridDiagonalRule::Rectilinear => c += n + d,
                GridDiagonalRule::Alternating1 | GridDiagonalRule::Alternating2 => {
                    nd += d;
                    c += n + ((nd / dec!(2)).floor() - (nd0 / dec!(2)).floor());
                }
                GridDiagonalRule::Illegal => c += n + d,
            }

            n0 = n1;
        }

        c
    }

    fn simplify_path(&self, path: Vec<HexagonalNode>) -> Vec<HexagonalNode> {
        let mut path: Vec<HexagonalNode> = path.clone();
        let mut i = 0;

        while i + 1 < path.len() {
            let n0 = path[i];
            let n1 = path[i + 1];

            if !self.test_adjacency(n0, n1) {
                path.splice(
                    (i + 1)..(i + 1),
                    self.get_direct_path(vec![n0, n1]).into_iter().dropping(1).dropping_back(1),
                );
            }

            i += 1;
        }

        i = 0;

        while i + 2 < path.len() {
            let n0 = path[i];
            let n1 = path[i + 1];
            let n2 = path[i + 2];

            let v0 = HexagonalGridCube3D {
                q: (n1.q - n0.q).clamp(-1, 1),
                r: (n1.r - n0.r).clamp(-1, 1),
                s: (n1.s - n0.s).clamp(-1, 1),
                k: (n1.k - n0.k).clamp(-1, 1),
            };

            let v1 = HexagonalGridCube3D {
                q: (n2.q - n1.q).clamp(-1, 1),
                r: (n2.r - n1.r).clamp(-1, 1),
                s: (n2.s - n1.s).clamp(-1, 1),
                k: (n2.k - n1.k).clamp(-1, 1),
            };

            if v0 == v1 {
                path.remove(i + 1);
            } else {
                i += 1;
            }
        }

        path
    }

    fn test_adjacency(&self, a: HexagonalNode, b: HexagonalNode) -> bool {
        let d0 = HexagonalGrid::cube_distance(a, b);
        let d1 = i32::abs(a.k - b.k);
        if d0 > 1 || d1 > 1 {
            false
        } else {
            if *GRID_DIAGONAL == GridDiagonalRule::Illegal { d0 + d1 == 1 } else { d0 + d1 != 0 }
        }
    }

    fn test_diagonal(&self, a: HexagonalNode, b: HexagonalNode) -> bool {
        let d0 = HexagonalGrid::cube_distance(a, b);
        let d1 = i32::abs(a.k - b.k);
        if d0 > 1 || d1 > 1 {
            false
        } else {
            if *GRID_DIAGONAL == GridDiagonalRule::Illegal { false } else { d0 + d1 == 2 }
        }
    }
}

impl AStar<HexagonalNode, TokenHexagonalShape> for HexagonalGrid {
    fn find_path(
        &self,
        cancellation_token: &CancellationToken,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        scene_rect: &Rect,
        fog_manager: Option<&FogManager>,
        region_manager: &RegionManager,
        wall_manager: &WallManager,
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
            let elevation_range: ElevationRange<i32> =
                ElevationRange::new(start_node.k.min(end_node.k), start_node.k.max(end_node.k));

            if let Some((nodes, _cost)) = astar(
                &start_node,
                |node| match cancellation_token.status() {
                    false => self
                        .calculate_cost(*node, end_node, &token_shape, fog_manager, region_manager, wall_manager)
                        .into_iter()
                        .chain(self.get_adjacent_nodes(*node).into_iter())
                        .filter(|(successor, _cost)| elevation_range.contains(successor.k))
                        .filter(|(successor, _cost)| {
                            scene_rect.contains(&Coord::from(self.get_node_center_point(*successor)))
                        })
                        .filter(|(successor, _cost)| {
                            if let Some(fog_manager) = fog_manager {
                                fog_manager.is_point_explored(self.get_node_center_point(*successor).into())
                            } else {
                                true
                            }
                        })
                        .filter(|(successor, _cost)| {
                            !wall_manager.check_collisions(
                                self.get_occupied_grid_space_offsets(self.convert_node_to_offset(*node), &token_shape)
                                    .into_iter()
                                    .map(|offset| self.get_offset_center_point(offset).into())
                                    .zip(
                                        self.get_occupied_grid_space_offsets(
                                            self.convert_node_to_offset(*successor),
                                            &token_shape,
                                        )
                                        .into_iter()
                                        .map(|offset| self.get_offset_center_point(offset).into()),
                                    )
                                    .collect(),
                            )
                        })
                        .collect(),
                    true => Vec::new(),
                },
                |node| self.measure_path(vec![node.clone(), end_node]),
                |node| node.q == end_node.q && node.r == end_node.r && node.s == end_node.s && node.k == end_node.k,
            ) {
                let nodes = self.simplify_path(nodes);

                for node in nodes.iter().dropping(1).dropping_back(1) {
                    path.push(start_waypoint.from_elevated_point(
                        self.get_node_top_left_point(*node).round(),
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
