use bevy::prelude::{Color, Isometry3d};

use super::sup::{GizmosX, ISO_IDEN};
use crate::prelude::{Range, V2, V3};

impl GizmosX<'_, '_> {
    #[inline]
    fn iso(&self, center: V2) -> Isometry3d {
        let mut iso = ISO_IDEN;
        iso.translation = center
            .as_3d()
            .with_y(self.heightmap.height_at_pos(center))
            .into();
        iso
    }

    #[inline]
    fn iso3d(&self, center: V3) -> Isometry3d {
        let mut iso = ISO_IDEN;
        iso.translation = center.as_3d().into();
        iso.translation.y += self.heightmap.height_at_pos(V2::new(center.x, center.y));
        iso
    }

    #[inline]
    pub fn rect_range<C: Into<Color>>(&mut self, r: Range, color: C) {
        self.rect(r.position(), r.size(), color);
    }

    #[inline]
    pub fn rect<C: Into<Color>>(&mut self, tl: V2, size: V2, color: C) {
        self.gz
            .rect(self.iso(tl + (size * 0.5)), size.as_2d(), color);
    }

    #[inline]
    pub fn circle<C: Into<Color>>(&mut self, center: V2, radius: f32, color: C) {
        self.gz.circle(self.iso(center), radius, color);
    }

    #[inline]
    pub fn line<C: Into<Color>>(&mut self, p1: V2, p2: V2, color: C) {
        self.gz.line(
            self.iso(p1).translation.into(),
            self.iso(p2).translation.into(),
            color,
        )
    }

    #[inline]
    pub fn line_custom_height<C: Into<Color>>(&mut self, p1: V3, p2: V3, color: C) {
        self.gz.line(
            self.iso3d(p1).translation.into(),
            self.iso3d(p2).translation.into(),
            color,
        )
    }

    #[inline]
    pub fn line_gradient<C: Into<Color>>(&mut self, p1: V2, p2: V2, color1: C, color2: C) {
        self.gz.line_gradient(
            self.iso(p1).translation.into(),
            self.iso(p2).translation.into(),
            color1,
            color2,
        )
    }

    #[inline]
    pub fn arrow<C: Into<Color>>(&mut self, p1: V2, p2: V2, color: C) {
        self.gz.arrow(
            self.iso(p1).translation.into(),
            self.iso(p2).translation.into(),
            color,
        );
    }

    #[inline]
    pub fn arrow_custom_height<C: Into<Color>>(&mut self, p1: V3, p2: V3, color: C) {
        self.gz.arrow(
            self.iso3d(p1).translation.into(),
            self.iso3d(p2).translation.into(),
            color,
        );
    }

    #[inline]
    pub fn point<C: Into<Color> + Copy>(&mut self, pos: V3, color: C) {
        let pos2 = V2::new(pos.x, pos.y);
        let size = V2::splat(0.1);

        self.rect(pos2 - (size * 0.5), size, color);
        self.line_custom_height(
            V3::new(pos.x, pos.y, 0.0),
            V3::new(pos.x, pos.y, pos.h),
            color,
        );
    }
}
