<script setup lang="ts">
import Card from 'primevue/card';
import Image from 'primevue/image';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import Iinjmold from "../assets/InjectionMoldingUpdate.jpg"
import injmold from "../assets/injmold.jpg"
import IinvSup from "../assets/inv_man.jpg"

</script>
<template>
  <h1>Injection Molding</h1>
  <Card>
    <template #title>Molding Capabilities:</template>
    <template #content>
      <Splitter>
        <SplitterPanel>
          <ul>
            <li>Lansen's injection molding machines range from
              38 to 360 tons with barrel capacities of 3.3 to 40 oz.</li>
            <li>Our molding department is supported by a quality
              assurance program that monitors materials and products from receiving through
              production and warehousing.</li>
            <li>Our injection molding machines and inspection equipment are calibrated and
              certified yearly.</li>
            <li>We provide secondary operations for special
              packaging, hot stamping, sublimating, sonic welding and component assembly.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="Iinjmold" />
        </SplitterPanel>
      </Splitter>
    </template>
  </Card>
  <Card>
    <template #title>Molding Facilities List:</template>
    <template #content>
      <ul>
        <li>360 Ton Haitian, 40oz Shot Capacity</li>
        <li>360 Ton Haitian, 25oz Shot Capacity</li>
        <li>251 Ton Haitian, 20.4oz Shot Capacity</li>
        <li>165 Ton Demag, 12.7oz Shot Capacity</li>
        <li>124 Ton Demag, 12.7oz Shot Capacity</li>
        <li>110 Ton Niigata, 10.6oz Shot Capacity</li>
        <li>90 Ton Demag, 9.3oz Shot Capacity</li>
        <li>85 Ton Niigata, 4.5oz Shot Capacity</li>
        <li>75 Ton Vandorn, 5oz Shot Capacity</li>
        <li>55 Ton Cincinnati, 4.4oz Shot Capacity</li>
        <li>56 Ton Demag, 4.8oz Shot Capacity</li>
        <li>55 Ton Demag, 4.5oz Shot Capacity</li>
        <li>38 Ton Demag, 3.3oz Shot Capacity</li>
      </ul>
    </template>
  </Card>
  <Card>
    <template #title>Secondary Operations:</template>
    <template #content>
      <ul>
        <li>Hot Stamping</li>
        <li>Sublimating</li>
        <li>Sonic Welding</li>
        <li>Machining: Milling, Drilling, Tapping</li>
        <li>Component Assembly and Packaging</li>
      </ul>
    </template>
  </Card>
  <Card><template #title>Support Equipment:</template>
    <template #content>
      <ul>
        <li>Complete In-House Molding Making Facility</li>
        <li>Novatec 600 Pound Dryer Sytems</li>
        <li>Novatec <i>NovaDrier</i> Five 60 pound dryers, individually controlled</li>
        <li>Dri-Air Five 35 pound dryers, individually controlled</li>
        <li>Computrac <i>Vapor Pro</i> Moisture Analyzer</li>
        <li>BUDZAR Hot Oil Temperature Control Unit, 550?F </li>
        <li>Wittmann Tempro Plus 285 temperature controller</li>
        <li>Conair Sprue Pickers</li>
        <li>Automated Hopper Loader Dryer Systems</li>
        <li>Logic Seal Systems</li>
        <li>Thermolators</li>
        <li>Granulators</li>
        <li>Part Conveyers</li>
      </ul>
    </template>
  </Card>

  <Card>
    <template #header>
      <Image :src="injmold" />
    </template>
    <template #title>Inventory Support:</template>
    <template #content>Our extensive warehouse capabilities allow us to stock finished goods for daily shipment to
      support our customers JIT (Just In Time) manufacturing operations.</template>
    <template #footer>
      <Image :src="IinvSup" />
    </template>
  </Card>
</template>