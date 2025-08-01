<script setup lang="ts">
import { Card, Chip } from 'primevue';
import { ref, reactive } from 'vue';
import ProgressSpinner from 'primevue/progressspinner';

import { defineAsyncComponent } from 'vue'
import GoForm from '@/components/GoForm.vue';
const spin = reactive({ spin: 1 });
const Contact = defineAsyncComponent(() => { var v = import('@/components/Contact.vue'); spin.spin = 0; return v });
const mailLink = ref('info' + '@'.repeat(1) + 'lansen.com');

</script>
<template>
  <Card>
    <template #header>Contact Us</template>
    <template #content>
      <ul>
        <li>Driving directions to Lansen Mold. Separate instructions for large vehicles. <RouterLink to="/directions">
            Driving Directions
          </RouterLink>
        </li>
        <li>Reach us by phone. Call 413-443-5328 and ask for Kari Kristensen.
        </li>
        <li>Reach us via E-mail.
          <Chip><a v-bind:href="'mailto:' + mailLink">{{ mailLink }}</a></Chip>
        </li>
        <li>Request a quote.
          <GoForm />
        </li>
        <li>Our standard contact form. Fill out the contact page below and we will respond to your request.</li>
      </ul>

    </template>
  </Card>
  <ProgressSpinner v-if="spin.spin == 1" id="spin"></ProgressSpinner>
  <Contact></Contact>
</template>