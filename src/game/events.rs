use bevy::prelude::*;
use bevy_ui_anchor::{
    AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor,
};

use crate::{
    components::{
        Collider, Fade, FadeEffect, ForStates, Goal, HitPoints, POLE_DIAMETER,
        POLE_HEIGHT, Pole, RemoveBeforeFadeOut, Team, WinningTeam,
    },
    game::GOAL_WIDTH,
};

use super::{CachedAssets, GameAssets, GameState, PlayableSet};

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>()
            .add_event::<SpawnUiMessage>()
            .add_event::<GoalScoredEvent>()
            .add_observer(spawn_pole_in_a_goal)
            .add_observer(spawn_ui_message)
            .add_systems(
                PostUpdate,
                spawn_poles_for_eliminated_goals.after(PlayableSet),
            )
            .add_systems(
                PostUpdate,
                decrement_hp_when_goal_gets_scored.in_set(PlayableSet),
            )
            .add_systems(Last, check_for_winning_team.in_set(PlayableSet));
    }
}

/// Signal when a [`Goal`] entity has been scored by a ball.
#[derive(Clone, Debug, Event)]
pub struct GoalScoredEvent(pub Entity);

/// Signals that a [`Goal`] has been eliminated from the game.
#[derive(Clone, Debug, Event)]
pub struct GoalEliminatedEvent(pub Entity);

/// An event fired to spawn a [`Pole`] in a [`Goal`].
#[derive(Debug, Event)]
pub struct SpawnPole {
    pub goal_entity: Entity,
    pub fade_in: bool,
}

/// An event fired when spawning a message UI.
#[derive(Debug, Event)]
pub struct SpawnUiMessage {
    pub message: String,
    pub game_state: GameState,
}

fn spawn_pole_in_a_goal(
    trigger: Trigger<SpawnPole>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
) {
    let SpawnPole {
        goal_entity,
        fade_in,
    } = trigger.event();

    let id = commands
        .entity(*goal_entity)
        .with_children(|builder| {
            builder.spawn((
                Pole,
                Collider,
                RemoveBeforeFadeOut::<Collider>::default(),
                if *fade_in {
                    Fade::new_in()
                } else {
                    Fade::In(Timer::default()) // Skip to end of animation.
                },
                FadeEffect::Scale {
                    max_scale: Vec3::new(
                        POLE_DIAMETER,
                        GOAL_WIDTH,
                        POLE_DIAMETER,
                    ),
                    axis_mask: Vec3::new(1.0, 0.0, 1.0),
                },
                Mesh3d(cached_assets.pole_mesh.clone()),
                MeshMaterial3d(cached_assets.pole_material.clone()),
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(f32::EPSILON),
                    Quat::from_euler(
                        EulerRot::XYZ,
                        0.0,
                        0.0,
                        std::f32::consts::FRAC_PI_2,
                    ),
                    Vec3::new(0.0, POLE_HEIGHT, 0.0),
                )),
            ));
        })
        .id();

    info!("Pole({id:?}): Spawned");
}

fn spawn_ui_message(
    trigger: Trigger<SpawnUiMessage>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
) {
    let SpawnUiMessage {
        message,
        game_state,
    } = trigger.event();

    commands.spawn((
        ForStates(vec![*game_state]),
        AnchorUiNode {
            target: AnchorTarget::Translation(Vec3::ZERO),
            offset: None,
            anchorwidth: HorizontalAnchor::Mid,
            anchorheight: VerticalAnchor::Mid,
        },
        Text(message.clone()),
        TextFont {
            font: game_assets.font_menu.clone(),
            font_size: 25.0,
            ..default()
        },
        TextColor(Srgba::RED.into()),
    ));
}

fn spawn_poles_for_eliminated_goals(
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut commands: Commands,
) {
    for GoalEliminatedEvent(goal_entity) in goal_eliminated_events.read() {
        commands.trigger(SpawnPole {
            goal_entity: *goal_entity,
            fade_in: true,
        });
    }
}

fn decrement_hp_when_goal_gets_scored(
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    mut hp_query: Query<&mut HitPoints, With<Goal>>,
) {
    // Decrement a goal's HP and potentially eliminate it.
    for GoalScoredEvent(goal_entity) in goal_scored_events.read() {
        let Ok(mut hp) = hp_query.get_mut(*goal_entity) else {
            continue;
        };

        hp.0 = hp.0.saturating_sub(1);

        if hp.0 == 0 {
            goal_eliminated_events.send(GoalEliminatedEvent(*goal_entity));
            info!("Goal({goal_entity:?}): Eliminated");
        }
    }
}

fn check_for_winning_team(
    mut commands: Commands,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    teams_query: Query<(&Team, &HitPoints), With<Goal>>,
) {
    for GoalEliminatedEvent(_) in goal_eliminated_events.read() {
        let mut winning_team = None;
        let survivor = teams_query.iter().find(|(_, hp)| hp.0 > 0);

        if let Some((survivor_team, _)) = survivor {
            let is_winning_team = teams_query
                .iter()
                .all(|(team, hp)| team.0 == survivor_team.0 || hp.0 == 0);

            if is_winning_team {
                winning_team = Some(survivor_team.0);
            }
        } else {
            // Nobody survived. It's a draw!
            winning_team = Some(0);
        }

        if let Some(winning_team) = winning_team {
            commands.insert_resource(WinningTeam(winning_team));
            next_game_state.set(GameState::StartMenu);
            info!("Game Over: Team {winning_team:?} won!");
            break;
        }
    }
}
