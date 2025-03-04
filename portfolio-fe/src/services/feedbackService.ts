import { collection, addDoc, getDocs, query, where, serverTimestamp } from 'firebase/firestore';
import { db } from '../utils/firebase';

export interface Feedback {
  contentId: string;
  contentType: 'model' | 'photo' | 'video';
  rating: number;
  comment: string;
  createdAt: Date;
}

export async function addFeedback(feedback: Omit<Feedback, 'createdAt'>) {
  try {
    const feedbackRef = collection(db, 'feedback');
    const docRef = await addDoc(feedbackRef, {
      ...feedback,
      createdAt: serverTimestamp()
    });
    console.log('Feedback added with ID:', docRef.id);
    return docRef.id;
  } catch (error) {
    console.error('Error adding feedback:', error);
    throw error;
  }
}

export async function getFeedbackForContent(contentId: string, contentType: string) {
  try {
    const feedbackRef = collection(db, 'feedback');
    const q = query(
      feedbackRef, 
      where('contentId', '==', contentId),
      where('contentType', '==', contentType)
    );
    
    const snapshot = await getDocs(q);
    return snapshot.docs.map(doc => ({
      id: doc.id,
      ...doc.data()
    }));
  } catch (error) {
    console.error('Error getting feedback:', error);
    throw error;
  }
}
