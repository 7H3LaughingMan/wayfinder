use crate::types::foundry::{
    EdgeDirection, EdgeSenseType, WallDoorState, WallDoorType, WallMovementType, documents::JsWallDocument,
};
use geo::{BoundingRect, Intersects, Line, Rect};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct WallDocument {
    pub id: String,
    c: [f64; 4],
    levels: HashSet<String>,
    light: EdgeSenseType,
    r#move: WallMovementType,
    sight: EdgeSenseType,
    sound: EdgeSenseType,
    dir: EdgeDirection,
    door: WallDoorType,
    ds: WallDoorState,
    line: Line,
    bounds: Rect,
}

impl WallDocument {
    pub fn new(
        id: String,
        c: [f64; 4],
        levels: HashSet<String>,
        light: EdgeSenseType,
        r#move: WallMovementType,
        sight: EdgeSenseType,
        sound: EdgeSenseType,
        dir: EdgeDirection,
        door: WallDoorType,
        ds: WallDoorState,
    ) -> Self {
        WallDocument {
            id,
            c,
            levels,
            light,
            r#move,
            sight,
            sound,
            dir,
            door,
            ds,
            line: Line::from([(c[0], c[1]), (c[2], c[3])]),
            bounds: Rect::new((c[0], c[1]), (c[2], c[3])),
        }
    }

    pub fn blocks_movement(&self) -> bool {
        match self.door == WallDoorType::None && self.r#move == WallMovementType::Normal {
            true => true,
            false => match self.door != WallDoorType::None && self.ds != WallDoorState::Open {
                true => true,
                false => false,
            },
        }
    }

    pub fn included_in_level(&self, level: &str) -> bool {
        if self.levels.len() == 0 { true } else { self.levels.contains(level) }
    }
}

impl From<JsWallDocument> for WallDocument {
    fn from(value: JsWallDocument) -> Self {
        WallDocument::new(
            value.id(),
            [
                value.c().get0().value_of(),
                value.c().get1().value_of(),
                value.c().get2().value_of(),
                value.c().get3().value_of(),
            ],
            value.levels().values().into_iter().flatten().map(String::from).collect(),
            value.light(),
            value.r#move(),
            value.sight(),
            value.sound(),
            value.dir(),
            value.door(),
            value.ds(),
        )
    }
}

impl BoundingRect<f64> for WallDocument {
    type Output = Rect;

    fn bounding_rect(&self) -> Self::Output {
        self.bounds
    }
}

impl Intersects<Line> for WallDocument {
    fn intersects(&self, rhs: &Line) -> bool {
        self.line.intersects(rhs)
    }
}
