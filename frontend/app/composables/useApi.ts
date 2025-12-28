/* eslint-disable @typescript-eslint/no-explicit-any */
export const useApi = <T = any>() => {
    const config = useRuntimeConfig();

    const baseURL = import.meta.server ? config.apiUrl : config.public.apiUrl;

    const apiFetch = $fetch.create({
        baseURL: baseURL
    });

    // return $fetch.create({
    //     baseURL: baseURL
    //     // Add headers, interceptors, etc.
    // });

    return {
        get: <R = T>(url: string, opts?: any) =>
            apiFetch<R>(url, { method: "GET", ...opts }),

        post: <R = T>(url: string, body?: any, opts?: any) =>
            apiFetch<R>(url, { method: "POST", body, ...opts })

        // Add put, patch, delete as needed
    };
};
