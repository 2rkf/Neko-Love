export type AuthStatus = "authenticated" | "unauthenticated" | "loading";

export interface AuthoriseValue {
    message: string;
};

export interface User {
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
