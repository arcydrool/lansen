import { createWebHashHistory, createRouter } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import AboutView from '../views/AboutView.vue'
import MoldView from '../views/MoldView.vue'
import InjectView from '../views/InjectView.vue'
import SpecimenView from '../views/SpecimenView.vue'
import PlaqueView from '../views/PlaqueView.vue'
import QuoteView from '../views/QuoteView.vue'
import ContactView from '../views/ContactView.vue'
import DirectionsView from '@/views/DirectionsView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'index', component: HomeView },
    { path: '/home', name: 'home', component: HomeView },
    { path: '/about', name: 'about', component: AboutView },
    { path: '/mold', name: 'mold', component: MoldView },
    { path: '/inject', name: 'inject', component: InjectView },
    { path: '/specimen', name: 'specimen', component: SpecimenView },
    { path: '/plaque', name: 'plaque', component: PlaqueView },
    { path: '/quote', name: 'quote', component: QuoteView },
    { path: '/contact', name: 'contact', component: ContactView },
    { path: '/directions', name: 'directions', component: DirectionsView },
  ],
})

export default router
