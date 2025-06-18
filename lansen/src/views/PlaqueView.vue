<script setup lang="ts">
import Card from 'primevue/card';
import Inquirer from '@/components/Inquirer.vue';
import Image from 'primevue/image';
import Iplaque from '@/assets/plaque.jpg';
import IDisk from '@/assets/disk.jpg';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';

</script>
<template>
  <h1>Plaque Molding</h1>
  <Card>
    <template #title>Plaque molding for the testing industry:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>We provide quick and thorough services
              for the testing industry. Our standard delivery is two to three weeks.
              Quicker deliveries maybe available after a project review.</li>
            <Inquirer />
            <li>ISO and ASTM family molds are balanced
              per standards on mold design ISO294 and ASTM D3641.</li>
            <li>Pricing starting at $390.00, includes
              2 hours of processing, tool setup, material handling and completed
              ?as molded? process sheet for each material type.</li>
            <li>Over 45 years experience in molding
              test specimens.</li>
            <li>Our injection molding machines and inspection equipment are calibrated
              and certified yearly.</li>
            <li>Complete in house tooling facilities
              for special mold manufacturing and special machining services.</li>
            <li>If you need specimen testing we will
              direct you to a qualified testing laboratory.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="Iplaque" alt="Plastic Plaque" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
  <Card>
    <template #title>Common uses for our Plaque molds include:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Flame Testing</li>
            <li>Rockwell</li>
            <li>Aging programs</li>
            <li>Paint testing</li>
            <li>Chip resistance</li>
            <li>Shrinkage</li>
            <li>Electrical testing</li>
            <li>Water absorption</li>
            <li>Impact testing</li>
            <li>Fogging</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="IDisk" alt="Plastic Disk" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
</template>