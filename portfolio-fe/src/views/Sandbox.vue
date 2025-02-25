<template>
  <div class="flex items-center justify-center bg-gray-900">
    <div class="w-full bg-gray-800 rounded-lg shadow-lg p-8">
      <canvas ref="canvas" class="w-full h-screen"></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import * as BABYLON from '@babylonjs/core';
import '@babylonjs/loaders';

const canvas = ref<HTMLCanvasElement | null>(null);
const engine = ref<BABYLON.Engine | null>(null);
const scene = ref<BABYLON.Scene | null>(null);
const camera = ref<BABYLON.UniversalCamera | null>(null);

const createScene = (): void => {
    if (!canvas.value || !engine.value) return;
    
    scene.value = new BABYLON.Scene(engine.value);
    

    camera.value = new BABYLON.UniversalCamera(
        "FPCamera",
        new BABYLON.Vector3(0, 2, -10),
        scene.value
    );

    const light = new BABYLON.HemisphericLight(
        'light',
        new BABYLON.Vector3(0, 1, 0),
        scene.value
    );
    light.intensity = 1.0;

    if (camera.value && canvas.value) {
        camera.value.attachControl(canvas.value, true);
        camera.value.speed = 0.5;
        camera.value.angularSensibility = 1000;
        
        
        camera.value.keysUp.push(87);    
        camera.value.keysDown.push(83);  
        camera.value.keysLeft.push(65);  
        camera.value.keysRight.push(68); 
    }

   
    const ground = BABYLON.MeshBuilder.CreateGround(
        "ground",
        { width: 20, height: 20 },
        scene.value
    );
    

    const groundMaterial = new BABYLON.StandardMaterial("groundMaterial", scene.value);
    groundMaterial.diffuseColor = new BABYLON.Color3(0.2, 0.2, 0.2);
    groundMaterial.specularColor = new BABYLON.Color3(0.1, 0.1, 0.1);
    ground.material = groundMaterial;

    console.log('Attempting to load model...');
    
    BABYLON.SceneLoader.ImportMeshAsync(
        '',
        'http://localhost:3000/static/models/',
        'gundam1.splat',
        scene.value
    ).then(result => {
        console.log('Model loaded successfully:', result);
        if (result.meshes.length > 0) {
            const model = result.meshes[0];
            model.position = new BABYLON.Vector3(0, 2, 0);
            model.scaling = new BABYLON.Vector3(1, 1, 1);
            console.log('Model positioned and scaled');
        }
    }).catch(error => {
        console.error('Error loading model:', error);
        const errorText = new BABYLON.GUI.TextBlock();
        errorText.text = "Error loading model";
        errorText.color = "red";
        errorText.fontSize = 24;
    });
};

onMounted(() => {
    if (canvas.value) {
        engine.value = new BABYLON.Engine(canvas.value, true);
        createScene();
        
        window.addEventListener('resize', () => {
            engine.value?.resize();
        });
        
        engine.value.runRenderLoop(() => {
            scene.value?.render();
        });
    }
});

onUnmounted(() => {
    engine.value?.dispose();
});
</script>
