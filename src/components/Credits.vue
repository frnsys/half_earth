<template>
<div class="credits">
  <div class="credits--inner">
  <h2>Concept</h2>
  <ul>
    <li>Arthur Röing Baer</li>
    <li>Chiara Di Leone</li>
    <li>Drew Pendergrass</li>
    <li>Son La Pham</li>
    <li>Francis Tseng</li>
    <li>Gregory Vettese</li>
    <li>Troy Vettese</li>
  </ul>
  <h2>Design</h2>
  <ul>
    <li>Son La Pham</li>
    <li>Francis Tseng</li>
  </ul>
  <h2>Development</h2>
  <ul>
    <li>Francis Tseng</li>
    <li>Son La Pham</li>
  </ul>
  <h2>Research and Writing</h2>
  <ul>
    <li>Lucy Chinen</li>
    <li>Drew Pendergrass</li>
    <li>Son La Pham</li>
    <li>Spencer Roberts</li>
    <li>Francis Tseng</li>
    <li>Troy Vettese</li>
  </ul>
  <h2>Music</h2>
  <ul>
    <li>PRINCE SHIMA</li>
  </ul>
  <h2>Playtesting</h2>
  <ul>
    <li>Spencer Roberts</li>
    <li>Dargan Frierson</li>
    <li>Sean Raspet</li>
    <li>Sarah Friend</li>
    <li>Filip Mesko</li>
    <li>Wassim Alsindi</li>
    <li>Bradley K</li>
    <li>Julia</li>
    <li>Grace Van de Pas</li>
    <li>Michael Vettese</li>
    <li>Xinyue</li>
    <li>Lukas Eigler-Harding</li>
    <li>Adrian Dinh</li>
    <li>Aural Ephem</li>
    <li>Nick Houde</li>
    <li>Simon Zhang</li>
    <li>Paul Turberg</li>
    <li>Jan Philipp Dapprich</li>
    <li>Matt Goerzen</li>
    <li>Neilson Koerner-Safrata</li>
    <li>Kira Simon-Kennedy</li>
    <li>Nicholas Carter</li>
  </ul>
  <h3 v-if="PLATFORM == 'STEAM'">Based on the book <em>Half-Earth Socialism</em> (Drew Pendergrass &amp; Troy Vettese, Verso 2022).</h3>
  <h3 v-else>Based on the book <em><a href="https://www.versobooks.com/books/3818-half-earth-socialism">Half-Earth Socialism</a></em> (Drew Pendergrass &amp; Troy Vettese, Verso 2022).</h3>
  <h2>Thank you for playing!</h2>

  <button class="btn" @click="$emit('closeCredits')">Back</button>
  </div>
</div>
</template>

<script>
export default {
}
</script>

<style>
.credits {
  text-align: center;
  flex: 1;
  padding: 1em;
  color: #fff;


}
.credits h2 {
  font-weight: normal;
}

.credits--inner{
  border-radius: 2em;
  padding: 1rem;
  background-color: rgba(0,0,0,0.1);

  border-top: 1px solid #000;
  border-left: 1px solid #000;
  border-bottom: 1px solid #888;
  border-right: 1px solid #888;
}

.credits h3 {
  font-weight: normal;
}
.credits h3 a {
  color: #fff;
}

@media only screen and (min-width: 481px) {
  .credits li{
    font-size: 1.2em;
    line-height: 1.5;
  }
}
</style>
