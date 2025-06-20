use crate::{
    exports::{Fog, Walls},
    traits::{BaseGrid, Node, SkipLast, TokenShape},
    types::{GridMeasurePathResult, Rectangle, TokenDocument, TokenMovementWaypoint},
};

pub trait AStar<N: Node + Eq, T: TokenShape>: BaseGrid<N, T> {
    fn find_path(
        &self,
        waypoints: Vec<TokenMovementWaypoint>,
        token: &TokenDocument,
        bounds: &Rectangle,
        walls: &Walls,
        fog: &Option<Fog>,
        grid_measure_path_result: &GridMeasurePathResult,
    ) -> Vec<TokenMovementWaypoint> {
        if waypoints.len() <= 1 {
            return waypoints;
        }

        let mut token_shape = self.get_token_shape(token.width, token.height, token.shape);
        let mut start_waypoint = &waypoints[0];
        let mut start_node = self.get_node(start_waypoint.create_elevated_point(), &token_shape);
        let mut path = vec![start_waypoint.clone()];

        start_node.set_diagonal(grid_measure_path_result.diagonals % 2 != 0);

        for end_waypoint in &waypoints[1..] {
            let end_node = self.get_node(end_waypoint.create_elevated_point(), &token_shape);
            let elevation_range = i32::min(start_node.get_elevation(), end_node.get_elevation())
                ..=(i32::max(start_node.get_elevation(), end_node.get_elevation()));

            if let Some((nodes, _cost)) = pathfinding::prelude::astar(
                &start_node,
                |node| self.get_adjacent_nodes(node, &token_shape, &elevation_range, bounds, walls, fog),
                |node| node.get_distance(&end_node),
                |node| node.at_node(&end_node),
            ) {
                let nodes = self.simplify_path(nodes);

                for node in nodes.iter().skip(1).skip_last() {
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
