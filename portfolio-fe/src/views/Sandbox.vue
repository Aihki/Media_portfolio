<template>
  <div class="flex items-center justify-center bg-gray-900">
    <div class="w-full bg-gray-800 rounded-lg shadow-lg p-8">
      <div class="flex flex-col gap-4 mb-4">
        <div class="flex items-center justify-between">
          <button
            @click="previousModel"
            aria-label="Previous model"
            class="px-4 py-2 bg-gray-700 text-white rounded-md hover:bg-gray-600 transition-colors"
          >
            <i class="pi pi-chevron-left"></i>
          </button>

          <div class="flex items-center gap-4">
            <div v-if="isLoading" class="animate-spin">
              <i class="pi pi-spinner text-white"></i>
            </div>

            <div class="text-white text-xl">
              {{ currentModelName }}
            </div>

            <button
              @click="toggleRotation"
              aria-label="Toggle model rotation"
              class="px-3 py-1 bg-gray-700 text-white rounded-md hover:bg-gray-600 transition-colors text-sm"
            >
              {{ isRotating ? 'Stop' : 'Rotate' }}
            </button>
          </div>

          <button
            @click="nextModel"
            aria-label="Next model"
            class="px-4 py-2 bg-gray-700 text-white rounded-md hover:bg-gray-600 transition-colors"
          >
            <i class="pi pi-chevron-right"></i>
          </button>
        </div>
      </div>

      <canvas ref="canvas" class="w-full"></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted, onUnmounted, computed } from 'vue';
  import * as BABYLON from '@babylonjs/core';
  import '@babylonjs/loaders';
  import { getModelsWithDetails, type Model } from '@/api';

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
  let rotationAnimation: number | null = null;


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
    if (!scene.value) return;

    isLoading.value = true;
    stopRotation();

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

      if (isRotating.value) {
        startRotation();
      }
    } catch (error) {
      console.error('Error loading model:', error);
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

    scene.value = new BABYLON.Scene(engine.value);

    camera.value = new BABYLON.UniversalCamera(
      'FPCamera',
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
          currentModelIndex.value = 0;
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
    stopRotation();
  });
</script>
