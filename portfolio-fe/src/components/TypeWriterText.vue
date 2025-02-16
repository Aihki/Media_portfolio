<template>
  <div class="inline-block w-full">
    <p class="whitespace-normal break-words">{{ displayText }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

const props = defineProps<{
  text: string;
}>();

const displayText = ref('');

onMounted(() => {
  let currentIndex = 0;
  const interval = setInterval(() => {
    if (currentIndex <= props.text.length) {
      displayText.value = props.text.slice(0, currentIndex);
      currentIndex++;
    } else {
      clearInterval(interval);
    }
  }, 50);
</script>
