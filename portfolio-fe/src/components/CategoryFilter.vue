<template>
  <div class="mb-6 relative">
    <button
      type="button"
      @click="toggleMenu"
      class="flex items-center px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-md text-white"
    >
      <i class="pi pi-filter mr-2"></i>
      <span v-if="selectedCategories.length" class="ml-2 bg-blue-600 px-2 py-1 rounded-full text-sm">
        {{ selectedCategories.length }}
      </span>
    </button>

    <div v-if="showMenu" class="absolute top-full mt-2 bg-gray-800 border border-gray-700 rounded-lg p-2 z-50">
      <div v-for="category in categories" :key="category._id.$oid" class="flex items-center p-2">
        <input
          type="checkbox"
          :id="'category-' + category._id.$oid"
          :checked="selectedCategories.includes(category._id.$oid)"
          @change="(e) => handleCheckboxChange(category, e)"
          class="mr-2"
        >
        <label :for="'category-' + category._id.$oid" class="cursor-pointer text-white">
          {{ category.name }}
        </label>
      </div>
    </div>

    <div v-if="selectedCategories.length" class="mt-2 flex flex-wrap gap-2">
      <span
        v-for="categoryId in selectedCategories"
        :key="categoryId"
        class="inline-flex items-center bg-blue-600 text-white px-3 py-1 rounded-full"
      >
        {{ getCategoryName(categoryId) }}
        <button @click="removeCategory(categoryId)" class="ml-2" :aria-label="'Remove ' + getCategoryName(categoryId)">
          <i class="pi pi-times"></i>
        </button>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { listCategories } from '@/api';

interface Category {
  _id: { $oid: string };
  name: string;
}

const showMenu = ref(false);
const categories = ref<Category[]>([]);
const selectedCategories = ref<string[]>([]);

const emit = defineEmits(['filter']);

function toggleMenu() {
  showMenu.value = !showMenu.value;
}

function getCategoryName(categoryId: string) {
  return categories.value.find(c => c._id.$oid === categoryId)?.name || categoryId;
}

function removeCategory(categoryId: string) {
  selectedCategories.value = selectedCategories.value.filter(id => id !== categoryId);
}

function handleCheckboxChange(category: Category, event: Event) {
  const isChecked = (event.target as HTMLInputElement).checked;
  const categoryId = category._id.$oid;
  console.log('Category ID:', categoryId, 'Checked:', isChecked);
  
  if (isChecked) {
    selectedCategories.value = [...selectedCategories.value, categoryId];
  } else {
    selectedCategories.value = selectedCategories.value.filter(id => id !== categoryId);
  }
}

watch(selectedCategories, (newValue) => {
  console.log('Selected categories:', newValue);
  emit('filter', newValue.length > 0 ? newValue : null);
});

async function fetchCategories() {
  try {
    categories.value = await listCategories();
    console.log('Fetched categories:', categories.value);
  } catch (error) {
    console.error('Error fetching categories:', error);
  }
}

onMounted(fetchCategories);
</script>

<style scoped>
input[type="checkbox"] {
  accent-color: #2563EB;
}
</style>
