<script setup lang="ts">
import { ref, computed } from 'vue';
import { Card, Chip } from 'primevue';
import { RouterLink } from 'vue-router';
const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');
const cyear = computed(() => { return new Date().getFullYear() });
</script>

<template>
  <Card id="contact" class="footer">
    <template #content>
      1 Main St., Berkshire MA 01224<br>
      Mail: P.O. Box 1481<br>
      Lanesboro, MA 01237<br>
      Truck: <Chip><RouterLink to="/directions">Large Vehicle Directions</RouterLink></Chip> - Tel: <Chip><a
        href="tel:+14134435328">(413)443-5328</a></Chip> - Fax: <Chip><a href="fax:+14134432688">(413)443-2688</a></Chip><br>
      <Chip><a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a></Chip>
    </template>
    <template #footer><span class="copy">&copy; <span id="copy">{{ cyear }}</span> Lansen Mold Co. Inc. - All rights
        reserved</span></template>
  </Card>

</template>

<style scoped>
.footer {
  position: sticky;
  z-index: 999;
  bottom: 0;
  width: 100%;
  text-align: center;
  font-family: Tahoma, Verdana, sans-serif;
  font-size: x-small;
  max-width: 804px;
}

.copy {
  font-size: xx-small;
}
</style>
