export default defineNuxtRouteMiddleware(async () => {
    let token: string | null = null;

    const sessionToken = useCookie<string>("session_token");
    token = sessionToken.value;

    if (!token) {
        return navigateTo("/login");
    }
});
