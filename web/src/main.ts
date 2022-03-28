import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')

import init, {run} from './hawk-wasm/pkg/hawk_wasm'

await init()

while (true) {
    run(prompt("Enter your Hawk code") ?? "")
}