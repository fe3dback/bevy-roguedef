use std::f32::consts::PI;

use bevy::math::EulerRot;
use bevy::prelude::{Color, Isometry3d, Quat};
use brg_core::prelude::{BlockPosition, Range, Tile, V2, V3};

use super::point::{Point, PointCoord};
use super::sup::{SupGizmos, ISO_IDEN};

impl SupGizmos<'_, '_> {
    #[inline]
    fn iso<T: PointCoord>(&mut self, center: Point<T>) -> Isometry3d {
        let mut iso = ISO_IDEN;

        iso.translation = match center {
            Point::Abs(p) => p.xyh().as_3d().into(),
            Point::Rel(p) => {
                let p2d = p.xyh().xy();
                let mut p = p.xyh();
                p.h = self.heightmap.height_at_pos(p2d);
                p.as_3d().into()
            }
        };

        iso
    }

    #[inline]
    pub fn rect_range<C: Into<Color> + Copy>(&mut self, r: Range<Tile>, color: C) {
        self.rect(Point::Rel(r.position_tl()), r.size_m(), color);
    }

    #[inline]
    pub fn rect<C: Into<Color> + Copy, T: PointCoord + Copy>(
        &mut self,
        tl: Point<T>,
        size: V2,
        color: C,
    ) {
        let tr = tl + V3::new(size.x, 0.0, 0.0);
        let bl = tl + V3::new(0.0, size.y, 0.0);
        let br = tl + V3::new(size.x, size.y, 0.0);

        self.line(tl, tr, color);
        self.line(tr, br, color);
        self.line(br, bl, color);
        self.line(bl, tl, color);
    }

    #[inline]
    pub fn circle<C: Into<Color> + Copy, T: PointCoord>(
        &mut self,
        center: Point<T>,
        radius: f32,
        color: C,
    ) {
        let mut iso_center = self.iso(center);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        self.gz.circle(iso_center, radius, color);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);
        self.gz.circle(iso_center, radius, color);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0);
        self.gz.circle(iso_center, radius, color);
    }

    #[inline]
    pub fn capsule<C: Into<Color> + Copy, T: PointCoord + Copy>(
        &mut self,
        center: Point<T>,
        radius: f32,
        height: f32,
        color: C,
    ) {
        let half_height: f32 = height * 0.5;
        let z = V3::new(0.0, 0.0, half_height);

        let mut iso_bottom = self.iso(center - z);
        iso_bottom.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);

        let mut iso_top = self.iso(center + z);
        iso_top.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);

        self.gz.circle(iso_bottom, radius, color);
        self.gz.circle(iso_top, radius, color);

        let len: f32 = radius * 0.5;

        let (x, y) = (V3::new(len, 0.0, 0.0), V3::new(0.0, len, 0.0));

        self.line((center - x) - z, (center - x) + z, color);
        self.line((center + x) - z, (center + x) + z, color);
        self.line((center - y) - z, (center - y) + z, color);
        self.line((center + y) - z, (center + y) + z, color);
    }

    #[inline]
    pub fn line<C: Into<Color>, T: PointCoord>(&mut self, p1: Point<T>, p2: Point<T>, color: C) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.line(iso1, iso2, color)
    }

    #[inline]
    pub fn line_gradient<C: Into<Color>, T: PointCoord>(
        &mut self,
        p1: Point<T>,
        p2: Point<T>,
        color1: C,
        color2: C,
    ) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.line_gradient(iso1, iso2, color1, color2)
    }

    #[inline]
    pub fn arrow<C: Into<Color>, T: PointCoord>(&mut self, p1: Point<T>, p2: Point<T>, color: C) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.arrow(iso1, iso2, color);
    }

    #[inline]
    pub fn point<C: Into<Color> + Copy, T: PointCoord + Copy>(&mut self, pos: Point<T>, color: C) {
        const LEN: f32 = 0.1;

        let (x, y, z) = (
            V3::new(LEN, 0.0, 0.0),
            V3::new(0.0, LEN, 0.0),
            V3::new(0.0, 0.0, LEN),
        );

        self.line(pos - x, pos + x, color);
        self.line(pos - y, pos + y, color);
        self.line(pos - z, pos + z, color);
    }
}
