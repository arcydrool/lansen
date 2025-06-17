<script setup lang="ts">
import Card from 'primevue/card';
import { RouterLink } from 'vue-router';
</script>

<template>
  <Card>
    <template #title>Custom Injection Molds, Engineering:</template>
    <template #content>
      <h1>
        Custom Injection molder supplying
        the plastics industry with cost effective solutions to molding and tooling needs.</h1>
      <h2>Summary of services provided at Lansen Mold:</h2>
      <ul>
        <li>Over 65 years experience, Lansen Mold was founded in 1960</li>
        <li>Project management, Design through packaged product.</li>
        <li>Mold manufacturing, prototyping, evaluation, and production tooling.</li>
      </ul>
      <h2>Injection Molding:</h2>
      <ul>
        <li>Over 65 years experience</li>
        <li>Injection molders ranging from 38 tons to 360 tons.</li>
        <li>Secondary operations, printing, machining etc.</li>
        <li>Warehousing of raw materials and finished goods.</li>
      </ul>
      <h2>Specimen and Plaque Molding:</h2>
      <ul>
        <li>Over 45 years experience</li>
        <li>Quick and thorough sampling</li>
        <li>ISO and ASTM standard tooling</li>
        <li>Large selection of specimen tooling</li>
        <li>Tensile Bar</li>
        <li>Flame Bar</li>
        <li>Izod</li>
        <li>Disk</li>
        <li>Tension/Impact</li>
        <li>Plaque molds</li>
        <li>Color Chips</li>
        <li>Flex Fatigue</li>
        <li>Flow Length</li>
      </ul>
      <!-- TODO: PHOTOS
      <img src="photos/molded_fan.jpg" width="228" height="184">

      <img src="photos/1267_b.jpg" width="212" height="143">
      <img src="photos/bar.jpg">
    -->
      
    </template>
  </Card>
  <router-link to="/quote">Click here for specimen &amp; plaque product listing and to process a request for
    quotation.</router-link>
</template>