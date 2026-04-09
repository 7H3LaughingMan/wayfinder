use crate::types::foundry::{
    WallDirection, WallDoorState, WallDoorType, WallMovementType, WallSenseType, documents::JsWallDocument,
};
use geo::{BoundingRect, Intersects, Line, Rect};

#[derive(Clone, Debug)]
pub struct WallDocument {
    pub id: String,
    c: [f64; 4],
    light: WallSenseType,
    r#move: WallMovementType,
    sight: WallSenseType,
    sound: WallSenseType,
    dir: WallDirection,
    door: WallDoorType,
    ds: WallDoorState,
    line: Line,
    bounds: Rect,
}

impl WallDocument {
    pub fn blocks_movement(&self) -> bool {
        match self.door == WallDoorType::None && self.r#move == WallMovementType::Normal {
            true => true,
            false => match self.door != WallDoorType::None && self.ds != WallDoorState::Open {
                true => true,
                false => false,
            },
        }
    }
}

impl From<JsWallDocument> for WallDocument {
    fn from(value: JsWallDocument) -> Self {
        let c = [
            value.c().get0().value_of(),
            value.c().get1().value_of(),
            value.c().get2().value_of(),
            value.c().get3().value_of(),
        ];

        WallDocument {
            id: value.id(),
            c,
            light: value.light(),
            r#move: value.r#move(),
            sight: value.sight(),
            sound: value.sound(),
            dir: value.dir(),
            door: value.door(),
            ds: value.ds(),
            line: Line::from([(c[0], c[1]), (c[2], c[3])]),
            bounds: Rect::new((c[0], c[1]), (c[2], c[3])),
        }
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

impl Intersects<Vec<Line>> for WallDocument {
    fn intersects(&self, rhs: &Vec<Line>) -> bool {
        rhs.iter().any(|line| self.intersects(line))
    }
}
