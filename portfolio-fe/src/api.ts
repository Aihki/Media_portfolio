import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';

export function getFileUrl(filename: string) {
  return `${API_URL}/static/${filename}`;
}

export interface Category {
  _id: { $oid: string };
  name: string;
}

export async function listCategories(): Promise<Category[]> {
  const response = await axios.get(`${API_URL}/api/categories`);
  console.log('Categories from server:', response.data); 
  return response.data;
}

export async function createCategory(name: string): Promise<Category> {
  const response = await axios.post<Category>(`${API_URL}/api/categories`, {
    name,
  });
  console.log('Created category:', response.data); 
  return response.data;
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
  console.log('🔍 Fetching photos with details...');
  const response = await axios.get(`${API_URL}/api/photos/details`);
  console.log('📸 Received photos:', response.data);
  
  const photosWithFullUrls = response.data.map((photo: Photo) => ({
    ...photo,
    url: `${API_URL}${photo.url}`
  }));
  
  console.log('🖼️ Photos with full URLs:', photosWithFullUrls);
  return photosWithFullUrls;
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

  const buffer = await data.file.arrayBuffer();
  
  // Ensure 24-byte alignment (6 float32s per point)
  const bytesPerPoint = 24;
  const points = Math.floor(buffer.byteLength / bytesPerPoint);
  const alignedLength = points * bytesPerPoint;
  
  // Create aligned buffer
  const alignedBuffer = buffer.slice(0, alignedLength);
  
  const blob = new Blob([alignedBuffer], { 
    type: 'application/octet-stream'
  });

  const formData = new FormData();
  formData.append('file', new File([blob], data.file.name, {
    type: 'application/octet-stream'
  }));
  formData.append('name', data.name);
  formData.append('category', data.categoryId);

  const response = await axios.post(`${API_URL}/api/upload-model`, formData, {
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'multipart/form-data'
    },
    maxContentLength: 536870912, // 512MB
    maxBodyLength: 536870912,
    timeout: 300000 // 5 minutes
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
  const response = await axios.post(`${API_URL}/api/login`, {
    username,
    password,
  });

  if (response.data && response.data.token) {

    axios.defaults.headers.common['Authorization'] =
      `Bearer ${response.data.token}`;
    return response.data.token;
  }

  throw new Error('Login failed');
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
  const response = await axios.get(`${API_URL}/api/models/details`);
  
  return response.data.map((model: Model) => ({
    ...model,
    url: model.url.startsWith('http') ? model.url : `${API_URL}${model.url}`
  }));
}

export async function getVideosWithDetails(): Promise<Video[]> {
  console.log('🔍 Fetching videos with details...');
  const response = await axios.get(`${API_URL}/api/videos/details`);
  console.log('🎥 Received videos:', response.data);
  
  const videosWithFullUrls = response.data.map((video: Video) => ({
    ...video,
    url: `${API_URL}${video.url}`
  }));
  
  console.log('📽️ Videos with full URLs:', videosWithFullUrls);
  return videosWithFullUrls;
}

export type Stats = {
  photos_count: number;
  models_count: number;
  videos_count: number;
};

export async function getStats(): Promise<Stats> {
  const response = await axios.get(`${API_URL}/api/stats`);
  return response.data;
}

export async function deletePhoto(id: string): Promise<void> {
  await axios.delete(`${API_URL}/api/photos/${id}`);
}

export async function deleteModel(id: string): Promise<void> {
  await axios.delete(`${API_URL}/api/models/${id}`);
}

export async function deleteVideo(id: string): Promise<void> {
  await axios.delete(`${API_URL}/api/videos/${id}`);
}
