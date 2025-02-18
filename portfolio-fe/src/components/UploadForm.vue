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
            <option v-for="cat in categories" :key="cat._id" :value="cat._id">
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
import { ref, onMounted } from 'vue';
import { useAuthStore } from '../utils/AuthStore';
import { listCategories, createCategory } from '../api';

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
const categories = ref<Array<{ _id: string; name: string }>>([]);
const selectedCategory = ref('');
const showNewCategory = ref(false);
const newCategoryName = ref('');

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
    categories.value.push(newCategory);
    selectedCategory.value = newCategory._id!;
    showNewCategory.value = false;
    newCategoryName.value = '';
  } catch (err) {
    error.value = 'Failed to create new category';
  }
}

const handleFileUpload = (event: Event) => {
  const selectedFile = (event.target as HTMLInputElement).files?.[0];
  if (!selectedFile) return;

  if (props.maxSize && selectedFile.size > props.maxSize) {
    error.value = `File is too large. Maximum size is ${props.maxSize / (1024 * 1024)}MB`;
    return;
  }

  file.value = selectedFile;
  error.value = null;
};

const handleUpload = async () => {
  if (!file.value || !name.value) return;
  
  const categoryId = selectedCategory.value;
  if (!categoryId) {
    error.value = 'Please select or create a category';
    return;
  }

  emit('upload', {
    file: file.value,
    name: name.value,
    categoryId
  });

  // Reset form
  file.value = null;
  name.value = '';
  selectedCategory.value = '';
  const input = document.querySelector('input[type="file"]') as HTMLInputElement;
  if (input) input.value = '';
};
</script>
