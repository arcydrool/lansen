<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { Chip } from 'primevue';
</script>
<template>
    <li>We respond to all inquirers within 24 hours.
        Please fill out our <Chip><router-link to="/quote"> online quotation form</router-link></Chip>
        or call John Rosier at <Chip><a href="tel:+14134435328">(413)443-5328</a></Chip>.</li>
</template>