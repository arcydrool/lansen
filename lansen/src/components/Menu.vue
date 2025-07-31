<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { computed } from 'vue';
import PanelMenu from 'primevue/panelmenu';
import MenuIcon from './MenuIcon.vue';
import type { MenuIconKey } from './icons/menu';
import type { ComputedRef } from 'vue';

type MenuItem = {
  icon: {type: MenuIconKey, required: true};
  route: String;
  label: String;
}
const items = computed(() => {
      return [
        { icon: "Home", route: "home", label: "Home" },
        { icon: "About", route: "about", label: "About Us" },
        { icon: "Custom", route: "mold", label: "Custom Mold" },
        { icon: "Inject", route: "inject", label: "Injection Molding" },
        { icon: "Spec", route: "specimen", label: "Specimen Molding" },
        { icon: "Plaque", route: "plaque", label: "Plaque Molding" },
        { icon: "Contact", route: "contact", label: "Contact" },
        { icon: "Quote", route: "quote", label: "Quote" },
      ]
    })
</script>

<template>

  <PanelMenu :model="items">
    <template #item="{ item }">
      <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
        <a class="flex items-center cursor-pointer text-surface-700 dark:text-surface-0 px-4 py-2" :href="href"
          @click="navigate">
          <span class="text-primary group-hover:text-inherit mr-2">
            <MenuIcon :icon="item.icon as MenuIconKey ?? 'About'" fill="var(--p-content-color)" />
          </span>
          <span>{{ item.label }}</span>
        </a>
      </router-link>
      <a v-else class="flex items-center cursor-pointer text-surface-700 dark:text-surface-0 px-4 py-2"
        :href="item.url" :target="item.target">
        <span class="text-primary group-hover:text-inherit mr-2"> 
          <MenuIcon :icon="item.icon as MenuIconKey ?? 'About'" fill="var(--p-content-color)" />
        </span>
        <span>{{ item.label }}</span>
        <span v-if="item.items" class="pi pi-angle-down text-primary ml-auto" />
      </a>
    </template>
  </PanelMenu>

</template>
