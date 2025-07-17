export default defineNuxtPlugin((nuxtApp) => {
    if (import.meta.client) {
        const { logout } = useAuth();
        let token: string | null = null;

        async function heartbeat() {
            const sessionToken = useCookie<string>("session_token");
            token = sessionToken.value as string;

            if (token) {
                try {
                    const payloadPart = (token).split(".")[1];
                    if (!payloadPart) {
                        throw new Error("Invalid token format");
                    }
                    const payload = JSON.parse(atob(payloadPart));
                    const exp = payload.exp * 1000;
                    const now = Date.now();

                    if (now >= exp) {
                        useToast().add({
                            title: "Session Warning",
                            duration: 0,
                            description: "Your session has expired. Please log in again."
                        });
                        setTimeout(() => {
                            logout();
                        }, 5000);
                    }
                } catch (e) {
                    console.error("Error parsing token:", e);
                }
            }
        }

        const interval = setInterval(heartbeat, 10000);

        nuxtApp.hook('app:beforeMount', () => {
            clearInterval(interval);
        });
    }
});
