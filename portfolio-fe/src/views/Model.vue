<template>
  <div class="flex flex-col gap-6">
    <div
      v-if="authStore.isAuthenticated"
      class="bg-gray-800 shadow-lg rounded-lg p-6"
    >
      <h2 class="text-xl font-bold text-gray-200 mb-4">Upload New Model</h2>
      <div class="border-2 border-gray-700 rounded-lg p-4">
        <UploadForm
          type="Model"
          acceptTypes=".splat, .obj"
          @upload="handleUpload"
        />
      </div>
    </div>

    <div class="bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-bold text-gray-200 mb-4 text-center">
        Model Gallery
      </h2>

      <CategoryFilter @filter="filterModels" />

      <div v-if="loading" class="text-center py-4">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"
        ></div>
      </div>

      <div v-if="error" class="text-red-400 text-center py-4">
        {{ error }}
      </div>

      <div
        v-if="!loading && filteredModels.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <div
          v-for="model in filteredModels"
          :key="model.id"
          class="bg-gray-700 rounded-lg p-4 shadow-md relative"
          @click="openModel(model)"
        >
          <button
            v-if="authStore.isAuthenticated"
            @click.stop="deleteModel(model.id)"
            class="absolute bottom-2 right-2 bg-red-600 hover:bg-red-700 text-white p-2 transition-colors z-10"
            aria-label="Delete model"
          >
            <i class="pi pi-trash"></i>
          </button>
          <canvas
            :ref="el => initCanvas(el, model.url)"
            class="w-full h-48 rounded mb-2"
          ></canvas>
          <div class="mt-2 text-gray-300">
            <h3 class="font-semibold">{{ model.name }}</h3>
            <p class="text-sm text-gray-400">
              Category: {{ model.category_name }}
            </p>
          </div>
        </div>
      </div>

      <div
        v-if="!loading && filteredModels.length === 0"
        class="text-gray-400 text-center py-4"
      >
        No models found
      </div>
    </div>

    <ModelView
      v-if="modelView"
      :model="modelView.url"
      :modelName="modelView.name"
      @close="modelView = null"
    />
  </div>
</template>

<script setup lang="ts">
  import { useAuthStore } from '@/utils/AuthStore';
  import { onMounted, ref, onUnmounted, computed } from 'vue';
  import {
    Engine,
    Scene,
    ArcRotateCamera,
    Vector3,
    SceneLoader,
    HemisphericLight,
  } from '@babylonjs/core';
  import '@babylonjs/loaders';
  import { getModelsWithDetails, uploadModel, deleteModel as deleteModelAPI, type Model } from '@/api';
  import UploadForm from '@/components/UploadForm.vue';
  import ModelView from '@/components/ModelView.vue';
  import CategoryFilter from '@/components/CategoryFilter.vue';

  const authStore = useAuthStore();
  const models = ref<Model[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const engineMap = new Map();
  const cleanupFunctions = ref<(() => void)[]>([]);
  const modelView = ref<Model | null>(null);
  const selectedCategories = ref<string[] | null>(null);

  const filteredModels = computed(() => {
    if (!selectedCategories.value || selectedCategories.value.length === 0) return models.value;
    return models.value.filter(model => selectedCategories.value?.includes(model.category_id));
  });

  const handleUpload = async (uploadData: {
    file: File;
    name: string;
    categoryId: string;
  }) => {
    try {
      loading.value = true;
      error.value = null;

      console.log('Model upload data:', {
        file: uploadData.file,
        name: uploadData.name,
        categoryId: uploadData.categoryId,
      });

      const data = {
        file: uploadData.file,
        name: uploadData.name,
        categoryId:
          typeof uploadData.categoryId === 'object'
            ? uploadData.categoryId.$oid
            : uploadData.categoryId,
      };

      await uploadModel(data);
      await fetchModels();
    } catch (err) {
      error.value =
        err instanceof Error ? err.message : 'Failed to upload model';
      console.error('Upload error:', err);
    } finally {
      loading.value = false;
    }
  };

  const fetchModels = async () => {
    try {
      loading.value = true;
      error.value = null;
      models.value = await getModelsWithDetails();
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
        disableWebGL2Support: false,
      });

      const scene = new Scene(engine);


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
      window.addEventListener('resize', resizeHandler);


      cleanupFunctions.value.push(() => {
        window.removeEventListener('resize', resizeHandler);
        if (engineMap.has(modelUrl)) {
          const { engine, scene } = engineMap.get(modelUrl);
          scene.dispose();
          engine.dispose();
          engineMap.delete(modelUrl);
        }
      });
    } catch (error) {
      console.error('Error initializing canvas:', error);
      error.value = 'Failed to initialize 3D viewer';
    }
  }


  onUnmounted(() => {
    cleanupFunctions.value.forEach(cleanup => cleanup());
    cleanupFunctions.value = [];
  });

  function loadModel(url: string, scene: Scene) {
    try {
      SceneLoader.ImportMeshAsync('', '', url, scene, undefined, '.splat')
        .then(result => {
          if (result.meshes.length > 0) {
            const splat = result.meshes[0];
            splat.name = 'standardMesh';
            splat.position = Vector3.Zero();
            splat.scaling = new Vector3(5, 5, 5);
          }
        })
        .catch(error => {
          console.error('Error loading model:', error);
          error.value = `Failed to load model: ${url.split('/').pop()}`;
        });
    } catch (error) {
      console.error('Error in loadModel:', error);
      error.value = `Error loading model: ${url.split('/').pop()}`;
    }
  }

  onUnmounted(() => {
    engineMap.forEach(({ engine, scene }) => {
      scene.dispose();
      engine.dispose();
    });
  });

  const openModel = (model: Model) => {
    modelView.value = model;
  };

  const deleteModel = async (id: string) => {
    if (!confirm('Are you sure you want to delete this model?')) return;
    
    try {
      loading.value = true;
      await deleteModelAPI(id);
      await fetchModels(); 
    } catch (err) {
      error.value = 'Failed to delete model';
      console.error('Delete error:', err);
    } finally {
      loading.value = false;
    }
  };

  const filterModels = (categories: string[] | null) => {
    selectedCategories.value = categories;
  };
</script>
