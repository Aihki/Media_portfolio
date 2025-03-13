import axios from 'axios';

// Make sure to set a default if VITE_API_URL is not defined
const API_URL = import.meta.env.VITE_API_URL || '';

console.log('Using API URL:', API_URL);

// Create an axios instance with CORS configuration
const apiClient = axios.create({
  baseURL: API_URL,
  withCredentials: false, // Important for CORS
  headers: {
    'Accept': 'application/json',
    'Content-Type': 'application/json'
  }
});

// Add error handlers to provide better feedback
apiClient.interceptors.response.use(
  response => response,
  error => {
    // Handle network errors more gracefully
    if (error.code === 'ERR_NETWORK' || error.message === 'Network Error') {
      console.warn('API connectivity issue:', {
        url: error.config?.url,
        method: error.config?.method,
        baseURL: error.config?.baseURL,
        message: 'Cannot connect to the API server. Check that the backend is running and CORS is properly configured.'
      });
    }
    
    // Log CORS issues specifically
    if (error.message && error.message.includes('CORS')) {
      console.error('CORS Error: The backend server is not configured to accept requests from this origin.');
      console.info('Solution: Ensure your backend has proper CORS headers enabled.');
      console.info('Server URL:', API_URL);
      console.info('Current origin:', window.location.origin);
    }
    
    return Promise.reject(error);
  }
);

// Helper function to safely make API calls with error handling
async function safeApiCall<T>(apiCall: () => Promise<T>, fallback: T): Promise<T> {
  try {
    return await apiCall();
  } catch (error) {
    console.error(`API call failed:`, error);
    return fallback;
  }
}

export function getFileUrl(filename: string) {
  return `${API_URL}/static/${filename}`;
}

export interface Category {
  _id: { $oid: string };
  name: string;
}

export async function listCategories(): Promise<Category[]> {
  return safeApiCall(async () => {
    const response = await apiClient.get('/api/categories');
    console.log('Categories from server:', response.data); 
    return response.data;
  }, []);
}

export async function createCategory(name: string): Promise<Category> {
  return safeApiCall(async () => {
    const response = await apiClient.post<Category>('/api/categories', { name });
    console.log('Created category:', response.data); 
    return response.data;
  }, { _id: { $oid: 'temp_id' }, name } as Category);
}

export type Photo = {
  id: string;
  name: string;
  filename: string;
  url: string;
  category_id: string;
  category_name: string; 
  created_at: string;
};

export async function getPhotosWithDetails(): Promise<Photo[]> {
  return safeApiCall(async () => {
    console.log('🔍 Fetching photos with details...');
    const response = await apiClient.get('/api/photos/details');
    console.log('📸 Received photos:', response.data);
    
    const photosWithFullUrls = response.data.map((photo: Photo) => ({
      ...photo,
      url: `${API_URL}${photo.url}`
    }));
    
    console.log('🖼️ Photos with full URLs:', photosWithFullUrls);
    return photosWithFullUrls;
  }, []);
}

export async function uploadPhoto(data: { 
  file: File; 
  name: string; 
  categoryId: string;
}): Promise<any> {
  if (!data?.file || !data?.name || !data?.categoryId) {
    throw new Error('Missing required upload data');
  }

  const formData = new FormData();
  formData.append('file', data.file);
  formData.append('name', data.name);
  formData.append('category', data.categoryId);

  console.log('Sending photo data:', {
    file: data.file.name,
    name: data.name,
    category: data.categoryId
  });

  try {
    const response = await axios.post(`${API_URL}/api/upload-photo`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
        Accept: 'application/json',
      },
      maxContentLength: Infinity,
      maxBodyLength: Infinity,
    });

    if (!response.data) {
      throw new Error('No response from server');
    }

    if (response.data.error) {
      throw new Error(response.data.error);
    }

    return response.data;
  } catch (error) {
    console.error('Error uploading photo:', error);
    throw error;
  }
}

export async function listPhotos(): Promise<string[]> {
  const response = await fetch(`${API_URL}/api/photos`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const paths = await response.json();

  return paths.map((path: string) => `${API_URL}${path}`);
}

export async function uploadModel(data: { 
  file: File; 
  name: string; 
  categoryId: string;
}): Promise<any> {
  if (!data?.file || !data?.name || !data?.categoryId) {
    throw new Error('Missing required upload data');
  }

  // Read file as ArrayBuffer and validate
  const buffer = await data.file.arrayBuffer();
  const bytesPerFloat = 4;
  const floatsPerPoint = 6;
  const bytesPerPoint = bytesPerFloat * floatsPerPoint;
  
  console.log('Upload file validation:', {
    totalSize: buffer.byteLength,
    expectedSize: Math.floor(buffer.byteLength / bytesPerPoint) * bytesPerPoint,
    remainder: buffer.byteLength % bytesPerPoint,
    points: Math.floor(buffer.byteLength / bytesPerPoint)
  });

  if (buffer.byteLength % bytesPerPoint !== 0) {
    throw new Error(`Invalid file size: ${buffer.byteLength} bytes. Must be divisible by ${bytesPerPoint}`);
  }

  // Create blob with proper type
  const blob = new Blob([buffer], { type: 'application/octet-stream' });
  const formData = new FormData();
  formData.append('file', new File([blob], data.file.name, { type: 'application/octet-stream' }));
  formData.append('name', data.name);
  formData.append('category', data.categoryId);

  // Upload with specific headers
  const response = await axios.post(`${API_URL}/api/upload-model`, formData, {
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'multipart/form-data'
    },
    maxContentLength: 536870912,
    maxBodyLength: 536870912,
    timeout: 300000
  });

  if (!response.data) {
    throw new Error('No response from server');
  }

  if (response.data.error) {
    throw new Error(response.data.error);
  }

  return response.data;
}

