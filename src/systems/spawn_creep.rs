use crate::*;

/// Spawns a creep using the provided event.
pub fn spawn_creep_system(
    game_events: &Vec<GameEvent>,
    stat_def: &StatDefinitions<Stats>,
    entities: &mut Entities,
    positions: &mut Components<Point>,
    creeps: &mut Components<Creep>,
    order_queue: &mut Components<OrderQueue>,
    simple_movements: &mut Components<SimpleMovement>,
    proximity_attacks: &mut Components<ProximityAttack>,
    stats: &mut Components<StatSet<Stats>>,
    teams: &mut Components<Team>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    sights: &mut Components<LineOfSight>,
    uuids: &mut Components<Uuid>,
) -> SystemResult {
    for ev in game_events.iter() {
        if let GameEvent::SpawnCreep(pos, team, uuid_opt) = ev {
            let creep = entities.create();
            positions.insert(creep, pos.clone());
            creeps.insert(creep, Creep);
            simple_movements.insert(creep, SimpleMovement);
            teams.insert(creep, *team);
            stats.insert(creep, stat_def.to_statset());
            proximity_attacks.insert(creep, ProximityAttack::new(CREEP_ATTACK_RADIUS));
            let bg = if *team == Team::Me {
                sights.insert(creep, LineOfSight::new(5));
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };

            // Spawn with Hold position order. To stop leaders when game mode is changed to micro-input.
            // order_queue.insert(creep, OrderQueue::new(vec![UnitOrder::HoldPosition]));
            order_queue.insert(creep, OrderQueue::new());

            sprites.insert(
                creep,
                Sprite {
                    glyph: to_cp437('c'),
                    fg: RGBA::named(YELLOW),
                    bg,
                },
            );
            sprite_indices.insert(creep, SpriteIndex(TileMapping::Creep.into()));
            if let Some(uuid) = uuid_opt {
                uuids.insert(creep, uuid.clone());
            } else {
                uuids.insert(creep, Uuid::new_v4());
            }
        }
    }
    Ok(())
}
