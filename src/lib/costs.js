function nearestMultiple(v, base) {
  return base * Math.round(v/base);
}

function nextPointCost(pointsInUse) {
  return Math.round((pointsInUse+1)**1.5);
}

function banProcessCost(p) {
  return Math.max(nearestMultiple(Math.round((100*p.mix_share)**(3/4)), 5), 5);
}

function promoteProcessCost(p) {
  return Math.max(nearestMultiple(Math.round((100*(1-p.mix_share))**(3/4)), 5), 5);
}

export default {
  nextPointCost,
  banProcessCost,
  promoteProcessCost
}
