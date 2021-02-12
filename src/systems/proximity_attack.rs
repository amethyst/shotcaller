use crate::*;

/// Attacks entities that are in close proximity with this entity.
pub fn proximity_attack_system(
    entities: &Entities,
    proximity_attacks: &Components<ProximityAttackSystems>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut v = vec![];
    for (e, proximity, stat, pos, team) in
        join!(&entities && &proximity_attacks && &stats && &positions && &teams)
    {
        if let ProximityAttackSystems::SimpleProximityAttack(radius) = proximity.unwrap() {
            let closest = find_closest_in_other_team(
                team.unwrap(),
                pos.unwrap(),
                &teams,
                &positions,
                &stats,
                &entities,
            );
            if let Some((target, p)) = closest {
                if dist(&p, pos.unwrap()) <= *radius {
                    let damage = stat.unwrap().stats.get(&Stats::Attack).unwrap().value;
                    v.push((e.unwrap().clone(), target.clone(), damage));
                }
            }
        }
    }

    for (attacker, target, dmg) in v.into_iter() {
        increment_attacks_dealt(&mut stats.get_mut(attacker).unwrap());
        increment_attacks_received(&mut stats.get_mut(target).unwrap());
        game_events.push(GameEvent::DamageEntity(attacker, target, dmg));
    }
    Ok(())
}
