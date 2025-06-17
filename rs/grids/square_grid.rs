use std::{collections::HashMap, ops::RangeInclusive};

use crate::{
    enums::TokenShapeType,
    exports::Walls,
    nodes::SquareNode,
    traits::{AStar, BaseGrid, Node},
    types::{
        ElevatedPoint, GridMeasurePathResult, GridOffset2D, GridOffset3D, Point, Rectangle, TokenDocument,
        TokenMovementWaypoint, TokenSquareShapeData,
    },
};

#[derive(Debug)]
pub struct SquareGrid {
    pub size: i32,
    pub distance: f64,
}

impl BaseGrid<SquareNode, TokenSquareShapeData> for SquareGrid {
    fn convert_node_to_offset(&self, SquareNode { i, j, k, d: _ }: SquareNode) -> GridOffset3D {
        GridOffset3D { i, j, k }
    }

    fn convert_offset_to_node(&self, GridOffset3D { i, j, k }: GridOffset3D) -> SquareNode {
        SquareNode { i, j, k, d: false }
    }

    fn get_adjacent_nodes(
        &self,
        node: &SquareNode,
        token_shape: &TokenSquareShapeData,
        elevation_range: &RangeInclusive<i32>,
        bounds: &Rectangle,
        walls: &Walls,
        explored: &Option<HashMap<GridOffset2D, bool>>,
    ) -> Vec<(SquareNode, u32)> {
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
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShapeData,
    ) -> SquareNode {
        x += self.size as f64 * if width.fract() == 0.0 { 0.5 } else { 0.25 };
        y += self.size as f64 * if height.fract() == 0.0 { 0.5 } else { 0.25 };

        SquareNode {
            i: (y / self.size as f64).floor() as i32,
            j: (x / self.size as f64).floor() as i32,
            k: ((elevation / self.distance) + 1e-8).floor() as i32,
            d: false,
        }
    }

    fn get_node_center_point(&self, SquareNode { i, j, k, d: _ }: &SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: (*j as f64 + 0.5) * self.size as f64,
            y: (*i as f64 + 0.5) * self.size as f64,
            elevation: (*k as f64 + 0.5) * self.distance,
        }
    }

    fn get_node_top_left_point(&self, SquareNode { i, j, k, d: _ }: &SquareNode) -> ElevatedPoint {
        ElevatedPoint {
            x: *j as f64 * self.size as f64,
            y: *i as f64 * self.size as f64,
            elevation: *k as f64 * self.distance,
        }
    }

    fn get_occupied_grid_space_offsets(
        &self,
        GridOffset3D { i, j, k }: GridOffset3D,
        token_shape: &TokenSquareShapeData,
    ) -> Vec<GridOffset3D> {
        token_shape.offsets.iter().map(|offset| GridOffset3D { i: i + offset.i, j: j + offset.j, k }).collect()
    }

    fn get_offset(
        &self,
        ElevatedPoint { mut x, mut y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center: _, anchor: _, width, height }: &TokenSquareShapeData,
    ) -> GridOffset3D {
        x += self.size as f64 * if width.fract() == 0.0 { 0.5 } else { 0.25 };
        y += self.size as f64 * if height.fract() == 0.0 { 0.5 } else { 0.25 };

        GridOffset3D {
            i: (y / self.size as f64).floor() as i32,
            j: (x / self.size as f64).floor() as i32,
            k: ((elevation / self.distance) + 1e-8).floor() as i32,
        }
    }

    fn get_offset_center_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: (j as f64 + 0.5) * self.size as f64,
            y: (i as f64 + 0.5) * self.size as f64,
            elevation: (k as f64 + 0.5) * self.distance,
        }
    }

    fn get_offset_top_left_point(&self, GridOffset3D { i, j, k }: GridOffset3D) -> ElevatedPoint {
        ElevatedPoint {
            x: j as f64 * self.size as f64,
            y: i as f64 * self.size as f64,
            elevation: k as f64 * self.distance,
        }
    }

    fn get_token_shape(&self, width: f64, height: f64, _shape: TokenShapeType) -> TokenSquareShapeData {
        let mut offsets = Vec::<GridOffset2D>::new();
        let width = (width * 2.0).round() / 2.0;
        let height = (height * 2.0).round() / 2.0;

        for i in 0..(height.ceil() as i32) {
            for j in 0..(width.ceil() as i32) {
                offsets.push(GridOffset2D { i, j });
            }
        }

        TokenSquareShapeData {
            offsets,
            points: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: width, y: 0.0 },
                Point { x: width, y: height },
                Point { x: 0.0, y: height },
            ],
            center: Point { x: width / 2.0, y: height / 2.0 },
            anchor: Point { x: 0.0, y: 0.0 },
            width,
            height,
        }
    }

    fn get_token_center_point(
        &self,
        ElevatedPoint { x, y, elevation }: ElevatedPoint,
        TokenSquareShapeData { offsets: _, points: _, center, anchor: _, width: _, height: _ }: &TokenSquareShapeData,
    ) -> ElevatedPoint {
        ElevatedPoint { x: x + (center.x * self.size as f64), y: y + (center.y * self.size as f64), elevation }
    }
}

impl AStar for SquareGrid {
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
