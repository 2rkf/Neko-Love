<template>
  <div class="min-h-screen flex flex-col bg-zinc-800 text-zinc-100 noselect">
    <Navbar />
    <main class="flex-grow flex items-center justify-center px-4 py-16">
      <UCard
        class="w-full max-w-md shadow-lg border border-zinc-700 bg-zinc-900"
        :ui="{ body: 'p-6s space-y-6 text-left' }"
      >
        <div class="text-center space-y-2">
          <h2 class="text-4xl font-extrabold text-white">Login</h2>
          <p class="text-zinc-400">Welcome back, please sign in to continue.</p>
        </div>

        <form class="space-y-4">
          <div class="space-y-2">
            <label class="text-lg font-medium text-orange-200">Username</label>
            <UInput
              v-model="username"
              size="lg"
              placeholder="Enter your username"
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
              placeholder="Enter your password"
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
              label="Login"
              class="w-full font-semibold cursor-pointer"
            />
            <UButton
              @click="toRegister"
              variant="outline"
              block
              size="lg"
              color="primary"
              label="Create an Account"
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
const password = ref("");
const toRegister = () => useRouter().push("/register");

useHead({
  title: "Login — Neko-Love",
  meta: [
    { name: "description", content: "Login to Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
