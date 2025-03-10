import { 
  collection, 
  addDoc, 
  getDocs, 
  query, 
  where, 
  serverTimestamp, 
  orderBy, 
  limit,
  type DocumentData,
  type QueryDocumentSnapshot,
  type DocumentReference,
} from 'firebase/firestore';
import { db } from '../utils/firebase';

export interface Feedback {
  contentId: string;
  contentType: 'video' | 'photo' | 'model';
  rating: number;
  comment: string;
  createdAt: Date;
}

export interface FeedbackDocument extends Feedback {
  id: string;
}

interface RawFeedback {
  id: string;
  contentId: string;
  contentType: 'video' | 'photo' | 'model';
  rating: number;
  comment: string;
  createdAt: Date;
}

export async function addFeedback(feedback: Omit<Feedback, 'createdAt'>) {
  try {
    const feedbackRef = collection(db, 'feedback');
    const docRef: DocumentReference = await addDoc(feedbackRef, {
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
    return snapshot.docs.map((doc: QueryDocumentSnapshot<DocumentData>) => ({
      id: doc.id,
      ...doc.data()
    } as FeedbackDocument));
  } catch (error) {
    console.error('Error getting feedback:', error);
    throw error;
  }
}

export async function getRecentFeedback(count = 5): Promise<RawFeedback[]> {
  try {
    const feedbackRef = collection(db, 'feedback');
    const q = query(
      feedbackRef,
      orderBy('createdAt', 'desc'),
      limit(count)
    );
    
    const snapshot = await getDocs(q);
    return snapshot.docs.map((doc: QueryDocumentSnapshot) => ({
      id: doc.id,
      contentId: doc.data().contentId,
      contentType: doc.data().contentType,
      rating: doc.data().rating,
      comment: doc.data().comment,
      createdAt: doc.data().createdAt?.toDate()
    }));
  } catch (error) {
    console.error('Error getting recent feedback:', error);
    throw error;
  }
}
