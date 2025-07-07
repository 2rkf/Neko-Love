<template>
  <div class="min-h-screen bg-zinc-800 text-zinc-100 noselect flex flex-col">
    <Navbar />

    <main v-if="status === 'authenticated' && user" class="flex-grow px-6 py-10 space-y-8">
      <div class="text-center">
        <h1 class="text-4xl font-extrabold text-white">Dashboard</h1>
        <p class="text-zinc-400 mt-2">Welcome back, here's your overview.</p>
      </div>

      <div
        class="max-w-xl mx-auto bg-zinc-900 p-6 rounded-xl border border-zinc-700"
      >
        <div class="mb-4">
          <h2 class="text-2xl font-bold text-orange-200">API Key</h2>
          <small class="text-zinc-400"
            >You may use this key to interact with Neko-Love API.</small
          >
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

      <div class="text-center space-y-2">
        <h2 class="text-3xl font-extrabold text-white">Tiers</h2>
        <p class="text-zinc-400 text-sm">
          Your current plan:
          <span
            :class="
              user.gold
                ? 'text-yellow-300 font-bold'
                : 'text-orange-300 font-bold'
            "
          >
            {{ user.gold ? "Gold" : "Free" }}
          </span>
        </p>
      </div>

      <div class="max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
        <UCard
          :class="[
            'bg-zinc-900 border',
            user.gold
              ? 'border-zinc-700'
              : 'border-orange-400 shadow-lg shadow-orange-200/10',
          ]"
        >
          <div class="p-6 space-y-4 text-center">
            <h3
              :class="[
                'text-2xl font-bold',
                user.gold ? 'text-orange-200' : 'text-orange-300',
              ]"
            >
              Free Tier
            </h3>
            <p class="text-zinc-400 text-sm">
              Access a limited set of endpoints and rate limits. Ideal for
              casual use.
            </p>
            <ul class="text-left text-sm text-zinc-300 space-y-2 pt-4">
              <li>✅ Access to public endpoints</li>
              <li>✅ 1,000 requests per day</li>
              <li>❌ No access to NSFW content</li>
              <li>❌ No priority support</li>
            </ul>
            <div
              v-if="!user.gold"
              class="mt-4 flex items-center justify-center gap-2 text-orange-300 font-semibold text-sm"
            >
              You're currently a Free member!
            </div>
          </div>
        </UCard>

        <UCard
          :class="[
            'bg-zinc-900 border',
            user.gold
              ? 'border-yellow-400 shadow-lg shadow-yellow-200/10'
              : 'border-zinc-700',
          ]"
        >
          <div class="p-6 space-y-4 text-center">
            <h3
              :class="[
                'text-2xl font-bold',
                user.gold ? 'text-yellow-300' : 'text-yellow-200',
              ]"
            >
              Gold Tier
            </h3>

            <p class="text-zinc-400 text-sm">
              Unlock full features and generous limits. Perfect for power users.
            </p>

            <div class="text-3xl font-bold text-yellow-300 pt-2">
              $10
              <span class="text-sm font-medium text-yellow-200">one-time</span>
            </div>

            <ul class="text-left text-sm text-zinc-300 space-y-2 pt-4">
              <li>✅ Full access to all endpoints</li>
              <li>✅ 10,000 requests per day</li>
              <li>✅ NSFW endpoint access</li>
              <li>✅ Priority support & Discord role</li>
            </ul>

            <UButton
              v-if="!user.gold"
              class="mt-4 bg-yellow-400 text-black hover:bg-yellow-300 font-semibold"
              size="lg"
              label="Upgrade to Gold"
            />
            <span
              v-else
              class="mt-4 inline-block text-yellow-400 font-semibold text-sm"
            >
              You're already a Gold member ✨
            </span>
          </div>
        </UCard>
      </div>
    </main>
  </div>
</template>

<script setup>
import { generateAuthToken } from "~/utils/generateAuthToken";

const { status, token, user } = useAuth();
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
