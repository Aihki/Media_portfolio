<template>
  <div class="max-w-4xl mx-auto px-4">
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-gray-100 mb-4">Pixel Canvas</h2>
      <p class="text-gray-400 mb-6">Click squares to draw</p>
    </div>

    <div class="bg-gray-800 p-4 sm:p-6 rounded-lg shadow-lg">
      <div 
        class="canvas-container bg-white rounded-lg overflow-hidden mx-auto"
        :style="{ width: `${CANVAS_SIZE}px`, height: `${CANVAS_SIZE}px` }"
      >
        <canvas ref="canvas" id="grid-canvas"></canvas>
      </div>

      <div class="mt-4 flex flex-wrap items-center justify-center gap-2 sm:gap-4">
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
import { ref, onMounted, onBeforeUnmount, computed } from 'vue';
import type { Canvas, Rect, Line, IEvent } from 'fabric/fabric-impl';

declare const fabric: {
  Canvas: new (elementId: string, options?: any) => Canvas;
  Line: new (points: number[], options?: any) => Line;
  Rect: new (options?: any) => Rect;
};

type FabricSquare = Rect & {
  fill: string;
  left: number;
  top: number;
  width: number;
  height: number;
};

const MOBILE_BREAKPOINT = 768;
const getCanvasSize = () => {
  if (typeof window === 'undefined') return 500;
  const screenWidth = window.innerWidth;
  const padding = screenWidth < MOBILE_BREAKPOINT ? 32 : 40;
  return Math.min(screenWidth - padding, 500);
};

const CANVAS_SIZE = ref(getCanvasSize());

const GRID_SIZE = computed(() => 
  window.innerWidth < MOBILE_BREAKPOINT ? 32 : 64
);

const CELL_SIZE = computed(() => 
  CANVAS_SIZE.value / GRID_SIZE.value
);

const canvas = ref(null);
const fabricCanvas = ref<Canvas | null>(null);
const squares = ref<FabricSquare[]>([]);
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
    width: CANVAS_SIZE.value,
    height: CANVAS_SIZE.value,
    backgroundColor: 'white',
    selection: false
  });

  squares.value = [];

  for (let i = 0; i <= GRID_SIZE.value; i++) {
    fabricCanvas.value.add(new fabric.Line(
      [i * CELL_SIZE.value, 0, i * CELL_SIZE.value, CANVAS_SIZE.value],
      {
        stroke: '#666',
        strokeWidth: 1,
        selectable: false,
        evented: false
      }
    ));
    
    fabricCanvas.value.add(new fabric.Line(
      [0, i * CELL_SIZE.value, CANVAS_SIZE.value, i * CELL_SIZE.value],
      {
        stroke: '#666',
        strokeWidth: 1,
        selectable: false,
        evented: false
      }
    ));
  }

  for (let i = 0; i < GRID_SIZE.value; i++) {
    for (let j = 0; j < GRID_SIZE.value; j++) {
      const square = new fabric.Rect({
        left: j * CELL_SIZE.value,
        top: i * CELL_SIZE.value,
        width: CELL_SIZE.value - 1,
        height: CELL_SIZE.value - 1,
        fill: 'white',
        selectable: false,
        hoverCursor: 'pointer'
      });

      square.on('mousedown', () => {
        if (!fabricCanvas.value) return;
        square.set('fill', square.fill === 'white' ? currentColor.value : 'white');
        fabricCanvas.value.renderAll();
      });

      square.on('mouseover', (e) => {
        if (!fabricCanvas.value) return;
        if (e.button === 1 || (e as any).buttons === 1) { 
          square.set('fill', square.fill === 'white' ? currentColor.value : 'white');
          fabricCanvas.value.renderAll();
        }
      });

      fabricCanvas.value.add(square);
      squares.value.push(square as unknown as FabricSquare);
    }
  }

  fabricCanvas.value.on('touch:start', handleTouchStart);
  fabricCanvas.value.on('touch:move', handleTouchMove);
};

const handleTouchInteraction = (e: IEvent<Event>) => {
  if (!fabricCanvas.value) return;
  const event = e.e as TouchEvent;
  if (!event.touches.length) return;

  const touch = event.touches[0];
  const rect = fabricCanvas.value.getElement().getBoundingClientRect();
  const canvasPointer = {
    x: ((touch.clientX - rect.left) * fabricCanvas.value.getWidth()) / rect.width,
    y: ((touch.clientY - rect.top) * fabricCanvas.value.getHeight()) / rect.height
  };

  const square = squares.value.find(s => 
    canvasPointer.x >= s.left && 
    canvasPointer.x <= s.left + s.width &&
    canvasPointer.y >= s.top && 
    canvasPointer.y <= s.top + s.height
  );

  if (square) {
    square.set('fill', currentColor.value);
    fabricCanvas.value.renderAll();
  }
};

const handleTouchStart = handleTouchInteraction;
const handleTouchMove = handleTouchInteraction;

const clearCanvas = () => {
  squares.value.forEach((square: FabricSquare) => {
    square.set('fill', 'white');
  });
  fabricCanvas.value?.renderAll();
};

const downloadCanvas = () => {
  if (!fabricCanvas.value) return;
  

  const tempCanvas = document.createElement('canvas');
  tempCanvas.width = CANVAS_SIZE.value;
  tempCanvas.height = CANVAS_SIZE.value;
  const ctx = tempCanvas.getContext('2d')!;
  

  squares.value.forEach((square: FabricSquare) => {
    if (square.fill !== 'white') {
      ctx.fillStyle = square.fill as string;
      ctx.fillRect(square.left, square.top, square.width, square.height);
    }
  });
  

  const link = document.createElement('a');
  link.download = 'pixel-art.png';
  link.href = tempCanvas.toDataURL('image/png');
  link.click();
};

const handleResize = () => {
  CANVAS_SIZE.value = getCanvasSize();
  initCanvas();
};

onMounted(() => {
  initCanvas();
  window.addEventListener('resize', handleResize);
});

onBeforeUnmount(() => {
  fabricCanvas.value?.dispose();
  window.removeEventListener('resize', handleResize);
});
</script>

<style scoped>
.canvas-container {
  width: 100% !important;
  max-width: 500px;
  aspect-ratio: 1;
  margin: 0 auto;
  touch-action: none;
}

canvas {
  width: 100% !important;
  height: 100% !important;
  touch-action: none;
}

#grid-canvas {
  touch-action: none;
  image-rendering: pixelated;
  -webkit-tap-highlight-color: transparent;
}

@media (max-width: 768px) {
  .max-w-4xl {
    padding: 0.5rem;
  }
  
  .bg-gray-800 {
    padding: 0.75rem;
  }
  
  .mt-4.flex {
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  button {
    padding: 0.75rem 1rem;
    font-size: 1rem;
    min-width: 80px;
    touch-action: manipulation;
  }
  
  input[type="color"] {
    width: 40px;
    height: 40px;
  }
}

html, body {
  overscroll-behavior-y: contain;
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