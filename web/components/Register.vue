<template>
  <div class="min-h-screen flex flex-col bg-zinc-800 text-zinc-100 noselect">
    <Navbar />
    <main class="flex-grow flex items-center justify-center px-4 py-16">
      <UCard
        class="w-full max-w-md shadow-lg border border-zinc-700 bg-zinc-900"
        :ui="{ body: 'p-6 space-y-6 text-left' }"
      >
        <div class="text-center space-y-2">
          <h2 class="text-4xl font-extrabold text-white">Register</h2>
          <p class="text-zinc-400">Create your account to join Neko-Love.</p>
        </div>

        <form class="space-y-4" @submit.prevent="submitForm">
          <div class="space-y-2">
            <label class="text-lg font-medium text-orange-200">Username</label>
            <UInput
              v-model="username"
              size="lg"
              placeholder="Choose a username"
              color="primary"
              class="w-full"
              :ui="{
                base: 'bg-zinc-900 text-white placeholder-zinc-400',
                wrapper: 'focus-within:ring-2 focus-within:ring-orange-300',
              }"
            />
          </div>

          <div class="space-y-2">
            <label class="text-lg font-medium text-orange-200">Email</label>
            <UInput
              v-model="email"
              type="email"
              size="lg"
              placeholder="you@example.com"
              color="primary"
              class="w-full"
              :ui="{
                base: 'bg-zinc-900 text-white placeholder-zinc-400',
                wrapper: 'focus-within:ring-2 focus-within:ring-orange-300',
              }"
            />
          </div>

          <div class="space-y-2">
            <label class="text-lg font-medium text-orange-200">Password</label>
            <UInput
              v-model="password"
              type="password"
              size="lg"
              placeholder="Create a password"
              color="primary"
              class="w-full"
              :ui="{
                base: 'bg-zinc-900 text-white placeholder-zinc-400',
                wrapper: 'focus-within:ring-2 focus-within:ring-orange-300',
              }"
            />
          </div>

          <div class="space-y-2">
            <label class="text-lg font-medium text-orange-200">Confirm Password</label>
            <UInput
              v-model="retypePassword"
              type="password"
              size="lg"
              placeholder="Confirm your password"
              color="primary"
              class="w-full"
              :ui="{
                base: 'bg-zinc-900 text-white placeholder-zinc-400',
                wrapper: 'focus-within:ring-2 focus-within:ring-orange-300',
              }"
            />
          </div>

          <div class="pt-4 space-y-4">
            <UButton
              type="submit"
              block
              size="lg"
              color="primary"
              label="Create Account"
              class="w-full font-semibold cursor-pointer"
            />
            <UButton
              @click="toLogin"
              variant="outline"
              block
              size="lg"
              color="primary"
              label="Login"
              class="w-full font-semibold cursor-pointer"
            />
          </div>
        </form>
      </UCard>
    </main>
  </div>
</template>

<script setup>
const username = ref("");
const email = ref("");
const password = ref("");
const retypePassword = ref("");

const router = useRouter();

const toLogin = () => router.push("/login");
const submitForm = () => {
  console.log("Register with:", {
    username: username.value,
    email: email.value,
    password: password.value,
  });
};

useHead({
  title: "Register — Neko-Love",
  meta: [
    { name: "description", content: "Register for Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
