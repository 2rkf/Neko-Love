type AuthStatus = "authenticated" | "unauthenticated" | "loading";

interface User {
    created_at: Date;
    id: string;
    username: string;
    password: string;
    auth_token: string;
    nickname: string;
    email: string;
}

export function useAuth() {
    const config = useAppConfig();
    const sessionToken = useCookie<string | null>("session_token");
    const token = ref<string | null>(sessionToken.value || null);
    const user = ref<User | null>(null);
    const status = ref<AuthStatus>("loading");
    const error = ref<Error | null>(null);

    const login = async (loginToken: string) => {
        try {
            sessionToken.value = loginToken;
            token.value = loginToken;
            status.value = "loading";
            await fetchUser();
            await navigateTo("/dashboard");
        } catch (err) {
            error.value = err as Error;
            status.value = "unauthenticated";
            throw err;
        }
    };

    const logout = async () => {
        sessionToken.value = null;
        token.value = null;
        user.value = null;
        status.value = "unauthenticated";
        await navigateTo("/");
    };

    const fetchUser = async () => {
        if (!token.value) {
            status.value = "unauthenticated";
            return;
        }

        try {
            status.value = "loading";
            const { data, error: fetchError } = await useFetch<User>(`${config.API_URL}/api/me`, {
                headers: {
                    Authorization: `Bearer ${token.value}`,
                },
            });

            if (fetchError.value) {
                throw fetchError.value;
            }

            user.value = data.value;
            status.value = "authenticated";
        } catch (err) {
            sessionToken.value = null;
            token.value = null;
            user.value = null;
            error.value = err as Error;
            status.value = "unauthenticated";
            throw err;
        }
    };

    onMounted(async () => {
        await fetchUser();
    });

    return {
        login,
        logout,
        token,
        user,
        status,
        error,
        fetchUser
    };
}
