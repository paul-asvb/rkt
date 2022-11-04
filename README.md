# RKTE

## Development

trunk serve

[bevy core concepts](https://github.com/bevyengine/bevy/blob/v0.8.1/examples/ecs/ecs_guide.rs#L9)

## Tutorial

[bevy](https://bevy-cheatbook.github.io/tutorial.html)
[pysics](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy)

## CI/CD

this repo is generated from https://github.com/NiklasEi/bevy_game_template. check out the repo for details about the setup, build and deployment.

## Multiplayer

https://johanhelsing.studio/posts/extreme-bevy


## todo

add fixed time

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}