<script setup lang="ts">
import { computed, reactive } from 'vue'
import { useHead } from '@vueuse/head'
import { RouterView } from 'vue-router'
import Menu from './components/Menu.vue'
import Splitter from 'primevue/splitter'
import SplitterPanel from 'primevue/splitterpanel'
import Foot from './components/Foot.vue'
import { Image, Card } from 'primevue'
import title from './assets/title.svg';

const siteData = reactive({
  title: 'Lansen Mold - Custom injection molder; mold design, manufacturing and repair; molding ISO and ASTM test specimens',
  description: 'Custom Injection molders supplying the plastics industry with cost effective solutions to molding and tooling needs.',
  keywords: ["plastics", "injection molding", "mold making", "mould making", "mould design", "mold design and engineering", "machining", "tooling", "injection molding", "tool and die", "die shop", "die maker", "plastic molds", "plastic moulds", "platic dies", "izod", "ASTM", "test", "testing", "IZOD", "notched izod"]
})


useHead({
  // Can be static or computed
  title: computed(() => siteData.title),
  meta: [
    {
      name: 'description',
      content: computed(() => siteData.description),
    },
    {
      name: 'keywords',
      content: computed(() => siteData.keywords.join(", ")),
    }
  ],
})
</script>

<template>
  
  <main>
    <div class="topbar"><Card><template #content><Image :src="title"></Image></template></Card></div>
    <div class="topbox"></div>
    <div class="ct">
    <div class="max-w-[807]">
    <Splitter class="mb-8" >
      <SplitterPanel :size="7" :minSize="5" >
        <Menu></Menu>
      </SplitterPanel>
      <SplitterPanel>
        <div class="max-w-prose">
        <RouterView />
        </div>
      </SplitterPanel>
    </Splitter>
    </div></div>
  </main>
  <div class="ct">
  <Foot></Foot>
  </div>
</template>
<style>
body {
  font-family: Tahoma, Verdana, sans-serif;
}

h1 {
  font-size: x-large;
}

h2 {
  font-size: large;
}

.topbar{
  display: inline flex;
  position: fixed;
  z-index: 1100;
  left: 50%;
  transform: translate(-50%,0%);
}

.topbox{
  width: 100%;
  height: 80px;
  padding-block: 1;
  background-color: var(--surface-ground);
}

.ct {
  display: flex;
  justify-content: center;
}
</style>