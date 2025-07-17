type AuthStatus = "authenticated" | "unauthenticated" | "loading";

interface User {
    api_key: string;
    blacklisted: 0 | 1;
    created_at: Date;
    email: string;
    gold: 0 | 1;
    id: string;
    nickname: string;
    password: string;
    username: string;
}

export function useAuth() {
    const sessionToken = useCookie<string | null>("session_token");
    const token = ref<string | null>(sessionToken.value || null);
    const user = ref<User | null>(null);
    const status = ref<AuthStatus>("loading");
    const error = ref<Error | null>(null);

    const loadCachedUser = () => {
        if (import.meta.client) {
            const cachedUser = localStorage.getItem("auth_user");
            if (cachedUser && token.value) {
                user.value = JSON.parse(cachedUser);
                status.value = "authenticated";
                return true;
            }
        }
        return false;
    };

    const saveUserToCache = (userData: User) => {
        if (import.meta.client) {
            localStorage.setItem("auth_user", JSON.stringify(userData));
        }
    };

    const updateUser = (updatedFields: Partial<User>) => {
        if (user.value) {
            user.value = { ...user.value, ...updatedFields };
            saveUserToCache(user.value);
        }
    };

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
        if (import.meta.client) {
            localStorage.removeItem("auth_user");
        }
        await navigateTo("/");
    };

    const fetchUser = async (force = false) => {
        if (!token.value) {
            status.value = "unauthenticated";
            user.value = null;
            if (import.meta.client) {
                localStorage.removeItem("auth_user");
            }
            return;
        }

        if (!force && loadCachedUser()) {
            return;
        }

        try {
            status.value = "loading";
            const response = await $fetch("/api/me", {
                method: "GET",
                headers: {
                    Authorization: `Bearer ${token.value}`,
                    "Content-Type": "application/json",
                },
            });

            user.value = response as User;
            status.value = "authenticated";
            saveUserToCache(user.value);
        } catch (err) {
            sessionToken.value = null;
            token.value = null;
            user.value = null;
            error.value = err as Error;
            status.value = "unauthenticated";
            if (import.meta.client) {
                localStorage.removeItem("auth_user");
            }
            throw err;
        }
    };

    onMounted(async () => {
        if (!loadCachedUser()) {
            await fetchUser();
        }
    });

    watch(token, async (newToken, oldToken) => {
        if (newToken !== oldToken) {
            await fetchUser(true);
        }
    });

    return {
        login,
        logout,
        token,
        user,
        status,
        error,
        fetchUser,
        updateUser,
    };
}
