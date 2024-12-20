import React, { createContext, useContext, useState, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';

const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  const [identity, setIdentity] = useState(null);
  const [client, setClient] = useState(null);

  useEffect(() => {
    initAuth();
  }, []);

  const initAuth = async () => {
    const authClient = await AuthClient.create();
    setClient(authClient);

    if (await authClient.isAuthenticated()) {
      setIdentity(authClient.getIdentity());
    }
  };

  const login = async () => {
    if (client) {
      await client.login({
        identityProvider: process.env.II_URL || 'https://identity.ic0.app',
        onSuccess: () => {
          setIdentity(client.getIdentity());
        },
      });
    }
  };

  const logout = async () => {
    if (client) {
      await client.logout();
      setIdentity(null);
    }
  };

  return (
    <AuthContext.Provider value={{ identity, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}