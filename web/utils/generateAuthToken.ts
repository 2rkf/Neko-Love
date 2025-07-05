export function generateAuthToken(): string {
    const charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let token = "";
    for (let i = 0; i < 40; i++) {
        token += charset.charAt(Math.floor(Math.random() * charset.length));
    }
    return `neko-love.${token}`;
}
