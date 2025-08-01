import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createHead } from '@vueuse/head'
import './assets/main.css';
import PrimeVue from 'primevue/config';
import Nora from '@primeuix/themes/nora';
import { definePreset, palette } from '@primeuix/themes';

const app = createApp(App)
const head = createHead()


const myPal = palette('#1f1318');

const MyPreset = definePreset(Nora, {
    semantic: {
        primary: myPal
    }
});


app.use(router)
app.use(head)
app.use(PrimeVue, {
    theme: {
        preset: MyPreset,
        options: {
        prefix: 'p',
        darkModeSelector: 'system',
        cssLayer: false
    }
    },
    
});

app.mount('#app')