<script setup lang="ts">
import { ref } from 'vue';
import { Card } from 'primevue';
const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');
</script>

<template>
  <Card id="contact" class="footer">
    <template #content>
      1 Main St., Berkshire MA 01224<br>
      Mail: P.O. Box 1481<br>
      Lanesboro, MA 01237<br>
      Tel: <a href="tel:+14134435328">(413)443-5328</a> - Fax: <a href="fax:+14134432688" >(413)443-2688</a><br>
      <a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a>
    </template>
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
}
</style>
