import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import MoldView from '../views/MoldView.vue'
import InjectView from '../views/InjectView.vue'
import SpecimenView from '../views/SpecimenView.vue'
import PlaqueView from '../views/PlaqueView.vue'
import ContactView from '../views/ContactView.vue'

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    { path: '/', name: 'home', component: HomeView },
    { path: '/about', name: 'about', component: AboutView },
    { path: '/mold', name: 'mold', component: MoldView },
    { path: '/mold', name: 'inject', component: InjectView },
    { path: '/specimen', name: 'mold', component: SpecimenView },
    { path: '/mold', name: 'plaque', component: PlaqueView },
    { path: '/mold', name: 'contact', component: ContactView },
  ],
})

export default router
