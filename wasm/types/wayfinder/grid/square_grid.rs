use crate::{
    exports::CancellationToken,
    traits::{AStar, BaseGrid},
    types::{
        foundry::{GridDiagonalRule, TokenShapeType},
        wayfinder::{
            ElevatedPoint, ElevationRange, FogManager, GridMeasurePathResult, GridOffset2D, GridOffset3D, Point,
            RegionManager, TokenDocument, TokenMovementWaypoint, TokenSquareShape, WallManager, node::SquareNode,
        },
    },
};
use geo::{Contains, Coord, Rect};
use itertools::Itertools;
use pathfinding::prelude::astar;
use rust_decimal::{Decimal, dec};
use tap::Tap;

#[derive(Clone, Debug)]
pub struct SquareGrid {
    pub size: i32,
    pub distance: f64,
    pub diagonals: GridDiagonalRule,
}

impl SquareGrid {
    pub fn measure_distance(
        SquareNode { i: i0, j: j0, k: k0, d: d0 }: SquareNode,
        SquareNode { i: i1, j: j1, k: k1, d: _ }: SquareNode,
        diagonals: GridDiagonalRule,
    ) -> Decimal {
        let mut c = dec!(0);
        let mut nd = match diagonals {
            GridDiagonalRule::Alternating2 => match d0 {
                true => dec!(0),
                false => dec!(1.5),
            },
            _ => match d0 {
                true => dec!(1.5),
                false => dec!(0),
            },
        };

        let [di, dj, dk] =
            [Decimal::from((i0 - i1).abs()), Decimal::from((j0 - j1).abs()), Decimal::from((k0 - k1).abs())].tap_mut(
                |values| {
                    values.sort();
                    values.reverse();
                },
            );
        let nd0 = nd;

        match diagonals {
            GridDiagonalRule::Equidistant => c += di,
            GridDiagonalRule::Exact | GridDiagonalRule::Approximate => {
                c += di + ((dec!(0.5) * (dj - dk)) + (dec!(0.75) * dk))
            }
            GridDiagonalRule::Rectilinear => c += di + (dj + dk),
            GridDiagonalRule::Alternating1 | GridDiagonalRule::Alternating2 => {
                nd += dj + (dec!(0.5) * dk);
                c += di + ((nd / dec!(2)).floor() - (nd0 / dec!(2)).floor());
            }
            GridDiagonalRule::Illegal => c += di + (dj + dk),
        };

        c
    }
}

impl BaseGrid<SquareNode, TokenSquareShape> for SquareGrid {
    fn calculate_cost(
        &self,
        from: SquareNode,
        to: SquareNode,
        token_shape: &TokenSquareShape,
        level: &str,
        fog_manager: Option<&FogManager>,
        _region_manager: &RegionManager,
        wall_manager: &WallManager,
    ) -> Option<(SquareNode, Decimal)> {
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
                level,
            ) {
                return None;
            }

            let m = (n0.i == n1.i) as i32 + (n0.j == n1.j) as i32 + (n0.k == n1.k) as i32;
            let k = match m {
                2 => dec!(1),
                1 => match self.diagonals {
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
                },
                _ => match self.diagonals {
                    GridDiagonalRule::Equidistant => dec!(1),
                    GridDiagonalRule::Exact | GridDiagonalRule::Approximate => dec!(1.75),
                    GridDiagonalRule::Rectilinear => dec!(3),
                    GridDiagonalRule::Alternating1 => match n0.d {
                        true => dec!(2),
                        false => dec!(1),
                    },
                    GridDiagonalRule::Alternating2 => match n0.d {
                        true => dec!(1),
                        false => dec!(2),
                    },
                    GridDiagonalRule::Illegal => dec!(0),
                },
            };

