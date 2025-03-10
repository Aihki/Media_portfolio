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
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import * as handPoseDetection from '@tensorflow-models/hand-pose-detection';
import '@mediapipe/hands'; // Add this import

const videoRef = ref<HTMLVideoElement | null>(null);
const router = useRouter();
const error = ref<string>('');
const isLoading = ref(true);
let detector: handPoseDetection.HandDetector | null = null;
let animationFrameId: number | null = null;

const isActive = ref(false);
const sensitivity = ref(0.5);

const LOADING_TIMEOUT = 10000; // 10 seconds timeout
let loadingTimeout: NodeJS.Timeout | null = null;

const DEBUG = true; // Enable debug logging

const detectHands = async () => {
  if (!detector || !videoRef.value) return;

  try {
    const hands = await detector.estimateHands(videoRef.value);
    
    if (hands.length > 0) {
      const hand = hands[0];
      if (DEBUG) {
        // Only log index finger related points and handedness
        const indexFingerPoints = {
          handedness: hand.handedness,
          score: hand.score,
          keypoints: hand.keypoints
            .filter((_, index) => [0, 5, 6, 7, 8].includes(index))
            .map(k => ({
              x: Math.round(k.x),
              y: Math.round(k.y),
              name: k.name
            }))
        };
        console.log('Index finger tracking:', indexFingerPoints);
      }
      
      // Pass both keypoints and handedness
      handleGesture({
        keypoints: [
          hand.keypoints[0],  // wrist
          hand.keypoints[5],  // index finger MCP
          hand.keypoints[8]   // index finger tip
        ],
        handedness: hand.handedness
      });
    }

    animationFrameId = requestAnimationFrame(detectHands);
  } catch (e) {
    console.error('Hand detection error:', e);
  }
};

// Remove isOnLeftSide and isOnRightSide functions as we'll use handedness instead

// Update handle gesture to use handedness
const handleGesture = (handData: { keypoints: any[], handedness: string }) => {
  const now = Date.now();
  if (now - lastNavigationTime.value < NAVIGATION_COOLDOWN) {
    return;
  }

  currentGesture.value = 'No gesture detected';
  gestureConfidence.value = 0;

  // Navigate based on which hand is detected
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

const currentNavItem = computed(() => {
  const current = navigationLinks.find(link => link.path === router.currentRoute.value.path);
  return current?.name || 'Home';
});

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

const lastNavigationTime = ref(Date.now());
const NAVIGATION_COOLDOWN = 1500; // Increased cooldown
const currentGesture = ref('No gesture detected');
const gestureConfidence = ref(0);

const GESTURE_THRESHOLD = {
  SCREEN_DIVIDE: 0.5, // Screen midpoint (50%)
  MIN_CONFIDENCE: 0.7
};

// Remove or comment out unused gesture functions
// isPointingLeft, isPointingRight, calculateAngle, etc.

const isPointingUp = ref(false);
const isTwoFingersUp = ref(false);
const selectionCooldown = ref(false);

const isFingerPointingUp = (keypoints: any[]) => {
  if (keypoints.length !== 3) return false; // Ensure we have exactly 3 points
  
  const [wrist, indexBase, indexTip] = keypoints;
  const verticalDistance = wrist.y - indexTip.y;
  const angle = calculateAngle(wrist, indexBase, indexTip);
  
  return verticalDistance > 100 && 
         angle > 70 && 
         angle < 110;
};

const handleSelection = async () => {
  if (selectionCooldown.value) return;
  selectionCooldown.value = true;
  
  // Get current route and path parameters
  const currentRoute = router.currentRoute.value;
  const currentPath = currentRoute.path;

  // Handle different sections
  if (currentPath === '/') {
    // Handle photo selection
    router.push('/photos/1'); // Replace with actual photo ID
  } else if (currentPath === '/models') {
    router.push('/models/1'); // Replace with actual model ID
  } else if (currentPath === '/videos') {
    router.push('/videos/1'); // Replace with actual video ID
  }

  // Reset cooldown after 2 seconds
  setTimeout(() => {
    selectionCooldown.value = false;
    isPointingUp.value = false;
  }, 2000);
};

const isThreeFingerUp = ref(false);
const isThumbUp = ref(false);
const isVictorySign = ref(false);
const isOpenPalm = ref(false);

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
          modelType: 'full',  // Change to full model
          maxHands: 1,
          solutionPath: 'https://cdn.jsdelivr.net/npm/@mediapipe/hands',
          minHandDetectionConfidence: 0.7,
          minHandPresenceConfidence: 0.7,
          minTrackingConfidence: 0.7
        });

        detectHands();
      }
    } else {
      // Stop everything
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
  // Initialize with controls disabled
  isActive.value = false;
  isLoading.value = false;
});

onUnmounted(() => {
  if (loadingTimeout) {
    clearTimeout(loadingTimeout);
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




  