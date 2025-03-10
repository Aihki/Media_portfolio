import { createApp } from 'vue'
import { createPinia } from 'pinia';
import App from './App.vue'
import router from './router'
import './index.css'
import 'primeicons/primeicons.css';
import * as tf from '@tensorflow/tfjs';
import '@tensorflow/tfjs-backend-webgl';

async function initTensorFlow() {
  // Set memory growth to true to avoid memory issues
  tf.env().set('WEBGL_FORCE_F16_TEXTURES', false);
  tf.env().set('WEBGL_VERSION', 2);
  tf.env().set('WEBGL_FLUSH_THRESHOLD', 1);
  tf.env().set('CHECK_COMPUTATION_FOR_ERRORS', false);

  await tf.setBackend('webgl');
  await tf.ready();
  
  // Log successful initialization
  console.log('TensorFlow.js initialized with WebGL backend');
}

initTensorFlow().then(() => {
  const app = createApp(App);
  const pinia = createPinia();

  app.use(pinia);
  app.use(router);
  app.mount('#app');
}).catch(console.error);
