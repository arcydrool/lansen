<script setup lang="ts">
import Card from 'primevue/card';
import { RouterLink } from 'vue-router';
import Image from 'primevue/image';
import i1267b from '../assets/1267_b.jpg';
import ibar from '../assets/bar.jpg';
import imfan from '../assets/molded_fan.jpg';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
</script>

<template>
  <h1>Custom Injection Molds, Engineering:</h1>
  <h2>Custom Injection molder supplying
    the plastics industry with cost effective solutions to molding and tooling needs.</h2>
  <Card>
    <template #title>Summary of services provided at Lansen Mold:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Over 65 years experience, Lansen Mold was founded in 1960</li>
            <li>Project management, Design through packaged product.</li>
            <li>Mold manufacturing, prototyping, evaluation, and production tooling.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="imfan" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
  <Card>
    <template #title>Injection Molding:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Over 65 years experience</li>
            <li>Injection molders ranging from 38 tons to 360 tons.</li>
            <li>Secondary operations, printing, machining etc.</li>
            <li>Warehousing of raw materials and finished goods.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="i1267b" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
  <Card>
    <template #title>Specimen and Plaque Molding:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
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
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="ibar" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
  <router-link to="/quote">Click here for specimen &amp; plaque product listing and to process a request for
    quotation.</router-link>
</template>