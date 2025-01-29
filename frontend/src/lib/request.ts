import { goto } from "$app/navigation";
import type Result from "../models/result";

// TODO: Get the URL from the environment
const url = "http://localhost:8080/api";

function formatEndpoint(path: string): string {
    return `${url}/${path}`;
}

async function request<T>(method: string, endpoint: string, body?: any): Promise<Result<T>> {
    const response = await fetch(endpoint, {
        method,
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${localStorage.getItem("accessToken")}`
        },
        body: JSON.stringify(body)
    });

    if (response.status === 401) {
        const tokenResult = await refresh();

        if (tokenResult === null || tokenResult === undefined || tokenResult.error || !tokenResult.result) {
            goto("/login");
            return {
                error: "Token expired",
            };
        }

        localStorage.setItem("accessToken", tokenResult.result.access_token);
        return request(method, endpoint, body);
    }

    return await response.json();
}

async function get<T>(path: string): Promise<Result<T>> {
    return request("GET", formatEndpoint(path));
}

async function post<T>(path: string, body: any): Promise<Result<T>> {
    return request("POST", formatEndpoint(path), body);
}

async function put<T>(path: string, body: any): Promise<Result<T>> {
    return request("PUT", formatEndpoint(path), body);
}

async function del<T>(path: string): Promise<Result<T>> {
    return request("DELETE", formatEndpoint(path));
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

export { get, post, put, del };
