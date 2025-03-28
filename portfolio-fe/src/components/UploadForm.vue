<template>
   <div v-if="authStore.isAuthenticated" class="mb-8 border-b border-gray-700 pb-6">
    <h3 class="text-lg font-semibold mb-4 text-gray-200">
      Upload New {{ type }}
    </h3>
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-300 mb-1">Name</label>
        <input
          v-model="name"
          type="text"
          class="w-full px-3 py-2 bg-gray-700 text-gray-200 rounded-md"
          :placeholder="`Enter ${type} name`"
        />
      </div>
      
      <div>
        <label class="block text-sm font-medium text-gray-300 mb-1">Category</label>
        <div class="flex gap-2">
          <select
            v-model="selectedCategory"
            class="flex-grow px-3 py-2 bg-gray-700 text-gray-200 rounded-md"
          >
            <option value="">Select category</option>
            <option v-for="cat in categories" :key="cat._id.$oid" :value="cat._id">
              {{ cat.name }}
            </option>
          </select>
          <button
            @click="showNewCategory = !showNewCategory"
            class="px-3 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500"
          >
            {{ showNewCategory ? 'Cancel' : 'New' }}
          </button>
        </div>
      </div>

      <div v-if="showNewCategory">
        <div class="flex gap-2">
          <input
            v-model="newCategoryName"
            type="text"
            class="flex-grow px-3 py-2 bg-gray-700 text-gray-200 rounded-md"
            placeholder="Enter new category name"
          />
          <button
            @click="createNewCategory"
            class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
          >
            Create
          </button>
        </div>
      </div>

      <div>
        <input
          type="file"
          @change="handleFileUpload"
          :accept="acceptTypes"
          class="w-full text-gray-300 file:bg-gray-700 file:border-0 file:text-gray-300 file:px-4 file:py-2 file:rounded-md file:hover:bg-gray-600"
        />
        <div v-if="classificationLoading" class="mt-2 text-gray-300">
          Analyzing image...
        </div>
        <div v-if="suggestions.length > 0 && !selectedCategory" class="mt-2">
          <div class="flex justify-between items-center mb-2">
            <p class="text-gray-300">Suggested categories:</p>
          </div>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="prediction in predictions"
              :key="prediction.className"
              @click="handleSuggestionClick(prediction.className)"
              class="px-3 py-1 bg-gray-700 text-gray-200 rounded-md hover:bg-gray-600 text-sm flex items-center gap-2"
            >
              <span>{{ prediction.className }}</span>
              <span class="text-xs text-gray-400">({{ Math.round(prediction.probability * 100) }}%)</span>
            </button>
          </div>
        </div>
      </div>

      <button
        @click="handleUpload"
        :disabled="!file || !name || (!selectedCategory && !showNewCategory)"
        class="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:text-gray-400 transition-colors"
      >
        Upload {{ type }}
      </button>
    </div>
    <div v-if="error" class="text-red-400 mt-2">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useAuthStore } from '../utils/AuthStore';
import { listCategories, createCategory, type Category } from '../api';
import { classifyImage } from '../utils/imageClassifier';

const authStore = useAuthStore();

const props = defineProps<{
  type: 'Photo' | 'Video' | 'Model';
  acceptTypes: string;
  maxSize?: number;
}>();

const emit = defineEmits<{
  (e: 'upload', data: { file: File; name: string; categoryId: string }): void;
}>();

const file = ref<File | null>(null);
const error = ref<string | null>(null);
const name = ref('');
const categories = ref<Category[]>([]);

interface CategoryId {
  $oid: string;
}

const selectedCategory = ref<string | CategoryId>('');

const showNewCategory = ref(false);
const newCategoryName = ref('');

const classificationLoading = ref(false);
const suggestions = ref<string[]>([]);

interface Prediction {
  className: string;
  probability: number;
}

const predictions = ref<Prediction[]>([]);

onMounted(async () => {
  await fetchCategories();
});

async function fetchCategories() {
  try {
    categories.value = await listCategories();
  } catch (err) {
    error.value = 'Failed to load categories';
  }
}

async function createNewCategory() {
  if (!newCategoryName.value) return;
  
  try {
    const newCategory = await createCategory(newCategoryName.value);
    console.log('New category created:', newCategory);
    
    if (!newCategory._id?.$oid) {
      throw new Error('Invalid category ID received');
    }
    
    categories.value = [...categories.value, newCategory];
    selectedCategory.value = newCategory._id.$oid;
    showNewCategory.value = false;
    newCategoryName.value = '';
  } catch (err) {
    console.error('Failed to create category:', err);
    error.value = 'Failed to create new category';
  }
}

const classifyCurrentFile = async () => {
  if (!file.value || props.type !== 'Photo') return;

  try {
    classificationLoading.value = true;
    const results = await classifyImage(file.value);
    predictions.value = results;
    suggestions.value = results.map(p => p.className.toLowerCase());

    const matchingCategory = categories.value.find(cat => 
      suggestions.value.some(pred => cat.name.toLowerCase().includes(pred))
    );

    if (matchingCategory) {
      selectedCategory.value = matchingCategory._id.$oid;
    }
  } catch (err) {
    console.error('Classification error:', err);
  } finally {
    classificationLoading.value = false;
  }
};

const handleFileUpload = async (event: Event) => {
  const selectedFile = (event.target as HTMLInputElement).files?.[0];
  if (!selectedFile) return;

  if (props.maxSize && selectedFile.size > props.maxSize) {
    error.value = `File is too large. Maximum size is ${props.maxSize / (1024 * 1024)}MB`;
    return;
  }

  file.value = selectedFile;
  error.value = null;

  if (props.type === 'Photo') {
    await classifyCurrentFile();
  }
};

const handleSuggestionClick = (suggestion: string) => {
  const matchingCategory = categories.value.find(cat => 
    cat.name.toLowerCase().includes(suggestion.toLowerCase())
  );

  if (matchingCategory) {
    selectedCategory.value = matchingCategory._id.$oid;
    suggestions.value = [];
    predictions.value = [];
  } else {
    newCategoryName.value = suggestion.charAt(0).toUpperCase() + suggestion.slice(1);
    showNewCategory.value = true;
  }
};

watch(selectedCategory, (newValue) => {
  if (newValue) {
    suggestions.value = [];
    predictions.value = [];
  }
});

const handleUpload = async () => {
  error.value = null;

  if (!file.value || !name.value || !selectedCategory.value) {
    error.value = 'Please fill in all required fields';
    return;
  }

  try {
    const categoryId = typeof selectedCategory.value === 'object' && '$oid' in selectedCategory.value
      ? selectedCategory.value.$oid
      : selectedCategory.value;
    
    const uploadData = {
      file: file.value,
      name: name.value,
      categoryId
    };

    console.log('UploadForm sending:', uploadData);
    emit('upload', uploadData);

    file.value = null;
    name.value = '';
    selectedCategory.value = '';
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    if (input) input.value = '';
  } catch (err) {
    console.error('Upload error:', err);
    error.value = 'Upload failed';
  }
};
</script>
