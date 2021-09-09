use nannou::prelude::*;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct Segment {
    pub a: Point2,
    pub b: Point2,
}

impl Segment {
    pub fn new(a: Point2, b: Point2) -> Self {
        Segment { a, b }
    }

    pub fn transform(&self, t: Affine2) -> Self {
        Segment {
            a: t.transform_point2(self.a),
            b: t.transform_point2(self.b),
        }
    }

    pub fn direction(&self) -> Vec2 {
        (self.b - self.a).normalize()
    }

    pub fn length(&self) -> f32 {
        (self.b - self.a).length()
    }

    pub fn to_ray(&self) -> Ray {
        Ray {
            origin: self.a,
            direction: self.direction(),
        }
    }
}

pub struct Ray {
    origin: Point2,
    direction: Vec2,
}

pub struct RayIntersection {
    pub point: Point2,
    pub distance: f32,
}

impl Ray {
    fn new(origin: Point2, direction: Vec2) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    fn from_points(a: Point2, b: Point2) -> Self {
        Ray {
            origin: a,
            direction: (b - a).normalize(),
        }
    }

    fn is_valid(&self) -> bool {
        self.direction.is_normalized()
    }

    fn intersect(&self, segment: &Segment) -> Option<RayIntersection> {
        let t = {
            let (na, nb) = (self.origin - segment.a, segment.a - segment.b);
            let (da, db) = (-self.direction, segment.a - segment.b);
            let numerator = Mat2::from_cols(na, nb).determinant();
            let denominator = Mat2::from_cols(da, db).determinant();
            if denominator == 0.0 {
                return None;
            } else {
                numerator / denominator
            }
        };
        let u = {
            let (na, nb) = (self.direction, self.origin - segment.a);
            let (da, db) = (-self.direction, segment.a - segment.b);
            let numerator = Mat2::from_cols(na, nb).determinant();
            let denominator = Mat2::from_cols(da, db).determinant();
            if denominator == 0.0 {
                return None;
            } else {
                numerator / denominator
            }
        };
        if (0.0..=1.0).contains(&u) && t >= 0.0 {
            Some(RayIntersection {
                point: self.origin + t * self.direction,
                distance: t,
            })
        } else {
            None
        }
    }

    pub fn intersect_first(&self, segments: &[Segment]) -> Option<RayIntersection> {
        segments
            .iter()
            .filter_map(|s| self.intersect(&s))
            .min_by(|ia, ib| {
                ia.distance
                    .partial_cmp(&ib.distance)
                    .unwrap_or(Ordering::Greater)
            })
    }
}
