import axios from 'axios';

const API_URL = 'http://localhost:3000';

export function getFileUrl(filename: string) {
  return `${API_URL}/static/${filename}`;
}

export async function uploadPhoto(formData: FormData): Promise<any> {
  const response = await fetch(`${API_URL}/upload/photo`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return await response.json();
}

export async function listPhotos(): Promise<string[]> {
  const response = await fetch(`${API_URL}/api/photos`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const paths = await response.json();
  // Convert relative paths to full URLs
  return paths.map((path: string) => `${API_URL}${path}`);
}

export async function uploadModel(file: File) {
  const formData = new FormData();
  formData.append('model', file);

  const response = await axios.post(`${API_URL}/upload/model`, formData, {
    headers: { 'Content-Type': 'multipart/form-data' },
  });

  return `${API_URL}${response.data.url}`;
}

export async function listModels(): Promise<string[]> {
  try {
    console.log('Fetching models from:', `${API_URL}/api/models`);
    const response = await axios.get(`${API_URL}/api/models`);
    console.log('Models response:', response.data);
    return response.data.map((path: string) => `${API_URL}${path}`);
  } catch (error) {
    console.error('Error fetching models:', error);
    if (axios.isAxiosError(error)) {
      console.error('Response:', error.response?.data);
      console.error('Status:', error.response?.status);
    }
    return [];
  }
}

export async function uploadVideo(formData: FormData): Promise<void> {
  try {
    const response = await retryRequest(() =>
      axios.post(`${API_URL}/api/upload-video`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
          Accept: 'application/json',
        },
        maxContentLength: Infinity,
        maxBodyLength: Infinity,
        responseType: 'json',
        timeout: 60000,
        validateStatus: status => status < 500,
      })
    );

    if (!response.data) {
      throw new Error('No response from server');
    }

    if (response.data.error) {
      throw new Error(response.data.error);
    }
  } catch (error) {
    console.error('Upload error details:', error);
    if (axios.isAxiosError(error)) {
      if (error.response) {
        throw new Error(
          `Upload failed: ${error.response.data?.message || error.response.statusText}`
        );
      } else if (error.request) {
        throw new Error('Server not responding. Please try again.');
      }
    }
    throw new Error('Failed to upload video. Please try again.');
  }
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

export async function login(username: string, password: string): Promise<string> {
  const response = await axios.post(`${API_URL}/api/login`, {
    username,
    password
  });
  
  if (response.data && response.data.token) {
    // Set the token in axios defaults for future requests
    axios.defaults.headers.common['Authorization'] = `Bearer ${response.data.token}`;
    return response.data.token;
  }
  
  throw new Error('Login failed');
}

// Add this to check if user is logged in
export function isLoggedIn(): boolean {
  return !!localStorage.getItem('auth_token');
}

// Add this to handle logout
export function logout(): void {
  localStorage.removeItem('auth_token');
  delete axios.defaults.headers.common['Authorization'];
}
