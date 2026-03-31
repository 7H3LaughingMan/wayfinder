use crate::types::pixi::JsRoundedRectangle;

#[derive(Clone, Copy, Debug)]
pub struct RoundedRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub radius: f64,
}

impl RoundedRectangle {
    pub fn new(
        x: impl Into<f64>,
        y: impl Into<f64>,
        width: impl Into<f64>,
        height: impl Into<f64>,
        radius: impl Into<f64>,
    ) -> Self {
        RoundedRectangle { x: x.into(), y: y.into(), width: width.into(), height: height.into(), radius: radius.into() }
    }

    pub fn contains(&self, x: impl Into<f64>, y: impl Into<f64>) -> bool {
        let x: f64 = x.into();
        let y: f64 = y.into();

        if self.width <= 0.0 || self.height <= 0.0 {
            return false;
        }

        if x >= self.x && x <= self.x + self.width {
            if y >= self.y && y <= self.y + self.height {
                let radius = f64::max(0.0, f64::min(self.radius, f64::min(self.width, self.height) / 2.0));

                if (y >= self.y + radius && y <= self.y + self.height - radius)
                    || (x >= self.x + radius && x <= self.x + self.width - radius)
                {
                    return true;
                }

                let mut dx = x - (self.x + radius);
                let mut dy = y - (self.y + radius);
                let radius2 = radius * radius;

                if dx * dx + dy * dy <= radius2 {
                    return true;
                }

                dx = x - (self.x + self.width - radius);
                if dx * dx + dy * dy <= radius2 {
                    return true;
                }

                dy = y - (self.y + self.height - radius);
                if dx * dx + dy * dy <= radius2 {
                    return true;
                }

                dx = x - (self.x + radius);
                if dx * dx + dy * dy <= radius2 {
                    return true;
                }
            }
        }

        return false;
    }
}

impl From<JsRoundedRectangle> for RoundedRectangle {
    fn from(value: JsRoundedRectangle) -> Self {
        RoundedRectangle::new(value.x(), value.y(), value.width(), value.height(), value.radius())
    }
}
