<script setup lang="ts">
import Card from 'primevue/card';
import Image from 'primevue/image';
import Ispec from '@/assets/spec1.jpg';
import Inquirer from '@/components/Inquirer.vue';
import GoForm from '@/components/GoForm.vue';
</script>
<template>
  <h1>Specimen Molding</h1>
  <Card>
    <template #title>Specimen molding for the testing industry:</template>
    <template #content>
      <ul>
        <li>We provide quick and thorough services for
          the testing industry. Our standard delivery is two to three weeks. Quicker deliveries
          maybe available after a project review.</li>
        <Inquirer />
        <li>ISO and ASTM family molds are balanced per
          standards on mold design ISO 294 and ASTM D3641.</li>
        <li>Pricing starting at $390.00, includes 2 hours
          of processing, tool setup, material handling and completed ?as molded? process
          sheet for each material type.</li>
        <li>Over 45 years experience in molding test specimens.</li>
        <li>Our injection molding machines and inspection equipment are calibrated and
          certified yearly.</li>
        <li>Complete in house tooling facilities for special
          mold manufacturing and special machining services.</li>
        <li>If you need specimen testing we will direct
          you to a qualified testing laboratory.</li>
      </ul>
      <Image :src="Ispec" alt="Lansen Mold offers a variety of products."/>
    </template>
  </Card>
  <GoForm />
</template>