            c += k;
            n0 = n1;
        }

        Some((to, c))
    }

    fn convert_node_to_offset(&self, SquareNode { i, j, k, d: _ }: SquareNode) -> GridOffset3D {
        GridOffset3D { i, j, k }
    }

    fn convert_offset_to_node(&self, GridOffset3D { j, i, k }: GridOffset3D) -> SquareNode {
        SquareNode { i, j, k, d: false }
    }

    fn convert_point_to_node(&self, ElevatedPoint { x, y, elevation }: ElevatedPoint) -> SquareNode {
        SquareNode {
            j: (y / (self.size as f64)).floor() as i32,
            i: (x / (self.size as f64)).floor() as i32,
            k: (elevation / self.distance + crate::EPSILON).floor() as i32,
            d: false,
        }
    }

    fn get_adjacent_nodes(&self, SquareNode { i, j, k, d }: SquareNode) -> Vec<(SquareNode, Decimal)> {
        match self.diagonals {
            GridDiagonalRule::Equidistant => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i - 1, j - 1, k, !d), dec!(1)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), dec!(1)),
                (SquareNode::new(i - 1, j, k - 1, !d), dec!(1)),
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i - 1, j, k + 1, !d), dec!(1)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i - 1, j + 1, k, !d), dec!(1)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), dec!(1)),
                (SquareNode::new(i, j - 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k + 1, !d), dec!(1)),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i, j + 1, k + 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j - 1, k, !d), dec!(1)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j, k - 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k + 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), dec!(1)),
                (SquareNode::new(i + 1, j + 1, k, !d), dec!(1)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), dec!(1)),
            ],
            GridDiagonalRule::Exact | GridDiagonalRule::Approximate => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), dec!(1.75)),
                (SquareNode::new(i - 1, j - 1, k, !d), dec!(1.5)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), dec!(1.75)),
                (SquareNode::new(i - 1, j, k - 1, !d), dec!(1.5)),
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i - 1, j, k + 1, !d), dec!(1.5)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), dec!(1.75)),
                (SquareNode::new(i - 1, j + 1, k, !d), dec!(1.5)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), dec!(1.75)),
                (SquareNode::new(i, j - 1, k - 1, !d), dec!(1.5)),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k + 1, !d), dec!(1.5)),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k - 1, !d), dec!(1.5)),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i, j + 1, k + 1, !d), dec!(1.5)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), dec!(1.75)),
                (SquareNode::new(i + 1, j - 1, k, !d), dec!(1.5)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), dec!(1.75)),
                (SquareNode::new(i + 1, j, k - 1, !d), dec!(1.5)),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k + 1, !d), dec!(1.5)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), dec!(1.75)),
                (SquareNode::new(i + 1, j + 1, k, !d), dec!(1.5)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), dec!(1.75)),
            ],
            GridDiagonalRule::Rectilinear => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), dec!(3)),
                (SquareNode::new(i - 1, j - 1, k, !d), dec!(2)),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), dec!(3)),
                (SquareNode::new(i - 1, j, k - 1, !d), dec!(2)),
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i - 1, j, k + 1, !d), dec!(2)),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), dec!(3)),
                (SquareNode::new(i - 1, j + 1, k, !d), dec!(2)),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), dec!(3)),
                (SquareNode::new(i, j - 1, k - 1, !d), dec!(2)),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k + 1, !d), dec!(2)),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k - 1, !d), dec!(2)),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i, j + 1, k + 1, !d), dec!(2)),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), dec!(3)),
                (SquareNode::new(i + 1, j - 1, k, !d), dec!(2)),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), dec!(3)),
                (SquareNode::new(i + 1, j, k - 1, !d), dec!(2)),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k + 1, !d), dec!(2)),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), dec!(3)),
                (SquareNode::new(i + 1, j + 1, k, !d), dec!(2)),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), dec!(3)),
            ],
            GridDiagonalRule::Alternating1 => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j - 1, k, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i - 1, j, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j + 1, k, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i, j - 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i, j + 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j - 1, k, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j + 1, k, !d), if d { dec!(2) } else { dec!(1) }),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), if d { dec!(2) } else { dec!(1) }),
            ],
            GridDiagonalRule::Alternating2 => vec![
                (SquareNode::new(i - 1, j - 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j - 1, k, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j - 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i - 1, j, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j + 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j + 1, k, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i - 1, j + 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i, j - 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i, j + 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j - 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j - 1, k, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j - 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j + 1, k - 1, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j + 1, k, !d), if d { dec!(1) } else { dec!(2) }),
                (SquareNode::new(i + 1, j + 1, k + 1, !d), if d { dec!(1) } else { dec!(2) }),
            ],
            GridDiagonalRule::Illegal => vec![
                (SquareNode::new(i - 1, j, k, d), dec!(1)),
                (SquareNode::new(i, j - 1, k, d), dec!(1)),
                (SquareNode::new(i, j, k - 1, d), dec!(1)),
                (SquareNode::new(i, j, k + 1, d), dec!(1)),
                (SquareNode::new(i, j + 1, k, d), dec!(1)),
                (SquareNode::new(i + 1, j, k, d), dec!(1)),
            ],
        }
    }

    fn get_direct_path(&self, waypoints: Vec<SquareNode>) -> Vec<SquareNode> {
        if waypoints.len() <= 1 {
            return waypoints;
        }

        let SquareNode { i: mut i0, j: mut j0, k: mut k0, d: d0 } = waypoints[0];
        let mut path = vec![SquareNode::new(i0, j0, k0, d0)];
        let diagonals = self.diagonals != GridDiagonalRule::Illegal;

        for SquareNode { i: i1, j: j1, k: k1, d: _ } in waypoints.into_iter().dropping(1) {
            if (i0 == i1) && (j0 == j1) && (k0 == k1) {
                continue;
            }

            let di = (i0 - i1).abs();
            let dj = (j0 - j1).abs();
            let dk = (k0 - k1).abs();
            let si = if i0 < i1 { 1 } else { -1 };
            let sj = if j0 < j1 { 1 } else { -1 };
            let sk = if k0 < k1 { 1 } else { -1 };

            if diagonals {
                let di2 = 2 * di;
                let dj2 = 2 * dj;
                let dk2 = 2 * dk;

                if (di >= dj) && (di >= dk) {
                    let mut ej = 0 - di;
                    let mut ek = ej;

                    loop {
                        ej += dj2;
                        ek += dk2;
                        i0 += si;

                        if ej >= 0 {
                            ej -= di2;
                            j0 += sj;
                        }

                        if ek >= 0 {
                            ek -= di2;
                            k0 += sk;
                        }

                        if i0 == i1 {
                            break;
                        }

                        path.push(SquareNode::new(i0, j0, k0, false));
                    }
                } else if (dj >= di) && (dj >= dk) {
                    let mut ei = 0 - dj;
                    let mut ek = ei;

                    loop {
                        ei += di2;
                        ek += dk2;
                        j0 += sj;

                        if ei >= 0 {
                            ei -= dj2;
                            i0 += si;
                        }

                        if ek >= 0 {
                            ek -= dj2;
                            k0 += sk;
                        }

                        if j0 == j1 {
                            break;
                        }

                        path.push(SquareNode::new(i0, j0, k0, false));
                    }
                } else {
                    let mut ei = 0 - dk;
                    let mut ej = ei;

                    loop {
                        ei += di2;
                        ej += dj2;
                        k0 += sk;

                        if ei >= 0 {
                            ei -= dk2;
                            i0 += si;
                        }

                        if ej >= 0 {
                            ej -= dk2;
                            j0 += sj;
                        }

                        if k0 == k1 {
                            break;
                        }

                        path.push(SquareNode::new(i0, j0, k0, false));
                    }
                }
            } else {
                let di1 = di.max(1);
                let dj1 = dj.max(1);
                let dk1 = dk.max(1);
                let tdi = dj1 * dk1;
                let tdj = di1 * dk1;
                let tdk = di1 * dj1;
                let tm = (di1 * dj1 * dk1) + 1;
                let mut ti = if di > 0 { tdi } else { tm };
                let mut tj = if dj > 0 { tdj } else { tm };
                let mut tk = if dk > 0 { tdk } else { tm };

                loop {
                    if ti < tj {
                        if ti <= tk {
                            ti += tdi;
                            i0 += si;
                        } else {
                            tk += tdk;
                            k0 += sk;
                        }
                    } else {
                        if tj <= tk {
                            tj += tdj;
                            j0 += sj;
                        } else {
                            tk += tdk;
                            k0 += sk;
                        }
                    }

                    if (i0 == i1) && (j0 == j1) && (k0 == k1) {
                        break;
                    }

                    path.push(SquareNode::new(i0, j0, k0, false));
                }
            }

            path.push(SquareNode::new(i1, j1, k1, false));

            i0 = i1;
            j0 = j0;
            k0 = k0;
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
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShape { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShape,
    ) -> SquareNode {
        x += (self.size as f64) * (if width.fract() == 0.0 { 0.5 } else { 0.25 });
        y += (self.size as f64) * (if height.fract() == 0.0 { 0.5 } else { 0.25 });

        SquareNode {
            i: (y / (self.size as f64)).floor() as i32,
            j: (x / (self.size as f64)).floor() as i32,
            k: (elevation / self.distance + crate::EPSILON).floor() as i32,
            d: false,
        }
    }

    fn get_node_center_point(&self, SquareNode { i, j, k, d: _ }: SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: ((j as f64) + 0.5) * (self.size as f64),
            y: ((i as f64) + 0.5) * (self.size as f64),
            elevation: ((k as f64) + 0.5) * self.distance,
        }
    }

    fn get_node_top_left_point(&self, SquareNode { i, j, k, d: _ }: SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: (j as f64) * (self.size as f64),
            y: (i as f64) * (self.size as f64),
            elevation: (k as f64) * self.distance,
        }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        GridOffset3D { j, i, k }: GridOffset3D,
        TokenSquareShape { offsets, points: _, center: _, anchor: _, width: _, height: _ }: &TokenSquareShape,
    ) -> Vec<GridOffset3D> {
        offsets.iter().map(|offset| GridOffset3D { i: i + offset.i, j: j + offset.j, k }).collect()
    }

    fn get_offset(
        &self,
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShape { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShape,
    ) -> GridOffset3D {
        x += (self.size as f64) * (if width.fract() == 0.0 { 0.5 } else { 0.25 });
        y += (self.size as f64) * (if height.fract() == 0.0 { 0.5 } else { 0.25 });

        GridOffset3D {
            j: (y / (self.size as f64)).floor() as i32,
            i: (x / (self.size as f64)).floor() as i32,
            k: (elevation / self.distance + crate::EPSILON).floor() as i32,
        }
    }

    fn get_offset_center_point(&self, GridOffset3D { j, i, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: ((j as f64) + 0.5) * (self.size as f64),
            y: ((i as f64) + 0.5) * (self.size as f64),
            elevation: (k as f64) * self.distance,
        }
    }

    fn get_offset_top_left_point(&self, GridOffset3D { j, i, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: (j as f64) * (self.size as f64),
            y: (i as f64) * (self.size as f64),
            elevation: (k as f64) * self.distance,
        }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenSquareShape { offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenSquareShape,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + center.x * (self.size as f64), y: y + center.y * (self.size as f64), elevation }
    }

    fn get_token_shape(&self, width: f64, height: f64, _shape: TokenShapeType) -> TokenSquareShape {
        let mut offsets = Vec::<GridOffset2D>::new();
        let width = (width * 2.0).round() / 2.0;
        let height = (height * 2.0).round() / 2.0;

        for i in 0..height.ceil() as i32 {
            for j in 0..width.ceil() as i32 {
                offsets.push(GridOffset2D { i, j });
            }
        }

        TokenSquareShape {
            offsets,
            points: vec![Point::new(0, 0), Point::new(width, 0), Point::new(width, height), Point::new(0, height)],
            center: Point::new(width / 2.0, height / 2.0),
            anchor: Point::new(0, 0),
            width,
            height,
        }
    }

    fn measure_path(&self, waypoints: Vec<SquareNode>) -> Decimal {
        if waypoints.len() <= 1 {
            return dec!(0);
        }

        let mut c = dec!(0);
        let mut n0 = waypoints[0];
        let mut nd = match self.diagonals {
            GridDiagonalRule::Alternating2 => match n0.d {
                true => dec!(0),
                false => dec!(1.5),
            },
            _ => match n0.d {
                true => dec!(1.5),
                false => dec!(0),
            },
        };

        for n1 in waypoints.into_iter().dropping(1) {
            let [di, dj, dk] = [
                Decimal::from((n0.i - n1.i).abs()),
                Decimal::from((n0.j - n1.j).abs()),
                Decimal::from((n0.k - n1.k).abs()),
            ]
            .tap_mut(|values| {
                values.sort();
                values.reverse();
            });
            let nd0 = nd;

            match self.diagonals {
                GridDiagonalRule::Equidistant => c += di,
                GridDiagonalRule::Exact | GridDiagonalRule::Approximate => {
                    c += di + ((dec!(0.5) * (dj - dk)) + (dec!(0.75) * dk))
                }
                GridDiagonalRule::Rectilinear => c += di + (dj + dk),
                GridDiagonalRule::Alternating1 | GridDiagonalRule::Alternating2 => {
                    nd += dj + (dec!(0.5) * dk);
                    c += di + ((nd / dec!(2)).floor() - (nd0 / dec!(2)).floor());
                }
                GridDiagonalRule::Illegal => c += di + (dj + dk),
            }

            n0 = n1;
        }

        c
    }

    fn simplify_path(&self, path: Vec<SquareNode>) -> Vec<SquareNode> {
        let mut path = path.clone();
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

            let v0 = GridOffset3D {
                i: (n1.i - n0.i).clamp(-1, 1),
                j: (n1.j - n0.j).clamp(-1, 1),
                k: (n1.k - n0.k).clamp(-1, 1),
            };

            let v1 = GridOffset3D {
                i: (n2.i - n1.i).clamp(-1, 1),
                j: (n2.j - n1.j).clamp(-1, 1),
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

    fn test_adjacency(
        &self,
        SquareNode { i: i1, j: j1, k: k1, d: _ }: SquareNode,
        SquareNode { i: i2, j: j2, k: k2, d: _ }: SquareNode,
    ) -> bool {
        let di = i32::abs(i1 - i2);
        let dj = i32::abs(j1 - j2);
        let dk = i32::abs(k1 - k2);
        if self.diagonals != GridDiagonalRule::Illegal { di.max(dj.max(dk)) == 1 } else { (di + dj + dk) == 1 }
    }

    fn test_diagonal(
        &self,
        SquareNode { i: i1, j: j1, k: k1, d: _ }: SquareNode,
        SquareNode { i: i2, j: j2, k: k2, d: _ }: SquareNode,
    ) -> bool {
        let di = i32::abs(i1 - i2);
        let dj = i32::abs(j1 - j2);
        let dk = i32::abs(k1 - k2);
        if self.diagonals != GridDiagonalRule::Illegal {
            if di.max(dj.max(dk)) == 1 { (di + dj + dk) >= 2 } else { false }
        } else {
            false
        }
    }
}

impl AStar<SquareNode, TokenSquareShape> for SquareGrid {
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
            if start_waypoint.level != end_waypoint.level {
                break;
            }

            let end_node = self.get_node(end_waypoint.create_elevated_point(), &token_shape);
            let elevation_range: ElevationRange<i32> =
                ElevationRange::new(start_node.k.min(end_node.k), start_node.k.max(end_node.k));

            if let Some((nodes, _cost)) = astar(
                &start_node,
                |node| match cancellation_token.status() {
                    false => self
                        .calculate_cost(
                            *node,
                            end_node,
                            &token_shape,
                            &start_waypoint.level,
                            fog_manager,
                            region_manager,
                            wall_manager,
                        )
                        .into_iter()
                        .chain(self.get_adjacent_nodes(*node).into_iter().sorted_by_key(|(successor, _cost)| {
                            SquareGrid::measure_distance(*successor, end_node, GridDiagonalRule::Rectilinear)
                        }))
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
                                &start_waypoint.level,
                            )
                        })
                        .collect(),
                    true => Vec::new(),
                },
                |node| self.measure_path(vec![node.clone(), end_node]),
                |node| node.i == end_node.i && node.j == end_node.j && node.k == end_node.k,
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
