use ambient_api::{
    core::{
        camera::{
            components::{fog, fovy},
            concepts::{
                PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
            },
        },
        messages::Frame,
        transform::components::{lookat_target, lookat_up, rotation, translation},
    },
    prelude::*,
};
use packages::tangent_schema::{player::components as pc, vehicle::components as vc};

#[main]
pub fn main() {
    let camera_id = PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            translation: Some(vec3(0., 0., 10.)),
            main_scene: Some(()),
            aspect_ratio_from_window: Some(entity::resources()),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    // .with(fog(), ())
    .with(lookat_target(), vec3(0., 0., 0.))
    .with(lookat_up(), vec3(0., -1., 0.))
    .spawn();

    Frame::subscribe(move |_| {
        let player_id = player::get_local();
        let Some(vehicle_id) = entity::get_component(player_id, pc::vehicle_ref()) else {
            return;
        };
        let Some(vehicle_position) = entity::get_component(vehicle_id, translation()) else {
            return;
        };
        let Some(vehicle_rotation) = entity::get_component(vehicle_id, rotation()) else {
            return;
        };
        let Some(vehicle_speed_kph) = entity::get_component(vehicle_id, vc::speed_kph()) else {
            return;
        };

        let vehicle_yaw = vehicle_rotation.to_euler(glam::EulerRot::ZYX).0;

        entity::set_component(camera_id, translation(), vehicle_position + 20.0 * Vec3::Z);
        entity::set_component(camera_id, lookat_target(), vehicle_position);
        entity::set_component(
            camera_id,
            lookat_up(),
            Quat::from_rotation_z(vehicle_yaw) * -Vec3::Y,
        );
        entity::set_component(
            camera_id,
            fovy(),
            0.9 + (vehicle_speed_kph.abs() / 300.0).clamp(0.0, 1.0),
        );
    });
}
