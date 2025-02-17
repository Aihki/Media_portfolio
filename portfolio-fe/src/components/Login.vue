<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <div class="max-w-md w-full bg-gray-800 rounded-lg shadow-lg p-8">
      <h2 class="text-3xl font-bold text-center text-white mb-8">Admin Login</h2>
      
      <form @submit.prevent="handleLogin" class="space-y-6">
        <div>
          <label class="block text-gray-300 text-sm font-bold mb-2">Username</label>
          <input 
            v-model="username" 
            type="text" 
            class="w-full px-3 py-2 bg-gray-700 text-white rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            required
          >
        </div>
        
        <div>
          <label class="block text-gray-300 text-sm font-bold mb-2">Password</label>
          <input 
            v-model="password" 
            type="password" 
            class="w-full px-3 py-2 bg-gray-700 text-white rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            required
          >
        </div>

        <div v-if="error" class="text-red-500 text-sm text-center">
          {{ error }}
        </div>
        
        <button 
          type="submit" 
          class="w-full bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Login
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { login } from '../api'

const router = useRouter()
const username = ref('')
const password = ref('')
const error = ref('')

async function handleLogin() {
  try {
    const token = await login(username.value, password.value)
    localStorage.setItem('auth_token', token)
    router.push('/admin')
  } catch (e) {
    error.value = 'Invalid username or password'
  }
}
</script>
