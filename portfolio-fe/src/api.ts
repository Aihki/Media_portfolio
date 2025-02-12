import axios from "axios";

const API_URL = "http://localhost:3000";

export function getFileUrl(filename: string) {
    return `${API_URL}/static/${filename}`;
}


export async function uploadPhoto(formData: FormData): Promise<void> {
    try {
        console.log('Uploading photo...');
        const response = await axios.post(`${API_URL}/api/upload-photo`, formData, {
            headers: { 
                'Content-Type': 'multipart/form-data',
                'Accept': 'application/json'
            },
            maxContentLength: Infinity,
            maxBodyLength: Infinity
        });

        console.log('Upload response:', response.data);
        if (!response.data) {
            throw new Error('Failed to upload photo');
        }
    } catch (error) {
        console.error('Upload error:', error);
        if (error instanceof Error) {
            throw new Error('Failed to upload photo: ' + (error.message || 'Unknown error'));
        } else {
            throw new Error('Failed to upload photo: Unknown error');
        }
    }
}

export async function listPhotos(): Promise<string[]> {
    console.log('Fetching photos from:', `${API_URL}/api/photos`);
    const response = await axios.get(`${API_URL}/api/photos`);
    console.log('Photos response:', response.data);
    return response.data.map((path: string) => `${API_URL}${path}`);
}