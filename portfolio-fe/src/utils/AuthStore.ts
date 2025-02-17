import { defineStore } from 'pinia';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    isAuthenticated: false,
    token: null as string | null,
  }),
  actions: {
    setAuth(token: string) {
      this.isAuthenticated = true;
      this.token = token;
      localStorage.setItem('token', token);
    },
    logout() {
      this.isAuthenticated = false;
      this.token = null;
      localStorage.removeItem('token');
    },
    checkAuth() {
      const token = localStorage.getItem('token');
      if (token) {
        this.isAuthenticated = true;
        this.token = token;
      }
    },
  },
});