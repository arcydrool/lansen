<script setup lang="ts">
import type { PropType } from 'vue';
import { icons } from './icons/menu.ts'
import type { MenuIconKey } from './icons/menu.ts';
defineProps({
  icon: {type: String as PropType<MenuIconKey>, required: true,},
  fill: String
})
</script>
<template>
  <svg
    width="24"
    height="24"
    viewBox="0 0 24 24"
    :fill="fill"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path v-for="(props, key) in icons[icon]" :key="key" v-bind="props" />
  </svg>
</template>