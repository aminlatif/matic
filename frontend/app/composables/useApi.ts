export const useApi = () => {
    const config = useRuntimeConfig();

    const baseURL = import.meta.server ? config.apiUrl : config.public.apiUrl;

    console.log("baseURL", baseURL);

    return $fetch.create({
        baseURL: baseURL
        // Add headers, interceptors, etc.
    });
};
