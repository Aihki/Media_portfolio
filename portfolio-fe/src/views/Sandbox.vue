<template>
  <div class="flex items-center justify-center bg-gray-900">
    <div class="w-full bg-gray-800 rounded-lg shadow-lg p-8">
    
      <div class="bg-gray-700 rounded-lg p-4 mb-6">
        <div class="flex items-center justify-between">
          <button
            @click="previousModel"
            :disabled="currentModelIndex === 0"
            aria-label="Previous model"
            class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <i class="pi pi-chevron-left"></i>
          </button>

          <div class="flex flex-col items-center gap-2">
            <div class="text-white text-xl font-semibold">
              {{ currentModelName }}
            </div>
            <div class="text-gray-400 text-sm">
              {{ models[currentModelIndex]?.category_name || 'No category' }}
            </div>
            <div class="flex items-center gap-2">
              <div v-if="isLoading" class="animate-spin text-blue-400">
                <i class="pi pi-spinner"></i>
              </div>
              <button
                @click="toggleRotation"
                class="px-3 py-1 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors text-sm"
              >
                {{ isRotating ? 'Stop Rotation' : 'Start Rotation' }}
              </button>
            </div>
          </div>

          <button
            @click="nextModel"
            :disabled="currentModelIndex === models.length - 1"
            aria-label="Next model"
            class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <i class="pi pi-chevron-right"></i>
          </button>
        </div>
      </div>

    
      <div class="relative">
        <canvas ref="canvas" class="w-full rounded-lg"></canvas>
        <div class="absolute bottom-4 right-4 text-sm text-white bg-black bg-opacity-50 px-3 py-1 rounded">
          WASD to move | Mouse to look
        </div>
      </div>

  
      <div class="mt-4 flex items-center justify-center gap-1">
        <div 
          v-for="(_, index) in models" 
          :key="index"
          class="w-2 h-2 rounded-full transition-colors"
          :class="index === currentModelIndex ? 'bg-blue-500' : 'bg-gray-600'"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted, onUnmounted, computed } from 'vue';
  import * as BABYLON from '@babylonjs/core';
  import '@babylonjs/loaders';
  import { getModelsWithDetails, type Model } from '../api';

  const canvas = ref<HTMLCanvasElement | null>(null);
  const engine = ref<BABYLON.Engine | null>(null);
  const scene = ref<BABYLON.Scene | null>(null);
  const camera = ref<BABYLON.UniversalCamera | null>(null);

  const models = ref<Model[]>([]);
  let currentMesh: BABYLON.AbstractMesh | null = null;

  const currentModelIndex = ref(0);
  const currentModelName = computed(() => {
    return models.value[currentModelIndex.value]?.name || 'No model selected';
  });

  const isLoading = ref(false);
  const isRotating = ref(false);
  let rotationAnimation: BABYLON.Observer<BABYLON.Scene> | null = null;  // Fix type

  const startRotation = () => {
    if (!currentMesh || !scene.value) return;

    rotationAnimation = scene.value.onBeforeRenderObservable.add(() => {
      if (currentMesh) {
        currentMesh.rotation.y += 0.01;
      }
    });
  };

  const stopRotation = () => {
    if (rotationAnimation !== null && scene.value) {
      scene.value.onBeforeRenderObservable.remove(rotationAnimation);
      rotationAnimation = null;
    }
  };

  const toggleRotation = () => {
    isRotating.value = !isRotating.value;
    if (isRotating.value) {
      startRotation();
    } else {
      stopRotation();
    }
  };

  const loadModel = async (model: Model) => {
    const currentScene = scene.value as BABYLON.Scene;
    if (!currentScene) return;

    isLoading.value = true;
    stopRotation();

    if (currentMesh) {
      currentMesh.dispose();
    }

    try {
      const modelUrl = new URL(model.url);
      const filename = modelUrl.pathname.split('/').pop() || '';
      const rootUrl = `${modelUrl.origin}/static/models/`;

      console.log('Loading model:', {
        rootUrl,
        filename,
        fullUrl: model.url
      });

      const loadProgress = (evt: BABYLON.ISceneLoaderProgressEvent) => {
        if (evt.lengthComputable) {
          const progress = (evt.loaded * 100 / evt.total).toFixed();
          console.log(`Loading progress: ${progress}%`);
        }
      };

      const result = await BABYLON.SceneLoader.ImportMeshAsync(
        '',
        rootUrl,
        filename,
        currentScene,
        loadProgress,
        filename.endsWith('.splat') ? '.splat' : undefined
      );

      if (!result.meshes.length) {
        throw new Error('No meshes loaded');
      }

      currentMesh = result.meshes[0];
      currentMesh.position = new BABYLON.Vector3(0, 0.5, 0);
      currentMesh.scaling = new BABYLON.Vector3(0.5, 0.5, 0.5);

      if (isRotating.value) {
        startRotation();
      }
    } catch (error) {
      console.error('Model loading error:', error);
      // Add user-friendly error message
      const errorDiv = document.createElement('div');
      errorDiv.className = 'text-red-500 text-center mt-4';
      errorDiv.textContent = 'Failed to load model. Please try again.';
      canvas.value?.parentNode?.appendChild(errorDiv);
    } finally {
      isLoading.value = false;
    }
  };

  const nextModel = async () => {
    if (currentModelIndex.value < models.value.length - 1) {
      currentModelIndex.value++;
      await loadModel(models.value[currentModelIndex.value]);
    }
  };

  const previousModel = async () => {
    if (currentModelIndex.value > 0) {
      currentModelIndex.value--;
      await loadModel(models.value[currentModelIndex.value]);
    }
  };

  const createScene = (): void => {
    if (!canvas.value || !engine.value) return;

    // Assert the engine type to fix compatibility issues
    const babylonEngine = engine.value as unknown as BABYLON.Engine;
    const newScene = new BABYLON.Scene(babylonEngine);
    scene.value = newScene;

    camera.value = new BABYLON.UniversalCamera(
      'FPCamera',
      new BABYLON.Vector3(0, 2, -5),
      newScene
    );

    const light = new BABYLON.HemisphericLight(
      'light',
      new BABYLON.Vector3(0, 1, 0),
      newScene
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
        // Create and assert the engine type
        const babylonEngine = new BABYLON.Engine(canvas.value, true);
        engine.value = babylonEngine as unknown as typeof engine.value;
        createScene();

        if (models.value.length > 0) {
          currentModelIndex.value = 0;
          await loadModel(models.value[0]);
        }

        window.addEventListener('resize', () => {
          engine.value?.resize();
        });

        babylonEngine.runRenderLoop(() => {
          if (scene.value) {
            (scene.value as BABYLON.Scene).render();
          }
        });
      }
    } catch (error) {
      console.error('Error fetching models:', error);
    }
  });

  onUnmounted(() => {
    engine.value?.dispose();
    stopRotation();
  });
</script>

<style scoped>
canvas {
  min-height: 500px;
  background-color: #1a1a1a;
  touch-action: none;
}

@media (max-width: 768px) {
  canvas {
    min-height: 300px;
  }
}
</style>
