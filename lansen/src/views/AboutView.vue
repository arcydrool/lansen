<script setup lang="ts">
import Card from 'primevue/card';
import Image from 'primevue/image';
import ashuwill from "../assets/ashuwillticook.jpg"
</script>
<template>
  <Card>
    <template #title>About Lansen Mold Inc.</template>
    <template #content>
      <P>
      <b>Lansen Mold</b> was founded in 1960 and continues to be a family owned and operated business. Throughout the years of continuing change in technology and manufacturing processes, Lansen's primary focus has not changed,
      <i>"to manufacture a quality on time product while providing personal service."</i>
      </P>
      <Image :src="ashuwill" alt="Lansen Mold is located along side the Ashuwillticook rail trail. The trail is a scenic walk through flora of Berkshire County."/>
      <P>
      Lansen Mold is located in the scenic foothills of Berkshire County. We are a proud member of the
      Berkshire Plastic Network and contribute time and materials to support our local trade schools and 
      educational programs.</P>
    </template>
  </Card>
</template>