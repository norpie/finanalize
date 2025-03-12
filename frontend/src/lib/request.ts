import { goto } from "$app/navigation";
import type Result from "../models/result";

const url = import.meta.env.VITE_BACKEND_URL;

function formatEndpoint(path: string): string {
    return `${url.replace(/\/$/, '')}/${path.replace(/^\/+/, '')}`;
}

async function request<T>(method: string, endpoint: string, dontRedirect?: boolean, body?: any): Promise<Result<T>> {
    const response = await fetch(formatEndpoint(endpoint), {
        method,
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${localStorage.getItem('token')}`
        },
        body: body ? JSON.stringify(body) : undefined
    });

    if (response.status === 401) {
        const tokenResult = await refresh();

        if (!tokenResult?.result?.access_token) {
            if (!dontRedirect) {
                goto("/login");
            }
            console.log("Token expired");
            return { result: null, error: "Token expired" };
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

async function refresh(): Promise<Result<{ access_token: string }> | null> {
    const response = await fetch(formatEndpoint("v1/auth/refresh"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
    });

    return response.status === 401 ? null : await response.json();
}



/** Create a new wallet */
async function createWallet() {
    return post("v1/wallet/new", {});
}

/** Get wallet balance */
async function getWalletBalance(walletId: string) {
    return get(`v1/wallet/${walletId}/balance`); 
}

async function getWalletTransactions(walletId: string) {
    return get(`/v1/wallet/${walletId}/transactions`);

}


/** Add credits to a wallet */
async function addCredits(walletId: string, amount: number) {
    return post(`v1/wallet/${walletId}/add_credits`, { amount });
}

/** Use tokens on a report */
async function useTokens(walletId: string, reportId: string, tokens: number, apiType: string) {
    return post(`v1/${walletId}/use_tokens`, { report_id: reportId, tokens, api_type: apiType });
}

/** Generate wallet bill */
async function generateWalletBill(walletId: string) {
    return get(`v1/wallet/${walletId}/bill`);
}

/** Relate wallet to a user */
async function relateWalletToUser(walletId: string, userId: string) {
    return post(`v1/wallet/${walletId}/relate_user/${userId}`, {});
}

export { createWallet, getWalletBalance, addCredits, useTokens, generateWalletBill, relateWalletToUser, getWalletTransactions };
export { request, get, post };

