<template>
  <div class="min-h-screen bg-zinc-800 text-zinc-100 noselect flex flex-col">
    <Navbar />

    <main class="flex-grow px-6 py-10 space-y-8">
      <div class="text-center">
        <h1 class="text-4xl font-extrabold text-white">Dashboard</h1>
        <p class="text-zinc-400 mt-2">Welcome back, here's your overview.</p>
      </div>

      <div
        class="max-w-xl mx-auto bg-zinc-900 p-6 rounded-xl border border-zinc-700"
      >
        <div class="mb-4">
          <h2 class="text-2xl font-bold text-orange-200">API Key</h2>
          <small>You may use this key to interact with Neko-Love API.</small>
        </div>

        <div class="space-y-4">
          <div class="flex gap-4">
            <UButton
              @click="generateToken"
              class="bg-orange-500 hover:bg-orange-600 text-white font-semibold cursor-pointer"
            >
              Generate & Save
            </UButton>
            <UModal>
              <UButton
                class="bg-zinc-700 hover:bg-zinc-600 text-white font-semibold cursor-pointer"
                label="View Key"
              />

              <template #content>
                <UCard>
                  <h3 class="text-lg font-semibold mb-4">Your API Key</h3>
                  <UInput
                    v-model="user.api_key"
                    readonly
                    class="w-full bg-zinc-800 text-sm"
                    placeholder="No API key yet"
                  />
                </UCard>
              </template>
            </UModal>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { generateAuthToken } from "~/utils/generateAuthToken";

const { token, user } = useAuth();
const toast = useToast();

const generateToken = async () => {
  const newToken = generateAuthToken();
  user.value.api_key = newToken;

  try {
    await useFetch(`/api/users/${user.value.username}`, {
      method: "PATCH",
      headers: {
        Authorization: `Bearer ${token.value}`,
      },
      body: { api_key: newToken },
    });

    toast.add({
      title: "Success",
      description: "Token generated and saved successfully.",
      color: "success",
    });
  } catch (err) {
    toast.add({
      title: "Error",
      description: "Failed to update token.",
      color: "error",
    });
  }
};

useHead({
  title: "Dashboard — Neko-Love",
  meta: [
    { name: "description", content: "View your dashboard on Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
