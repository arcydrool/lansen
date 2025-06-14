<script setup lang="ts">
import {ref} from 'vue';
const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');
</script>

<template>
    <div id="contact" class="footer">
    1 Main St., Berkshire MA 01224<br>
    Mail: P.O. Box 1481<br>
    Lanesboro, MA 01237<br>
    Tel:(413)443-5328 - Fax: (413)443-2688<br>
    <a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a>
    </div>
</template>

<style scoped>
.footer {
  position: fixed;
  left: 0;
  bottom: 0;
  width: 100%;
  border: 4px solid #2f2328;
  background-color: #1f1318;
  color: white;
  text-align: center;
  font-family: Tahoma;
  font-size: small;
}
</style>
