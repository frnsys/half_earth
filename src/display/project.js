import PROJECTS from '/assets/content/projects.json';

function activeEffects(project) {
  let details = PROJECTS[project.id];
  let activeOutcomeEffects = project.active_outcome == null ? [] : details.outcomes[project.active_outcome].effects;
  if (project.kind == 'Policy' && project.status !== 'Active') {
    // Project outcome effects are secret and delayed
    return details.effects;
  } else if (project.status == 'Inactive') {
    return details.effects.concat(outcomeEffects(details));
  } else if (project.status == 'Building') {
    return details.effects.concat(outcomeEffects(details));
  } else if (project.level === 0) {
    return details.effects.concat(activeOutcomeEffects);
  } else {
    return details.upgrades[project.level - 1].effects.concat(activeOutcomeEffects);
  }
}

function outcomeEffects(projectDetails) {
  let allEffects = {};
  projectDetails.outcomes.forEach(({effects, probability}) => {
    for (const effect of effects) {
      effect.probability = probability;
      let key = `${effect.type}${effect.subtype ? effect.subtype : ''}`;
      let hash = JSON.stringify(effect);
      if (!(key in allEffects)) {
        allEffects[key] = {
          effect,
          count: 1,
          hashes: new Set([hash]),
        };
      } else {
        allEffects[key].count += 1;
        allEffects[key].hashes.add(hash);
      }
    }
  });

  return Object.values(allEffects).map(({effect, count, hashes}) => {
    effect.random = count !== projectDetails.outcomes.length || hashes.size > 1;
    if (hashes.size > 1) effect.param = '?';
    return effect;
  });
}

export {activeEffects};
