<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
<nav>
  <router-link :to="{ name: 'home' }">Home</router-link>
  <router-link :to="{ name: 'about' }">About Us</router-link>
  <router-link :to="{ name: 'mold' }">Custom Mold</router-link>
  <router-link :to="{ name: 'inject' }">Injection Molding</router-link>
  <router-link :to="{ name: 'specimen' }">Specimen Molding</router-link>
  <router-link :to="{ name: 'plaque' }">Plaque Molding</router-link>
  <router-link :to="{ name: 'contact' }">Contact</router-link>
</nav>
</template>