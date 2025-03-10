declare module 'firebase/app' {
  export interface FirebaseApp {}
  export function initializeApp(config: any): FirebaseApp;
}

declare module 'firebase/firestore' {
  import { FirebaseApp } from 'firebase/app';
  
  export interface DocumentData {
    [field: string]: any;
  }

  export interface DocumentReference<T = DocumentData> {
    id: string;
    path: string;
    parent: CollectionReference<T>;
  }

  export interface QueryDocumentSnapshot<T = DocumentData> {
    id: string;
    ref: DocumentReference<T>;
    data(): T;
  }

  export interface CollectionReference<T = DocumentData> {
    path: string;
  }

  export interface Firestore {}
  export interface CollectionReference {}
  export interface DocumentReference {}
  export interface Timestamp {
    toDate(): Date;
  }

  export function getFirestore(app: FirebaseApp): Firestore;
  export function collection(firestore: Firestore, path: string): CollectionReference;
  export function addDoc(reference: CollectionReference, data: DocumentData): Promise<DocumentReference>;
  export function getDocs(query: any): Promise<{ docs: QueryDocumentSnapshot[] }>;
  export function query(reference: CollectionReference, ...queryConstraints: any[]): any;
  export function where(field: string, opStr: string, value: any): any;
  export function orderBy(field: string, directionStr?: 'asc' | 'desc'): any;
  export function limit(limit: number): any;
  export function serverTimestamp(): any;
}
