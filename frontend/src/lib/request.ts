import { goto } from "$app/navigation";
import type Result from "../models/result";

const url = import.meta.env.VITE_BACKEND_URL;

function formatEndpoint(path: string): string {
    return `${url}/${path}`;
}

async function request<T>(method: string, endpoint: string, dontRedirect?: boolean, body?: any): Promise<Result<T>> {
    const response = await fetch(formatEndpoint(endpoint), {
        method,
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${localStorage.getItem('token')}`
        },
        body: JSON.stringify(body)
    });

    if (response.status === 401) {
        const tokenResult = await refresh();

        if (tokenResult === null || tokenResult === undefined || tokenResult.error || !tokenResult.result) {
            if (!dontRedirect) {
                goto("/login");
            }
            console.log("Token expired");
            return {
                result: null,
                error: "Token expired",
            };
        }

        localStorage.setItem('token', tokenResult.result.access_token);
        return request(method, endpoint, dontRedirect, body);
    }

    return await response.json();
}

async function get<T>(path: string): Promise<Result<T>> {
    return request("GET", path);
}

async function post<T>(path: string, body: any): Promise<Result<T>> {
    return request("POST", path, false, body);
}

async function put<T>(path: string, body: any): Promise<Result<T>> {
    return request("PUT", path, false, body);
}

async function del<T>(path: string): Promise<Result<T>> {
    return request("DELETE", path, false);
}

async function refresh(): Promise<Result<{ access_token: string }> | null> {
    const response = await fetch(formatEndpoint("v1/auth/refresh"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    });

    if (response.status === 401) {
        return null;
    }

    return await response.json();
}

export { request, get, post, put, del };
