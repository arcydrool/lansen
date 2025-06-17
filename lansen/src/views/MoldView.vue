<script setup lang="ts">
import Card from 'primevue/card';
import Image from 'primevue/image';
import Icm1 from '../assets/cust_molds.jpg';
import Icm2 from '../assets/custom_molds2.jpg';
import Icm3 from '../assets/custom_molds3.jpg';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
</script>

<template>
  <h1>Custom Molds</h1>
  <Card>
    <template #title>Design Capabilities:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Lansen provides complete project management, product design, refinement and analysis.</li>
            <li>We work with current releases of Cadra, SolidWorks and Surfcam to provide the latest in technology to
              support our customers and mold making facility.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="Icm1" alt="Simulated Molded Cast and Mold" />
        </SplitterPanel>
      </Splitter>
    </template>

  </Card>
  <Card>
    <template #title>Tooling Capabilities:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Our trained and experienced mold makers use a combination of CAD/CAM and CNC machinery to produce high
              quality injection molds.</li>
            <li>We match our customer needs with quality cost effective tooling.</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="Icm2" alt="Sample Molded Cast and Mold" />
        </SplitterPanel>
      </Splitter>
    </template>

  </Card>
  <Card>
    <template #title>Design Capabilities:</template>
    <template #content>
      <Splitter :gutter-size="0">
        <SplitterPanel>
          <ul>
            <li>Cincinnati 3 Axis Vertical CNC Machining Center, 20x20x20 travel</li>
            <li>Fadal 3 Axis Vertical CNC Machining Center, 22x16x20 travel</li>
            <li>ProtoTrak DMP 3 Axis Bed Mill, AGE 3 controls, 31x17x23 travel</li>
            <li>Yuasa TPDX-5C Rotary 4th Axis, Full 4th Aixs and Indexer</li>
            <li>ProtoTrak TRL1745P CNC Lathe, 17x45 travel</li>
            <li>H. T Schudin Hydraulic Cylindrical Grinder, 8x16 travel, 16? Dia Wheel</li>
            <li>Milwaukee Model K Vertical Miller, 36x13x20 travel</li>
            <li>Agie Spirit 2 CNC EDM 12x10x10 travel</li>
            <li>Hansvedt EDM with CNC System 3R Head</li>
            <li>Surface Grinders, Ranging 6x11 to 17x42 travel</li>
            <li>Vertical Profile Machines, Bridgeport</li>
            <li>Harig DedTru Centerless Grinder</li>
            <li>South Bend Lathe, 10x24 travel</li>
          </ul>
        </SplitterPanel>
        <SplitterPanel>
          <Image :src="Icm3" alt="Sample Molded Cast and Mold" />
        </SplitterPanel>
      </Splitter>
    </template>

  </Card>
</template>