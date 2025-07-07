export default defineEventHandler(async (event) => {
    const path = event.node.req.url || ""
    const config = useRuntimeConfig();

    if (path.startsWith("/api/_nuxt_icon")) {
        return
    }

    if (path.startsWith("/api/")) {
        return proxyRequest(event, `${config.API_URL}${path}`)
    }
});
