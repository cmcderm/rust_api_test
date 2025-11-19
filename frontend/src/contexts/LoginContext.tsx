import React, { useContext } from 'react';
import { createContext, useState } from 'react';

import { Login, Logout } from '../services/LoginService';
import type { User } from '../models/user';

interface LoginContextType {
    isLoggedIn: boolean;
    user: User | undefined;
    login: (username: string, password: string) => void;
    logout: () => void;
}

const LoginContext = createContext<LoginContextType>({
    isLoggedIn: false,
    user: undefined,
    login: (_u, _p) => {},
    logout: () => {},
});

export function LoginContextProvider({ children }: {children: React.ReactNode}) {
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const [user, setUser] = useState<User | undefined>();

    const login = async (username: string, password: string) => {
        const response = await Login(username, password);
        if (response === undefined) {
            console.log("Login failed");
        }
        setIsLoggedIn(true);
        setUser(response)
    }

    const logout = () => {
        Logout();
    }

    return (
        <LoginContext.Provider value={{ isLoggedIn, user, login, logout }}>
            {children}
        </LoginContext.Provider>
    )
}

export function useMyContext() {
    const ctx = useContext(LoginContext);
    if (!ctx) { throw new Error("useMyContext must be used within a LoginContextProvider"); }
    return ctx;
}