export async function listModels(): Promise<string[]> {
  try {
    const response = await axios.get(`${API_URL}/api/models`);
    // Ensure URLs have server origin
    return response.data.map((path: string) => {
      if (path.startsWith('http')) return path;
      return `${API_URL}${path}`;
    });
  } catch (error) {
    console.error('Error fetching models:', error);
    return [];
  }
}

export async function uploadVideo(data: {
  file: File;
  name: string;
  categoryId: string;
}): Promise<void> {
  if (!data?.file || !data?.name || !data?.categoryId) {
    console.error('Missing upload data:', data);
    throw new Error('Missing required upload data');
  }

  const formData = new FormData();
  formData.append('file', data.file);
  formData.append('name', data.name);
  formData.append('category', data.categoryId);

  const response = await axios.post(`${API_URL}/api/upload-video`, formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
      Accept: 'application/json',
    },
    maxContentLength: Infinity,
    maxBodyLength: Infinity,
  });

  if (!response.data) {
    throw new Error('No response from server');
  }

  if (response.data.error) {
    throw new Error(response.data.error);
  }

  return response.data;
}

async function retryRequest<T>(
  fn: () => Promise<T>,
  retries = 3,
  delay = 1000
): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    if (retries === 0) throw error;
    await new Promise(resolve => setTimeout(resolve, delay));
    return retryRequest(fn, retries - 1, delay * 2);
  }
}

export async function listVideos(): Promise<string[]> {
  try {
    const response = await retryRequest(() =>
      axios.get(`${API_URL}/api/videos`, {
        responseType: 'json',
        timeout: 5000,
        headers: {
          Accept: 'application/json',
          'Cache-Control': 'no-cache',
          Pragma: 'no-cache',
        },
        validateStatus: status => status < 500,
      })
    );

    if (!response.data) {
      console.error('Empty response from server');
      return [];
    }

    if (!Array.isArray(response.data)) {
      console.error('Unexpected response format:', response.data);
      return [];
    }

    return response.data
      .map((path: string) => {
        try {
          return new URL(
            path.startsWith('/') ? path.slice(1) : path,
            API_URL
          ).toString();
        } catch (error) {
          console.error('Error constructing URL for path:', path, error);
          return `${API_URL}/${path.startsWith('/') ? path.slice(1) : path}`;
        }
      })
      .filter(Boolean);
  } catch (error) {
    console.error('Error fetching videos:', error);
    if (axios.isAxiosError(error) && error.response) {
      console.error('Server response:', error.response.data);
    }
    return [];
  }
}

export async function login(
  username: string,
  password: string
): Promise<string> {
  try {
    // Use apiClient instead of axios directly for better error handling
    const response = await apiClient.post(`/api/login`, {
      username,
      password,
    });

    if (response.data && response.data.token) {
      const token = response.data.token;
      localStorage.setItem('auth_token', token);
      
      // Set token for all future requests
      apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      
      return token;
    }

    throw new Error('Login failed: Invalid response format');
  } catch (error) {
    console.error('Login error:', error);
    throw error;
  }
}

export function isLoggedIn(): boolean {
  return !!localStorage.getItem('auth_token');
}

export function logout(): void {
  localStorage.removeItem('auth_token');
  delete axios.defaults.headers.common['Authorization'];
}

export type Video = {
  id: string;
  name: string;
  filename: string;
  url: string;
  category_id: string;
  category_name: string;
  created_at: string;
};

export type Model = {
  id: string;
  name: string;
  filename: string;
  url: string;
  category_id: string;
  category_name: string;
  created_at: string;
};

export async function getModelsWithDetails(): Promise<Model[]> {
  return safeApiCall(async () => {
    const response = await apiClient.get('/api/models/details');
    
    return response.data.map((model: Model) => ({
      ...model,
      url: model.url.startsWith('http') ? model.url : `${API_URL}${model.url}`
    }));
  }, []);
}

export async function getVideosWithDetails(): Promise<Video[]> {
  return safeApiCall(async () => {
    console.log('🔍 Fetching videos with details...');
    const response = await apiClient.get('/api/videos/details');
    console.log('🎥 Received videos:', response.data);
    
    const videosWithFullUrls = response.data.map((video: Video) => ({
      ...video,
      url: `${API_URL}${video.url}`
    }));
    
    console.log('📽️ Videos with full URLs:', videosWithFullUrls);
    return videosWithFullUrls;
  }, []);
}

export type Stats = {
  photos_count: number;
  models_count: number;
  videos_count: number;
};

export async function getStats(): Promise<Stats> {
  return safeApiCall(async () => {
    const response = await apiClient.get('/api/stats');
    return response.data;
  }, { photos_count: 0, models_count: 0, videos_count: 0 });
}

export async function deletePhoto(id: string): Promise<void> {
  return safeApiCall(async () => {
    await apiClient.delete(`/api/photos/${id}`);
  }, undefined);
}

export async function deleteModel(id: string): Promise<void> {
  return safeApiCall(async () => {
    await apiClient.delete(`/api/models/${id}`);
  }, undefined);
}

export async function deleteVideo(id: string): Promise<void> {
  return safeApiCall(async () => {
    await apiClient.delete(`/api/videos/${id}`);
  }, undefined);
}