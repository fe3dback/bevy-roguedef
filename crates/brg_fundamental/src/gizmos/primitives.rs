use std::f32::consts::PI;

use bevy::math::EulerRot;
use bevy::prelude::{Color, Isometry3d, Quat};
use brg_core::prelude::{BlockPosition, Range, Tile, V2, V3};

use super::sup::{GizmosX, ISO_IDEN};

impl GizmosX<'_, '_> {
    #[inline]
    fn iso(&mut self, center: V2) -> Isometry3d {
        let mut iso = ISO_IDEN;
        iso.translation = center
            .as_3d()
            .with_y(self.heightmap.height_at_pos(center))
            .into();
        iso
    }

    #[inline]
    fn iso3d(&mut self, center: V3) -> Isometry3d {
        let mut iso = ISO_IDEN;
        iso.translation = center.as_3d().into();
        iso.translation.y += self.heightmap.height_at_pos(V2::new(center.x, center.y));
        iso
    }

    #[inline]
    pub fn rect_range<C: Into<Color>>(&mut self, r: Range<Tile>, color: C) {
        self.rect(r.position_tl(), r.size_m(), color);
    }

    #[inline]
    pub fn rect<C: Into<Color>>(&mut self, tl: V2, size: V2, color: C) {
        let iso = self.iso(tl + (size * 0.5));
        self.gz.rect(iso, size.as_2d(), color);
    }

    #[inline]
    pub fn circle_custom_height<C: Into<Color> + Copy>(
        &mut self,
        center: V3,
        radius: f32,
        color: C,
    ) {
        let mut iso_center = self.iso3d(center);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        self.gz.circle(iso_center, radius, color);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);
        self.gz.circle(iso_center, radius, color);

        iso_center.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0);
        self.gz.circle(iso_center, radius, color);
    }

    #[inline]
    pub fn capsule<C: Into<Color> + Copy>(
        &mut self,
        center: V2,
        radius: f32,
        height: f32,
        color: C,
    ) {
        self.capsule_custom_height(center.with_height(0.0), radius, height, color);
    }

    #[inline]
    pub fn capsule_custom_height<C: Into<Color> + Copy>(
        &mut self,
        center: V3,
        radius: f32,
        height: f32,
        color: C,
    ) {
        let half_height: f32 = height * 0.5;
        let z = V3::new(0.0, 0.0, half_height);

        let mut iso_bottom = self.iso3d(center - z);
        iso_bottom.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);

        let mut iso_top = self.iso3d(center + z);
        iso_top.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0);

        self.gz.circle(iso_bottom, radius, color);
        self.gz.circle(iso_top, radius, color);

        let len: f32 = radius * 0.5;

        let (x, y) = (V3::new(len, 0.0, 0.0), V3::new(0.0, len, 0.0));

        self.line_custom_height((center - x) - z, (center - x) + z, color);
        self.line_custom_height((center + x) - z, (center + x) + z, color);
        self.line_custom_height((center - y) - z, (center - y) + z, color);
        self.line_custom_height((center + y) - z, (center + y) + z, color);
    }

    #[inline]
    pub fn line<C: Into<Color>>(&mut self, p1: V2, p2: V2, color: C) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.line(iso1, iso2, color)
    }

    #[inline]
    pub fn line_custom_height<C: Into<Color>>(&mut self, p1: V3, p2: V3, color: C) {
        let iso1 = self.iso3d(p1).translation.into();
        let iso2 = self.iso3d(p2).translation.into();

        self.gz.line(iso1, iso2, color)
    }

    #[inline]
    pub fn line_gradient<C: Into<Color>>(&mut self, p1: V2, p2: V2, color1: C, color2: C) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.line_gradient(iso1, iso2, color1, color2)
    }

    #[inline]
    pub fn arrow<C: Into<Color>>(&mut self, p1: V2, p2: V2, color: C) {
        let iso1 = self.iso(p1).translation.into();
        let iso2 = self.iso(p2).translation.into();

        self.gz.arrow(iso1, iso2, color);
    }

    #[inline]
    pub fn arrow_custom_height<C: Into<Color>>(&mut self, p1: V3, p2: V3, color: C) {
        let iso1 = self.iso3d(p1).translation.into();
        let iso2 = self.iso3d(p2).translation.into();

        self.gz.arrow(iso1, iso2, color);
    }

    #[inline]
    pub fn point_custom_height<C: Into<Color> + Copy>(&mut self, pos: V3, color: C) {
        const LEN: f32 = 0.1;

        let (x, y, z) = (
            V3::new(LEN, 0.0, 0.0),
            V3::new(0.0, LEN, 0.0),
            V3::new(0.0, 0.0, LEN),
        );

        self.line_custom_height(pos - x, pos + x, color);
        self.line_custom_height(pos - y, pos + y, color);
        self.line_custom_height(pos - z, pos + z, color);
    }
}
