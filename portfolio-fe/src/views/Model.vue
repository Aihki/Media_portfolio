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
      <div v-for="model in models" :key="model" class="bg-gray-800 rounded-lg p-4 shadow-md border border-gray-700">
        <canvas :ref="el => initCanvas(el, model)" class="w-full h-48 rounded mb-2"></canvas>
        <p class="text-center text-gray-300">{{ model.split("/").pop() }}</p>
      </div>
    </div>
    
    <div v-if="!loading && models.length === 0" class="text-gray-400 text-center py-4">
      No models found
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, onUnmounted } from "vue";
import {
  Engine,
  Scene,
  ArcRotateCamera,
  Vector3,
  SceneLoader,
  HemisphericLight
} from "@babylonjs/core";
import "@babylonjs/loaders";
import { listModels, uploadModel } from "@/api";
import UploadForm from "@/components/UploadForm.vue";

const models = ref([]);
const loading = ref(false);
const error = ref<string | null>(null);
const engineMap = new Map();
const cleanupFunctions = ref<(() => void)[]>([]);

const handleUpload = async (file: File) => {
  try {
    loading.value = true;
    error.value = null;
    await uploadModel(file);
    await fetchModels(); // Add this function to refetch models after upload
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to upload model';
    console.error('Upload error:', err);
  } finally {
    loading.value = false;
  }
};

const fetchModels = async () => {
  try {
    loading.value = true;
    error.value = null;
    models.value = await listModels();
  } catch (err) {
    error.value = 'Failed to load models';
    console.error('Error loading models:', err);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchModels);

function initCanvas(canvas: HTMLCanvasElement | null, modelUrl: string) {
  if (!canvas || engineMap.has(modelUrl)) return;

  try {
    const engine = new Engine(canvas, true, { 
      preserveDrawingBuffer: true, 
      stencil: true,
      disableWebGL2Support: false 
    });

    const scene = new Scene(engine);
    
    // Add light to the scene
    const light = new HemisphericLight("light", new Vector3(0, 1, 0), scene);
    light.intensity = 1.0;

    const camera = new ArcRotateCamera(
      "camera",
      Math.PI / 2,
      Math.PI / 3,
      15,
      Vector3.Zero(),
      scene
    );
    camera.attachControl(canvas, true);
    
    engineMap.set(modelUrl, { engine, scene });

    scene.executeWhenReady(() => {
      engine.runRenderLoop(() => {
        if (scene.activeCamera) {
          scene.render();
        }
      });
    });

    loadModel(modelUrl, scene);

    const resizeHandler = () => {
      engine.resize();
    };
    window.addEventListener("resize", resizeHandler);

    // Store cleanup function instead of calling onUnmounted directly
    cleanupFunctions.value.push(() => {
      window.removeEventListener("resize", resizeHandler);
      if (engineMap.has(modelUrl)) {
        const { engine, scene } = engineMap.get(modelUrl);
        scene.dispose();
        engine.dispose();
        engineMap.delete(modelUrl);
      }
    });

  } catch (error) {
    console.error("Error initializing canvas:", error);
    error.value = "Failed to initialize 3D viewer";
  }
}

// Single onUnmounted handler for the component
onUnmounted(() => {
  cleanupFunctions.value.forEach(cleanup => cleanup());
  cleanupFunctions.value = [];
});

function loadModel(url: string, scene: Scene) {
  try {
    SceneLoader.ImportMeshAsync("", "", url, scene, 
      undefined, 
      ".splat")
      .then((result) => {
        if (result.meshes.length > 0) {
          const splat = result.meshes[0];
          splat.name = "standardMesh";
          splat.position = Vector3.Zero();
          splat.scaling = new Vector3(5, 5, 5);
        }
      })
      .catch((error) => {
        console.error("Error loading model:", error);
        error.value = `Failed to load model: ${url.split("/").pop()}`;
      });
  } catch (error) {
    console.error("Error in loadModel:", error);
    error.value = `Error loading model: ${url.split("/").pop()}`;
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