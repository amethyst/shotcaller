use crate::*;

/// Moves the entity with A-move order towards the destination, but if enemy entity is in aggresion range, then moves towards enemy instead.
pub fn amove_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &Components<StatSet<Stats>>,
    order_queue: &Components<OrderQueue>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MicroInput => {}
    }

    for (e, orders, pos, team) in join!(&entities && &order_queue && &positions && &teams) {
        // Current order is a-move
        let oq = orders.unwrap();
        if oq.orders.len() > 0 {
            if let UnitOrder::AMovetoPoint(trg_pt) = oq.orders[0] {
                // find aggro range:
                let aggro_range = stats
                    .get(e.unwrap())
                    .unwrap()
                    .stats
                    .get(&Stats::AggroRange)
                    .unwrap()
                    .value
                    .clone() as f32;

                // decide between violence and travel:
                let closest = find_closest_in_other_team(
                    team.unwrap(),
                    pos.unwrap(),
                    &teams,
                    &positions,
                    &stats,
                    &entities,
                );

                //
                let mut new_target = trg_pt;

                // In theory this could stop working if
                if let Some((_, c)) = closest {
                    if dist(&c, pos.unwrap()) <= aggro_range {
                        new_target = c.clone();
                    }
                }

                // Just update target every frame. If actual point doesn;t change pathfinding will not rerun
                targets.insert(e.unwrap(), AiDestination::new(new_target));
            }
        }
    }

    Ok(())
}
