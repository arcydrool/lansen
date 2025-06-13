<script setup lang="ts">
import {ref} from 'vue';
const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');
</script>

<template>
    <div id="contact">
    1 Main St., Berkshire MA 01224<br>
    Mail: P.O. Box 1481<br>
    Lanesboro, MA 01237<br>
    Tel:(413)443-5328 - Fax: (413)443-2688<br>
    <a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a>
    </div>
</template>