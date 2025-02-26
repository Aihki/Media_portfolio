<template>
  <div class="flex items-center justify-center bg-gray-900">
    <div class="w-full bg-gray-800 rounded-lg shadow-lg p-8">
      <div class="max-h-32 overflow-y-auto mb-4 flex flex-wrap gap-2 p-2">
        <button
          v-for="model in models"
          :key="model.id"
          @click="loadModel(model)"
          class="px-4 py-2 bg-green-e rounded hover:bg-green-700 transition-colors flex-shrink-0"
        >
          {{ model.name }}
        </button>
      </div>
      <canvas ref="canvas" class="w-full"></canvas>
    </div>
  </div>
</template>
600 text-whit
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import * as BABYLON from '@babylonjs/core';
import '@babylonjs/loaders';
import { getModelsWithDetails, type Model } from '@/api';

const canvas = ref<HTMLCanvasElement | null>(null);
const engine = ref<BABYLON.Engine | null>(null);
const scene = ref<BABYLON.Scene | null>(null);
const camera = ref<BABYLON.UniversalCamera | null>(null);

const models = ref<Model[]>([]);
let currentMesh: BABYLON.AbstractMesh | null = null;

const loadModel = async (model: Model) => {
    if (!scene.value) return;
    
    if (currentMesh) {
        currentMesh.dispose();
    }

    try {
        const result = await BABYLON.SceneLoader.ImportMeshAsync(
            '',
            'http://localhost:3000/static/models/',
            model.filename,
            scene.value
        );

        if (result.meshes.length > 0) {
            currentMesh = result.meshes[0];
            currentMesh.position = new BABYLON.Vector3(0, 0.5, 0);
            currentMesh.scaling = new BABYLON.Vector3(0.5, 0.5, 0.5);
        }
    } catch (error) {
        console.error('Error loading model:', error);
    }
};

const createScene = (): void => {
    if (!canvas.value || !engine.value) return;
    
    scene.value = new BABYLON.Scene(engine.value);
    
    camera.value = new BABYLON.UniversalCamera(
        "FPCamera",
        new BABYLON.Vector3(0, 2, -5),
        scene.value
    );

    const light = new BABYLON.HemisphericLight(
        'light',
        new BABYLON.Vector3(0, 1, 0),
        scene.value
    );
    light.intensity = 1.0;

    if (camera.value && canvas.value) {
        camera.value.attachControl(canvas.value, true);
        camera.value.speed = 0.5;
        camera.value.angularSensibility = 1000;
        
        
        camera.value.keysUp.push(87);    
        camera.value.keysDown.push(83);  
        camera.value.keysLeft.push(65);  
        camera.value.keysRight.push(68); 
    }
};

onMounted(async () => {
    try {
        models.value = await getModelsWithDetails();
        
        if (canvas.value) {
            engine.value = new BABYLON.Engine(canvas.value, true);
            createScene();
        
            if (models.value.length > 0) {
                await loadModel(models.value[0]);
            }

            window.addEventListener('resize', () => {
                engine.value?.resize();
            });
            
            engine.value.runRenderLoop(() => {
                scene.value?.render();
            });
        }
    } catch (error) {
        console.error('Error fetching models:', error);
    }
});

onUnmounted(() => {
    engine.value?.dispose();
});
</script>
