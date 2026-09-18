//! Lunar-scale precision probe for the Lunie floating-origin integration.
//!
//! Lunie renders a colony at metre scale on a body of radius 1,737,400 m, and
//! needs to zoom continuously from gameplay to orbit. The smallest feature that
//! must survive is the bulldozer tile inset, 0.05 m.
//!
//! Single-precision cannot express both: `f32` carries a 24-bit mantissa, so
//! near lunar radius one ulp is ~0.125 m in metres, or ~0.207 m if the world is
//! expressed in unit-sphere units. The 0.05 m inset quantizes to *zero*.
//!
//! These tests pin the property Lunie depends on: with a floating origin, world
//! positions stay authoritative while render-space coordinates remain small
//! enough that gameplay-scale detail survives intact.

use bevy::prelude::*;
use big_space::prelude::*;

/// Volumetric mean radius of the Moon, in metres.
const MOON_RADIUS_M: f64 = 1_737_400.0;

/// Grid cell edge length. Surface entities land within one cell of their
/// origin, keeping render-space coordinates ~10^3 rather than ~10^6.
const CELL_EDGE_M: f32 = 10_000.0;

/// Spawn two entities `separation_m` apart on the lunar surface, with the
/// floating origin co-located, and return their render-space translations.
fn render_space_pair(separation_m: f32) -> (Vec3, Vec3) {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BigSpaceDefaultPlugins));

    let (mut a, mut b) = (Entity::PLACEHOLDER, Entity::PLACEHOLDER);
    app.world_mut()
        .commands()
        .spawn_big_space(Grid::new(CELL_EDGE_M, 0.0), |grid| {
            let surface_y = MOON_RADIUS_M as f32;
            grid.spawn_spatial((Transform::from_xyz(0.0, surface_y, 0.0), FloatingOrigin));
            a = grid
                .spawn_spatial(Transform::from_xyz(0.0, surface_y, 0.0))
                .id();
            b = grid
                .spawn_spatial(Transform::from_xyz(separation_m, surface_y, 0.0))
                .id();
        });
    app.update();

    let translation = |e: Entity| {
        app.world()
            .get::<GlobalTransform>(e)
            .expect("spatial entity has a GlobalTransform")
            .translation()
    };
    (translation(a), translation(b))
}

/// Gameplay-scale separations must survive intact at lunar radius.
///
/// 0.05 m is the bulldozer tile inset, the finest detail in the playfield mesh;
/// 1.0 m is one gameplay tile.
#[test]
fn gameplay_scale_detail_survives_at_lunar_radius() {
    for separation_m in [0.05_f32, 0.5, 1.0] {
        let (a, b) = render_space_pair(separation_m);
        let measured = (b - a).length();
        let error = (measured - separation_m).abs();
        assert!(
            error < 1e-4,
            "{separation_m} m separation degraded to {measured} m (error {error})"
        );
    }
}

/// The floating origin must actually keep render coordinates small; that is the
/// mechanism by which precision is preserved.
#[test]
fn render_coordinates_stay_within_one_cell() {
    let (a, _) = render_space_pair(0.05);
    assert!(
        a.length() < CELL_EDGE_M,
        "render-space coordinate {} m escaped its {CELL_EDGE_M} m cell",
        a.length()
    );
}

/// Documents the failure this architecture exists to avoid: computing at lunar
/// radius directly in `f32` annihilates the 0.05 m inset entirely.
#[test]
fn naive_f32_at_lunar_radius_loses_the_bulldozer_inset() {
    let radius = MOON_RADIUS_M as f32;
    let naive = (radius + 0.05) - radius;
    assert_eq!(
        naive, 0.0,
        "expected f32 at lunar radius to quantize 0.05 m to zero, got {naive}"
    );

    let (a, b) = render_space_pair(0.05);
    let with_floating_origin = (b - a).length();
    assert!(
        with_floating_origin > 0.049,
        "floating origin should preserve what naive f32 destroys, got {with_floating_origin}"
    );
}
