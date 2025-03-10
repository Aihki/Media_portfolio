<template>
  <div class="hand-gesture-navigation">
    <button 
      @click="toggleNavigation" 
      class="fixed bottom-4 left-4 px-4 py-2 bg-gray-800/90 text-white rounded-lg backdrop-blur-sm border border-gray-700/50 hover:bg-gray-700/90 transition-colors"
    >
      {{ isActive ? 'Stop' : 'Start' }} Hand Navigation
    </button>

    <template v-if="isActive">
      <div v-if="!error && !isLoading" 
           class="fixed top-4 right-4 p-4 bg-gray-800/90 text-white rounded-lg text-sm max-w-[200px] backdrop-blur-sm border border-gray-700/50">
        <div class="space-y-2 text-xs text-gray-400">
          <div class="flex items-center gap-2">
          <p>use the gesture to move to the next or previous page</p>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-base">👈</span> Show Left Hand
            <span class="text-green-400 text-xs">(Previous)</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-base">👉</span> Show Right Hand
            <span class="text-green-400 text-xs">(Next)</span>
          </div>
        </div>
      </div>

      <video
        ref="videoRef"
        class="fixed bottom-0 right-0 w-48 h-36 z-50"
        autoplay
        playsinline
      ></video>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import * as handPoseDetection from '@tensorflow-models/hand-pose-detection';
import '@mediapipe/hands';

const videoRef = ref<HTMLVideoElement | null>(null);
const router = useRouter();
const error = ref<string>('');
const isLoading = ref(true);
let detector: handPoseDetection.HandDetector | null = null;
let animationFrameId: number | null = null;

const isActive = ref(false);
const lastNavigationTime = ref(Date.now());
const NAVIGATION_COOLDOWN = 1500;
const currentGesture = ref('No gesture detected');
const gestureConfidence = ref(0);

const detectHands = async () => {
  if (!detector || !videoRef.value) return;

  try {
    const hands = await detector.estimateHands(videoRef.value);
    
    if (hands.length > 0) {
      const hand = hands[0];
      handleGesture({
        keypoints: [
          hand.keypoints[0],
          hand.keypoints[5],
          hand.keypoints[8]
        ],
        handedness: hand.handedness
      });
    }

    animationFrameId = requestAnimationFrame(detectHands);
  } catch (e) {
    console.error('Hand detection error:', e);
  }
};

const handleGesture = (handData: { keypoints: any[], handedness: string }) => {
  const now = Date.now();
  if (now - lastNavigationTime.value < NAVIGATION_COOLDOWN) {
    return;
  }

  currentGesture.value = 'No gesture detected';
  gestureConfidence.value = 0;

  if (handData.handedness === 'Left') {
    updateGestureState('Left Hand');
    navigateToPrevious();
    lastNavigationTime.value = now;
  } else if (handData.handedness === 'Right') {
    updateGestureState('Right Hand');
    navigateToNext();
    lastNavigationTime.value = now;
  }
};

const navigationLinks = [
  { path: '/', name: 'Photos' },
  { path: '/models', name: '3D Models' },
  { path: '/videos', name: 'Videos' },
  { path: '/sandbox', name: 'Sandbox' },
  { path: '/pixel-art', name: 'Pixel Art' },
  { path: '/about', name: 'About Me' }
];

const navigateToNext = () => {
  const currentIndex = navigationLinks.findIndex(link => link.path === router.currentRoute.value.path);
  const nextIndex = (currentIndex + 1) % navigationLinks.length;
  router.push(navigationLinks[nextIndex].path);
};

const navigateToPrevious = () => {
  const currentIndex = navigationLinks.findIndex(link => link.path === router.currentRoute.value.path);
  const prevIndex = currentIndex <= 0 ? navigationLinks.length - 1 : currentIndex - 1;
  router.push(navigationLinks[prevIndex].path);
};

const updateGestureState = (gestureName: string, confidence: number = 1) => {
  if (confidence > gestureConfidence.value) {
    currentGesture.value = gestureName;
    gestureConfidence.value = confidence;
  }
};

const toggleNavigation = async () => {
  try {
    if (!isActive.value) {
      isActive.value = true;
      const stream = await navigator.mediaDevices.getUserMedia({
        video: {
          facingMode: 'user',
          width: { ideal: 640 },
          height: { ideal: 480 }
        }
      });
      
      if (videoRef.value) {
        videoRef.value.srcObject = stream;
        await new Promise((resolve) => {
          if (videoRef.value) videoRef.value.onloadedmetadata = resolve;
        });

        const model = handPoseDetection.SupportedModels.MediaPipeHands;
        detector = await handPoseDetection.createDetector(model, {
          runtime: 'mediapipe',
          modelType: 'full',
          maxHands: 1,
          solutionPath: 'https://cdn.jsdelivr.net/npm/@mediapipe/hands'
        });

        detectHands();
      }
    } else {
      isActive.value = false;
      if (videoRef.value?.srcObject) {
        const stream = videoRef.value.srcObject as MediaStream;
        stream.getTracks().forEach(track => track.stop());
        videoRef.value.srcObject = null;
      }
      if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
      }
      detector = null;
    }
  } catch (e) {
    console.error('Camera toggle failed:', e);
    isActive.value = false;
    if (videoRef.value?.srcObject) {
      const stream = videoRef.value.srcObject as MediaStream;
      stream.getTracks().forEach(track => track.stop());
      videoRef.value.srcObject = null;
    }
  }
};

onMounted(() => {
  isActive.value = false;
  isLoading.value = false;
});

onUnmounted(() => {
  if (videoRef.value?.srcObject) {
    const stream = videoRef.value.srcObject as MediaStream;
    stream.getTracks().forEach(track => track.stop());
  }
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
  }
});
</script>

<style scoped>
.transform {
  display: inline-block;
  transform-origin: center;
}
.-rotate-90 {
  transform: rotate(-90deg);
}
.rotate-90 {
  transform: rotate(90deg);
}
</style>




