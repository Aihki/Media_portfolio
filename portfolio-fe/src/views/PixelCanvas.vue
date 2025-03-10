<template>
  <div class="max-w-4xl mx-auto">
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-gray-100 mb-4">Pixel Canvas</h2>
      <p class="text-gray-400 mb-6">Click squares to draw</p>
    </div>

    <div class="bg-gray-800 p-4 rounded-lg shadow-lg">
      <div class="canvas-container bg-white rounded-lg overflow-hidden mx-auto">
        <canvas ref="canvas" id="grid-canvas"></canvas>
      </div>

      <div class="mt-4 flex flex-wrap items-center justify-center gap-4">
        <div class="flex items-center gap-2">
          <label class="text-white">Color:</label>
          <input 
            type="color" 
            v-model="currentColor"
            class="w-8 h-8 rounded cursor-pointer"
          />
        </div>
        <button @click="clearCanvas" class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-md text-white">
          Clear Canvas
        </button>
        <button @click="downloadCanvas" class="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-md text-white">
          Download PNG
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const CANVAS_SIZE = 500;  
const GRID_SIZE = 50;    
const CELL_SIZE = CANVAS_SIZE / GRID_SIZE; 

const canvas = ref(null);
const fabricCanvas = ref(null);
const squares = ref([]);
const currentColor = ref('#000000');

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

  fabricCanvas.value = new fabric.Canvas('grid-canvas', {
    width: CANVAS_SIZE,
    height: CANVAS_SIZE,
    backgroundColor: 'white',
    selection: false
  });

  squares.value = [];


  for (let i = 0; i <= GRID_SIZE; i++) {
    fabricCanvas.value.add(new fabric.Line(
      [i * CELL_SIZE, 0, i * CELL_SIZE, CANVAS_SIZE],
      {
        stroke: '#666',
        strokeWidth: 1,
        selectable: false,
        evented: false
      }
    ));
    
    fabricCanvas.value.add(new fabric.Line(
      [0, i * CELL_SIZE, CANVAS_SIZE, i * CELL_SIZE],
      {
        stroke: '#666',
        strokeWidth: 1,
        selectable: false,
        evented: false
      }
    ));
  }


  for (let i = 0; i < GRID_SIZE; i++) {
    for (let j = 0; j < GRID_SIZE; j++) {
      const square = new fabric.Rect({
        left: j * CELL_SIZE,
        top: i * CELL_SIZE,
        width: CELL_SIZE - 1,
        height: CELL_SIZE - 1,
        fill: 'white',
        selectable: false,
        hoverCursor: 'pointer'
      });

      square.on('mousedown', () => {
        square.set('fill', square.fill === 'white' ? currentColor.value : 'white');
        fabricCanvas.value.renderAll();
      });

      square.on('mouseover', (e) => {
        if (e.buttons === 1) { 
          square.set('fill', square.fill === 'white' ? currentColor.value : 'white');
          fabricCanvas.value.renderAll();
        }
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

const downloadCanvas = () => {
  if (!fabricCanvas.value) return;
  

  const tempCanvas = document.createElement('canvas');
  tempCanvas.width = CANVAS_SIZE;
  tempCanvas.height = CANVAS_SIZE;
  const ctx = tempCanvas.getContext('2d')!;
  

  squares.value.forEach(square => {
    if (square.fill !== 'white') {
      ctx.fillStyle = square.fill;
      ctx.fillRect(square.left!, square.top!, square.width!, square.height!);
    }
  });
  

  const link = document.createElement('a');
  link.download = 'pixel-art.png';
  link.href = tempCanvas.toDataURL('image/png');
  link.click();
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
.canvas-container {
  width: 500px;  
  height: 500px; 
  margin: 0 auto;
}

canvas {
  width: 500px;   
  height: 500px;  
}

#grid-canvas {
  touch-action: none;
  image-rendering: pixelated;
}

input[type="color"] {
  -webkit-appearance: none;
  border: none;
  padding: 0;
}

input[type="color"]::-webkit-color-swatch-wrapper {
  padding: 0;
}

input[type="color"]::-webkit-color-swatch {
  border: none;
  border-radius: 4px;
}
</style>
