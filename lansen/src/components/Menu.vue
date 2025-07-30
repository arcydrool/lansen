<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { computed } from 'vue';
import PanelMenu from 'primevue/panelmenu';
import mhome from "./icons/mhome.svg";
import mabout from "./assets/ico/mabout.svg";
import mcustom from "./assets/ico/mcustom.svg";
import minject from "./assets/ico/minject.svg";
import mspec from "./assets/ico/mspec.svg";
import mplaque from "./assets/ico/mplaque.svg";
import mcontact from "./assets/ico/mcontact.svg";
import mquote from "./assets/ico/mquote.svg";

const items = computed(() => {
      return [
        { icon: "mhome", route: "home", label: "Home" },
        { icon: "mabout", route: "about", label: "About Us" },
        { icon: "mcustom", route: "mold", label: "Custom Mold" },
        { icon: "minject", route: "inject", label: "Injection Molding" },
        { icon: "mspec", route: "specimen", label: "Specimen Molding" },
        { icon: "mplaque", route: "plaque", label: "Plaque Molding" },
        { icon: "mcontact", route: "contact", label: "Contact" },
        { icon: "mquote", route: "quote", label: "Quote" },
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