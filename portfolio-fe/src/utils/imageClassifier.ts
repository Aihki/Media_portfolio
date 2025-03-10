import * as tf from '@tensorflow/tfjs';
import * as mobilenet from '@tensorflow-models/mobilenet';

interface Prediction {
  className: string;
  probability: number;
}

let model: mobilenet.MobileNet | null = null;

export async function initializeModel() {
  if (!model) {
    await tf.setBackend('webgl');
    await tf.ready();
    model = await mobilenet.load();
  }
  return model;
}

export async function classifyImage(file: File): Promise<Array<Prediction>> {
  try {
    if (!model) {
      await initializeModel();
    }

    const imageElement = await createImageElement(file);
    const predictions: Prediction[][] = [];
    

    const preprocessingConfigs = [
      { contrast: 1.2, brightness: 1.1, saturation: 1.1 },
      { contrast: 1.0, brightness: 1.0, saturation: 1.0 },
      { contrast: 0.9, brightness: 0.95, saturation: 0.9 }
    ];

    try {
      await Promise.all(preprocessingConfigs.map(async (config) => {
        const processedImage = await applyImageProcessing(imageElement, config);
        const batch = await model!.classify(processedImage, 7); 
        predictions.push(batch);
      }));

      const combinedPredictions = combineAndAveragePredictions(predictions);
      
      return combinedPredictions
        .filter(p => p.probability > 0.18)
        .map(normalizeConfidence);
    } finally {
      cleanupImage(imageElement);
    }
  } catch (error) {
    console.error('Classification error:', error);
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    throw new Error(`Failed to classify image: ${errorMessage}`);
  }
}

function cleanClassName(className: string): string {
  return className
    .toLowerCase()
    .split(',')[0] 
    .replace(/_/g, ' ')
    .replace(/\b(image|picture|photo|jpg|jpeg|png)\b/g, '')
    .split(' ')
    .filter(word => word.length > 2) 
    .join(' ')
    .trim();
}

function createImageElement(file: File): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onerror = () => reject(new Error('Failed to load image'));
    img.onload = () => {
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d')!;
      
   
      const OPTIMAL_SIZE = 224;
      let { width, height } = calculateOptimalDimensions(img.width, img.height, OPTIMAL_SIZE);
      
      canvas.width = width;
      canvas.height = height;

      ctx.filter = 'sharpen(1)';
      ctx.drawImage(img, 0, 0, width, height);
      
      const enhancedImg = new Image();
      enhancedImg.src = canvas.toDataURL('image/jpeg', 0.95);
      enhancedImg.onload = () => resolve(enhancedImg);
    };
    img.src = URL.createObjectURL(file);
  });
}

function calculateOptimalDimensions(width: number, height: number, targetSize: number) {
  const aspectRatio = width / height;
  return aspectRatio > 1 
    ? { width: targetSize, height: targetSize / aspectRatio }
    : { width: targetSize * aspectRatio, height: targetSize };
}

async function applyImageProcessing(
  img: HTMLImageElement, 
  config: { contrast: number; brightness: number; saturation: number }
): Promise<HTMLImageElement> {
  const canvas = document.createElement('canvas');
  const ctx = canvas.getContext('2d')!;
  
  canvas.width = img.width;
  canvas.height = img.height;
  
  ctx.filter = `contrast(${config.contrast}) brightness(${config.brightness}) saturate(${config.saturation})`;
  ctx.drawImage(img, 0, 0);
  
  return new Promise((resolve) => {
    const processed = new Image();
    processed.onload = () => resolve(processed);
    processed.src = canvas.toDataURL('image/jpeg', 0.95);
  });
}

function cleanupImage(imageElement: HTMLImageElement) {
  URL.revokeObjectURL(imageElement.src);
}

function combineAndAveragePredictions(allPredictions: Prediction[][]): Prediction[] {
  const predictionMap = new Map<string, number[]>();
  
  allPredictions.forEach(predictions => {
    predictions.forEach(pred => {
      const className = pred.className;
      const probs = predictionMap.get(className) || [];
      probs.push(pred.probability);
      predictionMap.set(className, probs);
    });
  });

  return Array.from(predictionMap.entries())
    .map(([className, probabilities]) => {

      const sortedProbs = [...probabilities].sort((a, b) => b - a);
      const weightedSum = sortedProbs.reduce((sum, prob, i) => 
        sum + prob * (1 / (i + 1)), 0
      );
      const weightedAvg = weightedSum / sortedProbs.reduce((sum, _, i) => 
        sum + 1 / (i + 1), 0
      );
      
      return {
        className,
        probability: weightedAvg
      };
    })
    .sort((a, b) => b.probability - a.probability)
    .slice(0, 3);
}

function normalizeConfidence(prediction: Prediction): Prediction {

  const normalized = 1 / (1 + Math.exp(-prediction.probability * 2));
  return {
    className: cleanClassName(prediction.className),
    probability: Number(normalized.toFixed(3))
  };
}
