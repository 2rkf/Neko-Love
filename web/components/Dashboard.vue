<template>
  <div
    class="min-h-screen bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 font-[Nunito] noselect flex flex-col transition-all duration-500"
  >
    <Navbar />

    <main
      v-if="status === 'authenticated' && user"
      class="flex-grow px-6 py-10 space-y-10"
    >
      <div class="text-center space-y-2 animate-fade-in-up">
        <h1 class="text-4xl font-extrabold text-pink-600 drop-shadow-pink">
          Dashboard
        </h1>
        <p class="text-pink-700 text-sm">Welcome back, <b>{{ user.nickname }}</b>!</p>
      </div>

      <div
        class="max-w-xl mx-auto bg-white/80 p-6 rounded-lg shadow-lg backdrop-blur border border-pink-200 animate-fade-in-up"
      >
        <h2 class="text-2xl font-bold text-orange-400 mb-1">API Key</h2>
        <p class="text-pink-700 text-sm mb-4">
          Use this key to interact with Neko-Love API.
        </p>

        <div class="flex flex-col sm:flex-row gap-4">
          <UButton
            @click="generateToken"
            class="bg-orange-400 hover:bg-orange-300 text-white font-bold cursor-pointer transition"
          >
            Generate & Save
          </UButton>

          <UModal>
            <UButton
              class="bg-pink-200 hover:bg-pink-300 text-pink-900 font-semibold cursor-pointer transition"
              label="View Key"
            />
            <template #content>
              <UCard>
                <h3 class="text-lg font-bold text-pink-600 mb-2">
                  Your API Key
                </h3>
                <UInput
                  v-model="user.api_key"
                  readonly
                  class="w-full text-sm text-pink-700 px-3 py-2"
                  placeholder="No API key yet"
                />
              </UCard>
            </template>
          </UModal>
        </div>
      </div>

      <div class="text-center space-y-2 animate-fade-in-up">
        <h2 class="text-3xl font-extrabold text-pink-600">Tiers</h2>
        <p class="text-pink-700 text-sm">
          Current tier:
          <span
            :class="
              user.gold
                ? 'text-yellow-500 font-bold'
                : 'text-orange-400 font-bold'
            "
          >
            {{ user.gold ? "Gold" : "Free" }}
          </span>
        </p>
      </div>

      <div
        class="max-w-2xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8 mt-6 animate-fade-in-up"
      >
        <UCard
          :class="[
            'bg-white/80 backdrop-blur transition border-2',
            user.gold
              ? 'border-pink-100'
              : 'border-orange-300 shadow-lg shadow-orange-300/20',
          ]"
        >
          <div class="p-6 space-y-4 text-center">
            <h3 class="text-2xl font-bold text-orange-400">Free Tier</h3>
            <p class="text-pink-700 text-sm">
              Access a limited set of endpoints and rate limits. Ideal for
              casual use.
            </p>
            <div class="text-3xl font-bold text-orange-400">$0</div>
            <ul class="text-left text-sm text-pink-700 space-y-2 pt-4">
              <li>
                <UIcon name="i-lucide-check" class="text-orange-400" /> Public
                endpoint access
              </li>
              <li>
                <UIcon name="i-lucide-check" class="text-orange-400" /> 1,000
                requests per day
              </li>
              <li>
                <UIcon name="i-lucide-x" class="text-gray-400" /> No NSFW
                endpoint access
              </li>
              <li>
                <UIcon name="i-lucide-x" class="text-gray-400" /> No priority
                support
              </li>
            </ul>
            <div
              v-if="!user.gold"
              class="mt-4 text-orange-500 font-semibold text-sm"
            >
              You're on the Free tier!
            </div>
          </div>
        </UCard>

        <UCard
          :class="[
            'bg-white/80 backdrop-blur transition border-2',
            user.gold
              ? 'border-yellow-400 shadow-yellow-300/20 shadow-lg'
              : 'border-pink-100',
          ]"
        >
          <div class="p-6 space-y-4 text-center">
            <h3 class="text-2xl font-bold text-yellow-400">Gold Tier</h3>
            <p class="text-pink-700 text-sm">
              Unlock full features and generous limits. Perfect for power users.
            </p>
            <div class="text-3xl font-bold text-yellow-400">
              $10
              <span class="text-sm font-medium text-yellow-300">one-time</span>
            </div>
            <ul class="text-left text-sm text-pink-700 space-y-2 pt-4">
              <li>
                <UIcon name="i-lucide-check" class="text-yellow-400" /> All
                endpoint access
              </li>
              <li>
                <UIcon name="i-lucide-check" class="text-yellow-400" /> 10,000
                requests per day
              </li>
              <li>
                <UIcon name="i-lucide-check" class="text-yellow-400" /> NSFW
                endpoint access
              </li>
              <li>
                <UIcon name="i-lucide-check" class="text-yellow-400" /> Priority
                support + Discord role
              </li>
            </ul>
            <UButton
              v-if="!user.gold"
              class="mt-4 bg-yellow-300 text-black hover:bg-yellow-200 font-semibold cursor-pointer"
              size="lg"
              label="Upgrade to Gold"
            />
            <div
              v-else
              class="mt-4 text-yellow-500 font-semibold text-sm flex items-center justify-center gap-2"
            >
              <UIcon name="i-lucide-sparkles" class="w-4 h-4" />
              You're already a Gold member!
            </div>
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
      description: "Token generated and saved successfully.",
      color: "success",
    });
  } catch (err) {
    toast.add({
      description: "Failed to update token.",
      color: "error",
    });
  }
};

useHead({
  title: "Dashboard — Neko-Love",
  meta: [
    { name: "description", content: "View your dashboard on Neko-Love." },
    { name: "theme-color", content: "#ffd8b1" },
  ],
});
</script>
