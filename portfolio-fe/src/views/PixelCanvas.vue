<template>
  <div class="max-w-4xl mx-auto">
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-gray-100 mb-4">Grid Canvas</h2>
      <p class="text-gray-400 mb-6">Click on squares to toggle them</p>
    </div>

    <div class="bg-gray-800 p-4 rounded-lg shadow-lg">
      <div class="relative bg-white rounded-lg overflow-hidden" style="height: 400px;">
        <canvas ref="canvas" id="grid-canvas"></canvas>
      </div>

      <div class="mt-4 flex flex-wrap items-center justify-center gap-4">
        <button @click="clearCanvas" class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-md">
          Clear Grid
        </button>
</div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';

const canvas = ref(null);
const fabricCanvas = ref(null);
const gridSize = ref(16);
const squares = ref([]);

const initCanvas = async () => {
  if (!window.fabric) {
    await new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = 'https://cdnjs.cloudflare.com/ajax/libs/fabric.js/5.3.1/fabric.min.js';
      script.onload = resolve;
      script.onerror = reject;
      document.head.appendChild(script);
    });
  }

  if (fabricCanvas.value) {
    fabricCanvas.value.dispose();
  }

  const container = canvas.value.parentElement;
  const size = Math.min(container.clientWidth, container.clientHeight);
  const squareSize = size / gridSize.value;

  fabricCanvas.value = new fabric.Canvas('grid-canvas', {
    width: size,
    height: size,
    backgroundColor: 'white',
    selection: false
  });

  squares.value = [];

  for (let i = 0; i < gridSize.value; i++) {
    for (let j = 0; j < gridSize.value; j++) {
      const square = new fabric.Rect({
        left: j * squareSize,
        top: i * squareSize,
        width: squareSize,
        height: squareSize,
        fill: 'white',
        stroke: '#ccc',
        strokeWidth: 1,
        selectable: false,
        hoverCursor: 'pointer'
      });

      square.on('mousedown', () => {
        square.set('fill', square.fill === 'white' ? 'black' : 'white');
        fabricCanvas.value.renderAll();
      });

      fabricCanvas.value.add(square);
      squares.value.push(square);
    }
  }
};

const clearCanvas = () => {
  squares.value.forEach(square => {
    square.set('fill', 'white');
  });
  fabricCanvas.value?.renderAll();
};

onMounted(() => {
  initCanvas();
  window.addEventListener('resize', initCanvas);
});

onUnmounted(() => {
  fabricCanvas.value?.dispose();
  window.removeEventListener('resize', initCanvas);
});
</script>

<style scoped>
canvas {
  width: 100%;
  height: 100%;
}
</style>
