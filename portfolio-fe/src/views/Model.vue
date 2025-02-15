<template>
<div class="bg-gray-800 shadow-lg rounded-lg p-6">

  <UploadForm 
  type="Model"
  acceptTypes=".splat"
  @upload="handleUpload"
/>
<div v-if="loading" class="text-center py-4">
  <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
</div>
<div v-if="error" class="text-red-400 text-center py-4">
  {{ error }}
</div>
<div v-if="!loading && models.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <canvas :ref="el => initCanvas(el, model)" class="w-full h-48 rounded mb-2"></canvas>
      <p class="text-center text-gray-300">{{ model.split("/").pop() }}</p>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, onUnmounted } from "vue";
import {
  Engine,
  Scene,
  ArcRotateCamera,
  HemisphericLight,
  SceneLoader,
  Vector3,
} from "@babylonjs/core";
import "@babylonjs/loaders";
import { listModels } from "@/api";
import UploadForm from "@/components/UploadForm.vue";

const models = ref([]);
const engineMap = new Map();

onMounted(async () => {
  try {
    models.value = await listModels();
    console.log("Loaded models:", models.value);
  } catch (error) {
    console.error("Error in onMounted:", error);
  }
});

function initCanvas(canvas, modelUrl) {
  if (!canvas || engineMap.has(modelUrl)) return;

  const engine = new Engine(canvas, true);
  const scene = new Scene(engine);
  
  // Setup camera for this viewer
  const camera = new ArcRotateCamera(
    "camera",
    Math.PI / 2,
    Math.PI / 3,
    15,
    Vector3.Zero(),
    scene
  );
  camera.attachControl(canvas, false);
  

  engineMap.set(modelUrl, { engine, scene });

  engine.runRenderLoop(() => scene.render());


  loadModel(modelUrl, scene);


  window.addEventListener("resize", () => engine.resize());
}

function loadModel(url, scene) {
  try {
    SceneLoader.ImportMeshAsync("standardMesh", "", url, scene)
      .then((result) => {
        const splat = result.meshes[0];
        
        splat.name = "standardMesh";
        splat.position = new Vector3(0, 0, 0);
        splat.scaling = new Vector3(5, 5, 5);
        
        console.log(`Loaded model: ${url.split("/").pop()}`);
      });
  } catch (error) {
    console.error("Error loading model:", error, url);
  }
}

// Cleanup on component unmount
onUnmounted(() => {
  engineMap.forEach(({ engine, scene }) => {
    scene.dispose();
    engine.dispose();
  });
});
</script>