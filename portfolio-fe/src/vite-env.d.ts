/// <reference types="vite/client" />
/// <reference types="@firebase/firestore-types" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'firebase/app' {}
declare module 'firebase/firestore' {}