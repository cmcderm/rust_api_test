import axios from 'axios';

import config from '../config';
import type { User } from '../models/user';

const BASE_URL = config.API_BASE_URL;

export const Login = async (username: String, password: String): Promise<User | undefined> => {
    if (!username || !password) { return; }
    const response = await axios.post<User>(`${BASE_URL}/api/login`, {
        username,
        password,
    });

    return response.data;
};

export const Logout = () => {
    axios.post(`${BASE_URL}/api/logout`);
}

