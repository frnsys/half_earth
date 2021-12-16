// Ensure these match effects.rs
function demandOutlookChange(world, output, amount) {
  return (world.regions.reduce((acc, r) => acc + r.demand_levels[output], 0) * amount)/world.regions.length;
}

function incomeOutlookChange(world, amount) {
  return (world.regions.reduce((acc, r) => acc + r.income_level, 0) * amount)/world.regions.length;
}

export default {demandOutlookChange, incomeOutlookChange};
