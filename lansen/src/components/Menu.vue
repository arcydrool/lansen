<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { computed } from 'vue';
import PanelMenu from 'primevue/panelmenu';

const items = computed(() => {
      return [
        { route: "home", label: "Home" },
        { route: "about", label: "About Us" },
        { route: "mold", label: "Custom Mold" },
        { route: "inject", label: "Injection Molding" },
        { route: "specimen", label: "Specimen Molding" },
        { route: "plaque", label: "Plaque Molding" },
        { route: "contact", label: "Contact" },
        { route: "quote", label: "Quote" },
      ]
    })
</script>

<template>

  <PanelMenu :model="items">
    <template #item="{ item }">
      <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
        <a class="flex items-center cursor-pointer text-surface-700 dark:text-surface-0 px-4 py-2" :href="href"
          @click="navigate">
          <span class="ml-2">{{ item.label }}</span>
        </a>
      </router-link>
      <a v-else class="flex items-center cursor-pointer text-surface-700 dark:text-surface-0 px-4 py-2"
        :href="item.url" :target="item.target">
        <span class="ml-2">{{ item.label }}</span>
        <span v-if="item.items" class="pi pi-angle-down text-primary ml-auto" />
      </a>
    </template>
  </PanelMenu>

</template>