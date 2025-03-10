<template>
  <div
    class="fixed inset-0 z-50 bg-black bg-opacity-75 flex items-center justify-center"
    @click="handleBackdropClick"
  >
    <div class="relative w-full max-w-6xl h-[80vh] m-4 bg-gray-800 rounded-lg">
      <button
        @click="$emit('close')"
        class="absolute top-4 right-4 bg-gray-700 text-white w-8 h-8 rounded-full hover:bg-gray-600 z-10"
      >
        ×
      </button>

      <canvas ref="canvasRef" class="w-full h-full rounded-lg"></canvas>

      <div
        class="absolute top-4 left-4 text-white bg-gray-800 bg-opacity-75 px-3 py-1 rounded"
      >
        {{ modelName }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import {
    Engine,
    Scene,
    ArcRotateCamera,
    Vector3,
    SceneLoader,
    HemisphericLight,
  } from '@babylonjs/core';

  const props = defineProps<{
    model: string;
    modelName: string;
  }>();

  const emit = defineEmits<{
    (e: 'close'): void;
  }>();

  const canvasRef = ref<HTMLCanvasElement | null>(null);
  let engine: Engine | null = null;
  let scene: Scene | null = null;

  onMounted(() => {
    if (!canvasRef.value) return;

    initScene();
  });

  function initScene() {
    if (!canvasRef.value) return;

    engine = new Engine(canvasRef.value, true);
    scene = new Scene(engine);

    const light = new HemisphericLight('light', new Vector3(0, 1, 0), scene);
    light.intensity = 1.0;

    const camera = new ArcRotateCamera(
      'camera',
      Math.PI / 2,
      Math.PI / 3,
      15,
      Vector3.Zero(),
      scene
    );
    camera.attachControl(canvasRef.value, true);

    loadModel();

    engine.runRenderLoop(() => {
      if (scene) {
        scene.render();
      }
    });

    window.addEventListener('resize', handleResize);
  }

  function loadModel() {
    if (!scene) return;

    SceneLoader.ImportMeshAsync('', '', props.model, scene, undefined, '.splat')
      .then(result => {
        if (result.meshes.length > 0) {
          const splat = result.meshes[0];
          splat.position = Vector3.Zero();
          splat.scaling = new Vector3(5, 5, 5);
        }
      })
      .catch(error => {
        console.error('Error loading model:', error);
      });
  }

  const handleResize = () => {
    if (engine) {
      engine.resize();
    }
  };

  const handleBackdropClick = (e: MouseEvent) => {
    if (e.target === e.currentTarget) {
      emit('close');
    }
  };

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
    scene?.dispose();
    engine?.dispose();
  });
</script>
